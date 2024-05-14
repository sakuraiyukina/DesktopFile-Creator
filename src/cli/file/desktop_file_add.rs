use crate::cli::Help;

enum FileType {
	IsAppImage,
	IsCommand,
	IsExecutable,
	IsScript,
}

enum FileKey {
	Type            (String),
	Version         (String),
	Name            (String),
	GenericName     (String),
	NoDisplay       (bool),
	Comment         (String),
	Icon            (String),
	Hidden          (String),
	OnlyShowIn      (Vec<String>),
	NotShowIn       (Vec<String>),
	DBusActivatable (bool),
	TryExec         (String),
	Exec            (String),
	Path            (String),
	Terminal        (bool),
	Actions         (Vec<String>),
	MimeType        (Vec<String>),
	Categories      (Vec<String>),
	Keywords        (String),
	StartupNotify   (bool),
	StartupWMClass  (String),
	URL             (String),
}

pub fn init(args: Vec<String>) {
	if args.len() < 3 {
		Help::print_help("subcommand-add");
	} else {
		// file_create();
	}
}

fn file_create(args: Vec<String>) {
	// TODO;
}
