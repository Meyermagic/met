

use common::MetCommand;

use std::io::fs;
use std::io;

use metaphor::tree;
use metaphor::diff::{Patch, PatienceLinePatch};

pub struct Command;
impl Command {
	pub fn new() -> Box<MetCommand> { box Command as Box<MetCommand> }
}
impl MetCommand for Command {
	fn name(&self) -> &'static str { "test_diff" }
	fn run(&self, args: &[String]) {
		let old_dir = Path::new(".").join(args.get(0).unwrap().as_slice());
		let new_dir = Path::new(".").join(args.get(1).unwrap().as_slice());

		let old_tree = tree::FsTree::new(&old_dir).unwrap();
		let new_tree = tree::FsTree::new(&new_dir).unwrap();
		println!("foo");
		let changes = tree::tree_diff(old_tree, new_tree);
		match changes {
			Err(e) => println!("err: {}", e),
			_ => {},
		}
		//println!("changes: {}", changes);
	}
}