

/*
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
	fn name(&self) -> &'static str { "tag" }
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
			2 => {
				author = args.get(0).unwrap().as_slice();
				short = args.get(1).unwrap().as_slice();
			},
			_ => fail!("usage: met tag [key].[value] [id]"),
		}

		let mut database = metaphor::TrivialDb::new(&met_root);
	}
}
*/