use std::io::BufRead;

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

pub enum Help {
	General,
	Add,
	Del,
	Set,
	Unknown,
}

impl Help {
	const GENERAL: &'static str = include_str!("help/help");
	const ADD: &'static str = include_str!("help/subcommand-add");
	const DEL: &'static str = include_str!("help/subcommand-del");
	const SET: &'static str = include_str!("help/subcommand-set");

	pub fn print_help(help_type: &str) {
		let help = match help_type {
			"general"        => Self::GENERAL,
			"subcommand-add" => Self::ADD,
			"subcommand-del" => Self::DEL,
			"subcommand-set" => Self::SET,
			_                => Self::GENERAL,
		};
		println!("{}", help);
	}
}

pub fn parse_arguments(args: Vec<String>) {
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
				.long("quiet")
			)
			.arg(Arg::new("help")
				.short('h')
				.long("help")
			)
			.arg(Arg::new("Type")
				.short('t')
				.long("type"))
			/*
			.arg(Arg::new("Version")
				.short('v')
				.long("version")
			)

			 */
			.arg(Arg::new("Name")
				.short('n')
				.long("name")
			)
			.arg(Arg::new("GenericName")
				.short('g')
				.long("genericname")
			)
			.arg(Arg::new("NoDisplay")
				.long("nodisplay")
			)
			.arg(Arg::new("Comment")
				.short('C')
				.long("comment")
			)
			.arg(Arg::new("Icon")
				.short('i')
				.long("icon")
			)
			.arg(Arg::new("Hidden")
				.long("hidden")
			)
			.arg(Arg::new("OnlyShowIn")
				.long("onlyshowin")
			)
			.arg(Arg::new("NotShowIn")
				.long("notshowin")
			)
			.arg(Arg::new("DBusActivatable")
				.long("dbusactivatable")
			)
			.arg(Arg::new("TryExec")
				.short('T')
				.long("typeexec")
			)
			.arg(Arg::new("Exec")
				.short('e')
				.long("exec")
			)
			.arg(Arg::new("Path")
				.short('p')
				.long("path")
			)
			.arg(Arg::new("Terminal")
				.long("terminal")
			)
			.arg(Arg::new("Actions")
				.short('a')
				.long("actions")
			)
			.arg(Arg::new("MimeType")
				.short('m')
				.long("mimetype")
			)
			.arg(Arg::new("Categories")
				.short('G')
				.long("categories")
			)
			.arg(Arg::new("Keywords")
				.short('k')
				.long("keywords")
			)
			.arg(Arg::new("StartupNotify")
				.long("startupnotify")
			)
			.arg(Arg::new("StartupWMClass")
				.long("startupwmclass")
			)
			.arg(Arg::new("URL")
				.short('u')
				.long("url")
			)
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
		            // TODO;
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
	match cmd.subcommand() {
		Some(("add", _)) => desktop_file_add::init(args),
		Some(("del", _)) => desktop_file_del::init(args),
		Some(("set", _)) => desktop_file_set::init(args),
		_ => { Help::print_help("general") }
	};
}
