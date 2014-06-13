use common::MetCommand;

use std::io::fs;
use std::io;

static DATABASE: &'static [u8] = include_bin!("empty.db");
static CONFIG: &'static str = include_str!("metaphor.toml");

pub struct Command;
impl Command {
	pub fn new() -> Box<MetCommand> { box Command as Box<MetCommand> }
}
impl MetCommand for Command {
	fn name(&self) -> &'static str { "init" }
	fn run(&self, args: &[String]) {
		let mut repo_root = Path::new(".");
		if args.len() == 1 {
			repo_root.push(args.head().unwrap().clone());
		} else if args.len() > 1 {
			error!("usage: met init [path]");
		}
		
		// Create .met directory
		let met_root = repo_root.join(".met");
		fs::mkdir(&met_root, io::UserDir);

		// Create staging area
		let stage_dir = met_root.join("stage");
		fs::mkdir(&stage_dir, io::UserDir);

		// Create reference "head" area
		let head_dir = met_root.join("head");
		fs::mkdir(&head_dir, io::UserDir);

		// Create default config file
		let config_path = met_root.join("metaphor.toml");
		let mut config_file = io::File::create(&config_path).unwrap();
		config_file.write_str(CONFIG);

		// Create database directory
		//let db_dir = met_root.join("databases");
		//fs::mkdir(&db_dir, io::UserDir);

		// Create empty databases
		let db_names = vec!("tag.db", "commit.db", "changeseq.db", "change.db", "patch.db",
			                  "forward.db", "backward.db");
		for &db_name in db_names.iter() {
			let db_path = met_root.join(db_name);
			let mut db_file = io::File::create(&db_path);
			db_file.write(DATABASE);
		}
	}
}