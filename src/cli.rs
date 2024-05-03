use std::fs::File;
use std::io::{BufRead, BufReader};

use clap::{Arg, Command};

use self::file::{
	desktop_file_add,
	desktop_file_del,
	desktop_file_set,
};

mod file {
	pub mod desktop_file_add;
	pub mod desktop_file_del;
	pub mod desktop_file_set;
}

enum Subcommand {
	Add,
	Del,
	Set,
	Unknown,
}

pub fn parse_arguments(args: Vec<String>) {
	if args.len() < 2 {
		print_help("help");
	}

	let cmd = Command::new("DesktopFile-Creator")

		// Global Settings
		.disable_help_flag(true)

		// Subcommands
		.subcommand(Command::new("add")
			.arg(Arg::new("base64")
				.short('b')
				.long("base64")
			)
			.arg(Arg::new("stdout")
				.short('c')
				.long("stdout")
			)
			.arg(Arg::new("quiet")
				.short('q')
				.long("quiet"))
		)

		.subcommand(Command::new("del")
			.arg(Arg::new("force")
				.short('f')
				.long("force")
			)
			.arg(Arg::new("quiet")
				.short('q')
				.long("quiet"))
			.arg(Arg::new("verbose")
				.short('n')
				.long("verbose"))
		)

		.subcommand(Command::new("set")
		            // TODO
		)

		// Global Options
		.arg(Arg::new("help")
			.short('h')
			.long("help")
			.global(true))

		.arg(Arg::new("version")
			.short('v')
			.long("version")
			.global(true))


		.get_matches();

	// Get Argument subcommands and parse to ENUM.
	let subcommand = match cmd.subcommand() {
		Some(("add", _sub_m)) => { Subcommand::Add }
		Some(("del", _sub_m)) => { Subcommand::Del }
		Some(("set", _sub_m)) => { Subcommand::Set }
		_ => { Subcommand::Unknown }
	};

	//
	match subcommand {
		Subcommand::Add => desktop_file_add::init(args),
		Subcommand::Del => desktop_file_del::init(args),
		Subcommand::Set => desktop_file_set::init(args),
		Subcommand::Unknown => print_help("help"),
	}
}

pub fn print_help(help_type: &str) {
	let file_path = format!("src/help/{}", help_type);
	if let Ok(file) = File::open(&file_path) {
		for line in BufReader::new(file).lines() {
			if let Ok(line_content) = line {
				println!("{}", line_content);
			}
		}
	}
}