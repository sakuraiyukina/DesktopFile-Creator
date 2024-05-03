use crate::cli::print_help;

pub fn init(args: Vec<String>) {
	if args.len() < 3 {
		print_help("subcommand-add");
	} else {
		file_create();
	}
}

fn file_create() {
	println!("Hello World");
}