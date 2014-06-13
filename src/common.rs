

/*
pub struct Argument {
	short: char,
	long: &'static str,
	short_desc: &'static str,
	long_desc: &'static str,
}

pub struct Command {
	name: &'static str,
	args: Vec<Argument>,
	subcommands: Vec<Command>,
}
*/

use std::io;
use std::io::fs;

pub trait MetCommand {
	fn name(&self) -> &'static str;
	fn run(&self, args: &[String]);
}


// Recursively copy the contents of a directory to another directory
pub fn copy_recursive(from: &Path, to: &Path) {
	for entry in fs::walk_dir(from).unwrap() {
		let path = entry.path_relative_from(from).unwrap();
		match entry.lstat().unwrap().kind {
			io::TypeFile => {
				fs::copy(&from.join(&path), &to.join(&path));
			},
			io::TypeDirectory => {
				fs::mkdir(&to.join(&path), io::UserDir);
			},
			_ => {
				debug!("skipping entry that is not File or Directory");
			}
		}
	}
}