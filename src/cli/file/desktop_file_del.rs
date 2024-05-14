use crate::cli::Help;

pub fn init(args: Vec<String>) {
	if args.len() < 3 {
		Help::print_help("subcommand-del")
	} else {
		file_del();
	}
}

fn file_del() {
	// TODO
}