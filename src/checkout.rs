use common::MetCommand;

use std::io::fs;
use std::io;
use std::os;

use metaphor;
use metaphor::repository::find_repo_root;
use metaphor::tree::tree_diff;
use metaphor::{Database, Tag, Patch};
use metaphor::diff::PatienceLinePatch;
use metaphor::{ToDisk, Object, ID, FlatTree, Tree, FromDisk};
use metaphor::Change;

use metaphor::DiskChangeSeq;

use common::copy_recursive;

pub struct Command;
impl Command {
	pub fn new() -> Box<MetCommand> { box Command as Box<MetCommand> }
}
impl MetCommand for Command {
	fn name(&self) -> &'static str { "checkout" }
	fn run(&self, args: &[String]) {
		let cd = os::make_absolute(&Path::new("."));
		let repo_root = match find_repo_root(&cd) {
			Some(path) => path,
			None => fail!("no repo found in this or any parent directory."),
		};
		debug!("repo root: {}", repo_root.display());
		let met_root = repo_root.join(".met");
		let head_dir = met_root.join("head");
		let stage_dir = met_root.join("stage");
		
		let commit_id = match args.len() {
			1 => {
				ID::from_hex(args.get(0).unwrap().as_slice())
			},
			_ => fail!("usage: met checkout [commit_id]"),
		};

		let mut database = metaphor::TrivialDb::new(&met_root);

		let mut commit_ids: Vec<ID> = database.commit_history(commit_id).unwrap();
		let mut changeseq_ids: Vec<ID> = commit_ids.iter().map(|cid| database.get_commit(cid.clone()).unwrap().changes).collect();
		let mut disk_changeseqs: Vec<DiskChangeSeq> = changeseq_ids.iter().map(|csid| database.get_changeseq(csid.clone()).unwrap()).collect();

		// Empty head
		fs::rmdir_recursive(&head_dir);
		fs::mkdir(&head_dir, io::UserDir);

		// Emtpy stage
		fs::rmdir_recursive(&stage_dir);
		fs::mkdir(&stage_dir, io::UserDir);

		let mut head_tree = metaphor::FsTree::new(&head_dir).unwrap();

		for disk_changeseq in disk_changeseqs.iter() {
			let changeseq = disk_changeseq.from_disk(&mut database).unwrap();
			changeseq.patch_tree(&mut head_tree);
		}

		// Copy head to stage
		copy_recursive(&head_dir, &stage_dir);

		// Clear working directory
		for entry in fs::readdir(&repo_root).unwrap().iter() {
			debug!("entry: {}", entry.display());
			if !met_root.is_ancestor_of(entry) {
				match entry.lstat().unwrap().kind {
					io::TypeFile => {
						fs::unlink(entry);
					},
					io::TypeDirectory => {
						fs::rmdir_recursive(entry);
					},
					_ => {
						debug!("skipping {}, not file or directory", entry.display());
					}
				}
			}
		}

		// Copy head to working dir
		copy_recursive(&head_dir, &repo_root);

		let head_tag = metaphor::tag::DiskTag{
			key: String::from_str("metaphor"),
			value: String::from_str("head"),
			targets: vec!(commit_id),
		};

		database.set_tag(head_tag);
	}
}