mod cli;

use std::env;
use crate::cli::print_help;

fn main() {
	let args: Vec<String> = env::args().collect();
	if args.len() < 2 {
		print_help("help");
		return;
	}
	cli::parse_arguments(args);
}
