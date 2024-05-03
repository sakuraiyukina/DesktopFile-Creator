use crate::cli::print_help;

pub fn init(args: Vec<String>) {
	if args.len() < 3 {
		print_help("subcommand-set");
	} else {
		file_set();
	}
}

fn file_set() {
	// TODO
}