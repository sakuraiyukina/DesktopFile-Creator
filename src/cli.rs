use std::collections::HashMap;

use clap::{Arg, Command};

use self::file::{
	desktop_file_add,
	desktop_file_del,
	desktop_file_get,
	desktop_file_set,
};

mod file {
	pub mod desktop_file_add;
	pub mod desktop_file_del;
	pub mod desktop_file_get;
	pub mod desktop_file_set;
}

pub enum Help {
	General,
	Add,
	Del,
	Get,
	Set,
}

impl Help {
	fn help_strings() -> HashMap<&'static str, &'static str> {
		let mut map = HashMap::new();
		map.insert("general", include_str!("help/help"));
		map.insert("subcommand-add", include_str!("help/subcommand-add"));
		map.insert("subcommand-del", include_str!("help/subcommand-del"));
		map.insert("subcommand-get", include_str!("help/subcommand-get"));
		map.insert("subcommand-set", include_str!("help/subcommand-set"));
		map
	}

	pub fn print_help(help_type: &str) {
		let help_strings: HashMap<&str, &str> = Self::help_strings();
		let help: &str = help_strings.get(help_type).unwrap();
		println!("{}", help);
	}
}

pub fn parse_arguments(args: Vec<String>) {
	let cmd = Command::new("DesktopFile-Creator")

		// Global Settings
		.version("0.1.0")
		.disable_help_flag(true)
		.disable_version_flag(true)

		// Subcommands
		.subcommand(Command::new("add")
			.arg(Arg::new("cmd.base64")
				.short('b')
				.long("base64")
			)

			.arg(Arg::new("cmd.add.stdout")
				.short('c')
				.long("stdout")
			)

			.arg(Arg::new("cmd.add.quiet")
				.short('q')
				.long("quiet")
			)

			.arg(Arg::new("cmd.add.help")
				.short('h')
				.long("help")
			)

			.arg(Arg::new("cmd.add.Type")
				.short('T')
				.long("type")
			)

			.arg(Arg::new("cmd.add.Version")
				.short('V')
				.long("version")
			)

			.arg(Arg::new("cmd.add.Name")
				.short('N')
				.long("name")
			)

			.arg(Arg::new("cmd.add.GenericName")
				.short('G')
				.long("genericname")
			)

			.arg(Arg::new("cmd.add.NoDisplay")
				.long("nodisplay")
			)

			.arg(Arg::new("cmd.add.Comment")
				.short('C')
				.long("comment")
			)

			.arg(Arg::new("cmd.add.Icon")
				.short('I')
				.long("icon")
			)

			.arg(Arg::new("cmd.add.Hidden")
				.short('H')
				.long("hidden")
			)

			.arg(Arg::new("cmd.add.OnlyShowIn")
				.long("onlyshowin")
			)

			.arg(Arg::new("cmd.add.NotShowIn")
				.long("notshowin")
			)

			.arg(Arg::new("cmd.add.DBusActivatable")
				.long("dbusactivatable")
			)

			.arg(Arg::new("cmd.add.TryExec")
				.short('R')
				.long("tryexec")
			)

			.arg(Arg::new("cmd.add.Exec")
				.short('E')
				.long("exec")
			)

			.arg(Arg::new("cmd.add.Path")
				.short('P')
				.long("path")
			)

			.arg(Arg::new("cmd.add.Terminal")
				.long("terminal")
			)

			.arg(Arg::new("cmd.add.Actions")
				.short('A')
				.long("actions")
			)

			.arg(Arg::new("cmd.add.MimeType")
				.short('M')
				.long("mimetype")
			)

			.arg(Arg::new("cmd.add.Categories")
				.short('W')
				.long("categories")
			)

			.arg(Arg::new("cmd.add.Keywords")
				.short('K')
				.long("keywords")
			)

			.arg(Arg::new("cmd.add.StartupNotify")
				.long("startupnotify")
			)

			.arg(Arg::new("cmd.add.StartupWMClass")
				.long("startupwmclass")
			)

			.arg(Arg::new("cmd.add.URL")
				.short('U')
				.long("url")
			)
		)

		.subcommand(Command::new("del")
			.arg(Arg::new("cmd.del.force")
				.short('f')
				.long("force")
			)

			.arg(Arg::new("cmd.del.quiet")
				.short('q')
				.long("quiet"))

			.arg(Arg::new("cmd.del.verbose")
				.short('n')
				.long("verbose"))
		)

		.subcommand(Command::new("get")
			// TODO;
		)

		.subcommand(Command::new("set")
		    // TODO;
		)

		// Global Options
		.arg(Arg::new("cmd.help")
			.short('h')
			.long("help")
			.num_args(0)
			.global(true))

		.arg(Arg::new("cmd.version")
			.short('v')
			.long("version")
			.num_args(0)
			.global(true))


		.get_matches();

	if cmd.get_flag("cmd.help") {
		Help::print_help("general");
		return;
	}
	else if cmd.get_flag("cmd.version") {
		println!("{}", env!("CARGO_PKG_VERSION"));
		return;
	}
	else {
		match cmd.subcommand() {
			Some(("add", _)) => desktop_file_add::init(args),
			Some(("del", _)) => desktop_file_del::init(args),
			Some(("get", _)) => desktop_file_get::init(args),
			Some(("set", _)) => desktop_file_set::init(args),
			_ => {},
		}
	}
}
