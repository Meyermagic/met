use common::MetCommand;

use std::io::fs;
use std::io;
use std::os;

use metaphor::repository::find_repo_root;
use metaphor::tree::{FsTree, tree_diff};

pub struct Command;
impl Command {
	pub fn new() -> Box<MetCommand> { box Command as Box<MetCommand> }
}
impl MetCommand for Command {
	fn name(&self) -> &'static str { "diff" }
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

		match args.len() {
			0 => (),
			_ => fail!("usage: met diff"),
		}

		let old_tree = FsTree::new(&head_dir).unwrap();
		let new_tree = FsTree::new(&stage_dir).unwrap();

		let changes = tree_diff(old_tree, new_tree);

	}
}