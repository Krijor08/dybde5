use std::process::{Command};
use std::path::Path;
use std::io;

pub fn run_script() -> io::Result<()> {
	let script_path = Path::new("./Scripts/asdf.sh");
	if !script_path.exists() {
		eprintln!("Script not found: {}", script_path.display());
		return Ok(());
	}

	let output = Command::new(script_path)
		.arg("arg1")
		.arg("arg2")
		.arg("arg3")
		.output()?;

	if output.status.success() {
		println!("Script output:\n{}", String::from_utf8_lossy(&output.stdout));
	} else {
		eprintln!("Script failed with error:\n{}", String::from_utf8_lossy(&output.stderr));
	}

	Ok(())
}