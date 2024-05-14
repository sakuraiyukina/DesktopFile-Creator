use crate::cli::Help;

pub fn init(args: Vec<String>) {
	if args.len() < 3 {
		Help::print_help("subcommand-get");
	}
	else {
		// TODO;
	}
}