use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};

fn main() {
	let path = Path::new("src");

	let files = fs::read_dir(path)
		.unwrap()
		.map(|entry| entry.unwrap().path())
		.filter(|path| path.is_file())
		.collect::<Vec<_>>();

	let target_dir = PathBuf::from(std::env::var("OUT_DIR").unwrap());

	fs::create_dir_all(&target_dir).unwrap();

	for file in files {
		let file_content = fs::read_to_string(&file).unwrap();
		let target_file = target_dir.join(file.file_name().unwrap());

		let mut target_file = fs::File::create(&target_file).unwrap();
		target_file.write_all(file_content.as_bytes()).unwrap();

		println!("cargo:rerun-if-changed={}", file.display());
		println!("cargo:rerun-if-changed={:?}", target_file);
	}

	fs::copy("target/debug/DesktopFile-Creator", "../build").unwrap();
}