




use common::MetCommand;

use std::io::fs;
use std::io;
use std::os;

use metaphor;
use metaphor::repository::find_repo_root;
use metaphor::tree::tree_diff;
use metaphor::{Database, Tag, Patch};
use metaphor::diff::PatienceLinePatch;
use metaphor::{ToDisk, Object, ID, FlatTree, Tree};

use common::copy_recursive;

pub struct Command;
impl Command {
	pub fn new() -> Box<MetCommand> { box Command as Box<MetCommand> }
}
impl MetCommand for Command {
	fn name(&self) -> &'static str { "commit" }
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

		let mut author = "";
		let mut short = "";
		let mut long = "";
		match args.len() {
			1 => {
				author = args.get(0).unwrap().as_slice();
			},
			2 => {
				author = args.get(0).unwrap().as_slice();
				short = args.get(1).unwrap().as_slice();
			},
			3 => {
				author = args.get(0).unwrap().as_slice();
				short = args.get(1).unwrap().as_slice();
				long = args.get(2).unwrap().as_slice();
			},
			_ => fail!("usage: met commit [author] [short summary] [long summary]"),
		}

		let mut database = metaphor::TrivialDb::new(&met_root);

		let head_tag = metaphor::tag::tag_id("metaphor", "head");
		// Check if we're on the first commit
		let parent_cid = match database.get_tag(head_tag) {
			Some(tag) => {
				tag.targets().get(0).unwrap().clone()
			},
			None => {
				debug!("no head in database");
				metaphor::EmptyID
			}
		};
		debug!("parent commit: {}", parent_cid);

		let old_tree = metaphor::FsTree::new(&head_dir).unwrap();
		let new_tree = metaphor::FsTree::new(&stage_dir).unwrap();

		let changes = tree_diff(old_tree, new_tree).unwrap();

		let commit = metaphor::MemCommit::new(author, short, long, parent_cid, changes);

		let (disk_commit, (changeseqs, changes, patches)) = commit.to_disk();

		let new_commit_id = disk_commit.id();
		debug!("new commit id: {}", new_commit_id);

		for patch in patches.move_iter() {
			debug!("storing patch with id {}", patch.id());
			database.set_patch(patch);
		}

		for change in changes.move_iter() {
			debug!("storing change with id {}", change.id());
			database.set_change(change);
		}

		for changeseq in changeseqs.move_iter() {
			debug!("storing changeseq with id {}", changeseq.id());
			database.set_changeseq(changeseq);
		}

		debug!("storing new commit");
		database.set_commit(disk_commit);

		debug!("removing metaphor.head tag from old commit");
		database.untag_object("metaphor", "head", parent_cid);
		debug!("adding metaphor.head tag to new commit");
		database.tag_object("metaphor", "head", new_commit_id);

		// Empty head
		fs::rmdir_recursive(&head_dir);
		fs::mkdir(&head_dir, io::UserDir);

		// Copy stage to head
		copy_recursive(&stage_dir, &head_dir);
	}
}





#![feature(phase)]

#[phase(plugin, link)]
extern crate log;
extern crate metaphor = "libmetaphor";
extern crate sync;

use sync::Arc;
use sync::raw::RwLock;

use std::io::fs;
use std::io;

use std::io::{TcpListener, TcpStream};
use std::io::{Acceptor, Listener};

use metaphor::repository::find_repo_root;
use metaphor::tree::tree_diff;
use metaphor::{Database, Tag, Patch, TrivialDb};
use metaphor::diff::PatienceLinePatch;
use metaphor::{ToDisk, Object, ID, FlatTree, Tree};

pub struct RepoToken;



fn handle_client(mut stream: TcpStream, db_lock: Arc<RwLock<TrivialDb>>) {
	debug!("got client");
	let call = stream.read_le_u64().unwrap();
	match call {
		1 => {
			// Clone
			let path_byte_length = stream.read_le_u64().unwrap();
			let path_bytes = stream.read_exact(path_byte_length).unwrap();

		},
		2 => {
			// Push
		},
		3 => {
			// Pull
		},
		_ => {
			error!("Didn't understand message");
		}
	}
}


fn main() {
	let mut repo_lock = Arc::new(RwLock::new(TrivialDb::new(&Path::new(".met"))));


	debug!("starting server");
	let listener = TcpListener::bind("127.0.0.1", 8080);
	let mut acceptor = listener.listen();

	for stream in acceptor.incoming() {
		match stream {
			Err(e) => { error!("failed to connect to client"); },
			Ok(stream) => spawn(proc() {
				handle_client(stream, repo_lock.clone());
			}),
		}
	}

	drop(acceptor);
}