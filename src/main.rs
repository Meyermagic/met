#![feature(phase)]

#[phase(plugin, link)]
extern crate log;
extern crate metaphor = "libmetaphor";

use std::os;
use common::MetCommand;
use std::collections::HashMap;

mod common;
mod branch;
mod clone;
mod commit;
mod destroy;
mod graft;
mod init;
mod merge;
mod splice;
mod stage;
mod tag;
mod track;
mod diff;
mod test_diff;
mod checkout;

struct MainCommand;
impl MetCommand for MainCommand {
	fn name(&self) -> &'static str { "met" }
	fn run(&self, args: &[String]) {
		let mut commands = HashMap::new();
		commands.insert(init::Command.name(), init::Command::new());
		commands.insert(test_diff::Command.name(), test_diff::Command::new());
		commands.insert(stage::Command.name(), stage::Command::new());
		commands.insert(diff::Command.name(), diff::Command::new());
		commands.insert(commit::Command.name(), commit::Command::new());
		commands.insert(checkout::Command.name(), checkout::Command::new());


		// TODO: Consume top-level options, then run this match on the rest
		match args {
			[ref subcommand_name, ..subcommand_args] => {
				let subcommand_name = subcommand_name.as_slice();
				match commands.find_equiv(&subcommand_name) {
					Some(ref subcommand) => { subcommand.run(subcommand_args); },
					None => { error!("met has no subcommand '{}'. available commands: {}", subcommand_name, commands.keys().collect::<Vec<&&'static str>>()); },
				}
			},
			_ => { error!("no subcommand specified"); }
		}
	}
}

fn main() {
	let _args = os::args();
	let args = _args.tail();
	println!("args: {}", args);

	let mut met = MainCommand;
	met.run(args.as_slice());
}
