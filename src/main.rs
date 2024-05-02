mod core;

use std::env;

fn main() {
	let args: Vec<String> = env::args().collect();
	core::parse_arguments(args);
}
