use common::MetCommand;

use std::io::fs;
use std::io;
use std::os;

use metaphor::repository::find_repo_root;


pub struct Command;
impl Command {
	pub fn new() -> Box<MetCommand> { box Command as Box<MetCommand> }
}
impl MetCommand for Command {
	fn name(&self) -> &'static str { "stage" }
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

		let stage_source = match args.len() {
			0 => repo_root.clone(),
			1 => cd.join(args.get(0).unwrap().as_slice()),
			_ => fail!("usage: met stage [path]"),
		};

		debug!("non-rel stage source: {}", stage_source.display());

		let stage_source = match stage_source.path_relative_from(&repo_root) {
			Some(path) => path,
			None => fail!("file or directory to stage must be inside repository root"),
		};

		debug!("rel stage source: {}", stage_source.display());

		let cd_suc = os::change_dir(&repo_root);
		debug!("cd: {}", cd_suc);

		let dirpart = Path::new(stage_source.dirname());
		debug!("dir part: {}", dirpart.display());

		if dirpart != Path::new(".") {
			debug!("path to stage not at repo root");
			fs::mkdir_recursive(&stage_dir.join(dirpart), io::UserDir);
		}

		if stage_source.is_file() {
			debug!("staging a file");
			let target = stage_dir.join(&stage_source);
			fs::copy(&stage_source, &target);
		} else if stage_source.is_dir() {
			debug!("staging a directory");
			let met_rel = Path::new("./.met");
			for path in fs::walk_dir(&stage_source).unwrap() {
				debug!("walking over {}", path.display());
				if met_rel.is_ancestor_of(&path) {
					debug!("skipping .met");
					continue;
				}
				//FIXME: Handle symlinks properly
				match path.lstat().unwrap().kind {
					io::TypeFile => {
						debug!("staging file {}", path.display());
						fs::copy(&path, &stage_dir.join(&path));
					},
					io::TypeDirectory => {
						debug!("staging dir {}", path.display());
						fs::mkdir(&stage_dir.join(&path), io::UserDir);
					},
					_ => {},
				}
			}
		}
	}
}