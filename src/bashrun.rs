use std::process::{Command};
use std::path::Path;
use std::io;

use logger::Message;
use crate::logger;

pub fn run_ip_script() -> io::Result<()> {
	let script_path = Path::new("scripts/ipscan");
	if !script_path.exists() {
		logger(&Message {
			content: format!("Script not found at path: {}", script_path.display()),
			level: 400,
		});
	} else {
		logger(&Message {
			content: format!("Running script at path: {}", script_path.display()),
			level: 102,
		});
	}

	let output = Command::new(format!("bash"))
		.arg(script_path.display().to_string())
		.output()?;

	if output.status.success() {
		logger(&Message {
			content: format!("Script output:\n{}", String::from_utf8_lossy(&output.stdout)),
			level: 200,
		});
	} else {
		logger(&Message {
			content: format!("Script failed with error:\n{}", String::from_utf8_lossy(&output.stderr)),
			level: 500,
		});
	}

	Ok(())
}