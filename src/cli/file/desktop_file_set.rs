use crate::cli::Help;

pub fn init(args: Vec<String>) {
	if args.len() < 3 {
		Help::print_help("subcommand-set");
	} else {
		file_set();
	}
}

fn file_set() {
	// TODO
}