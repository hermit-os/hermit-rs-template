extern crate walkdir;

use std::env;
use std::path::Path;
use std::process::Command;
use walkdir::{DirEntry, WalkDir};

fn configure_cargo_rerun_if_changed(src_dir: &Path) {
	fn is_not_ignored(entry: &DirEntry) -> bool {
		// Ignore .git .vscode and target directories, but not .cargo or .github
		if entry.depth() == 1
			&& entry.path().is_dir()
			&& (entry.path().ends_with("target")
				|| entry.path().ends_with(".git")
				|| entry.path().ends_with(".vscode"))
		{
			return false;
		}
		true
	}

	WalkDir::new(src_dir)
		.into_iter()
		.filter_entry(|e| is_not_ignored(e))
		.filter_map(|v| v.ok())
		.filter_map(|v| v.path().canonicalize().ok())
		.for_each(|x| println!("cargo:rerun-if-changed={}", x.display()));
}

fn main() {
	let loader_dir = env::current_dir().unwrap().join("loader").join("src");

	configure_cargo_rerun_if_changed(loader_dir.as_ref());

	// create load by calling `make`
	let output = Command::new("make")
		.arg("release=1")
		.current_dir("loader")
		.output()
		.unwrap();

	let stdout = std::string::String::from_utf8(output.stdout);
	let stderr = std::string::String::from_utf8(output.stderr);

	println!("Build libhermit-rs output-status: {}", output.status);
	println!("Build libhermit-rs output-stdout: {}", stdout.unwrap());
	println!("Build libhermit-rs output-stderr: {}", stderr.unwrap());
	assert!(output.status.success());
}
