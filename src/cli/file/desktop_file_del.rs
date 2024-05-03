use crate::cli::print_help;

pub fn init(args: Vec<String>) {
	if args.len() < 3 {
		print_help("subcommand-del")
	} else {
		file_del();
	}
}

fn file_del() {
	// TODO
}