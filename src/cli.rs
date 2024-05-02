use std::fs::File;
use std::io::{BufRead, BufReader};

use clap::{Arg, Command};

mod file {
	pub mod desktop_file_add;
	pub mod desktop_file_del;
	pub mod desktop_file_set;
}


use self::file::{
	desktop_file_add,
	desktop_file_del,
	desktop_file_set
};

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
		.subcommand(Command::new("subcommand-add")
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

		.subcommand(Command::new("subcommand-del")
			.arg(Arg::new("force")
				.short('f')
				.long("force")
			)
			.arg(Arg::new("quiet")
				.short('q')
				.long("quiet"))
			.arg(Arg::new("verbose")
				.short('v')
				.long("verbose"))
		)

		.subcommand(Command::new("subcommand-set")
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
		Some(("subcommand-add", _sub_m)) => { Subcommand::Add }
		Some(("subcommand-del", _sub_m)) => { Subcommand::Set }
		Some(("subcommand-set", _sub_m)) => { Subcommand::Del }
		_ => { Subcommand::Unknown }
	};

	// 
	match subcommand {
		Subcommand::Add     => desktop_file_add::create("test"),
		Subcommand::Del     => desktop_file_del::del("test"),
		Subcommand::Set     => desktop_file_set::set("test"),
		Subcommand::Unknown => print_help("help"),
	}
}

fn print_help(help_type: &str) {
	let file_path = format!("src/help/{}", help_type);
	if let Ok(file) = File::open(&file_path) {
		for line in BufReader::new(file).lines() {
			if let Ok(line_content) = line {
				println!("{}", line_content);
			}
		}
	}
}