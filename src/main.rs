mod cli;

use std::env;
use crate::cli::Help;

fn main() {
	let args: Vec<String> = env::args().collect();
	if args.len() < 2 {
		Help::print_help("general");
		return;
	}
	cli::parse_arguments(args);
}
