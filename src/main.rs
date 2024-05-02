mod cli;

use std::env;

fn main() {
	let args: Vec<String> = env::args().collect();
	cli::parse_arguments(args);
}
