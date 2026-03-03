use std::thread;
use std::process::{ Command, Child };
use std::path::Path;
use std::io;
use std::io::{ BufRead };
use std::process::Stdio as stdio;

use logger::Message;
use crate::logger;

pub async fn run_ip_script() -> io::Result<()> {
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

	let mut child: Child = Command::new("bash")
		.arg(script_path.display().to_string())
		.stdout(stdio::piped())
		.stderr(stdio::piped())
		.spawn()?;

	let stdout = child.stdout.take().expect("Failed to capture stdout");
	let stderr = child.stderr.take().expect("Failed to capture stderr");

	let stdout_handle = thread::spawn(move || {
		let reader = io::BufReader::new(stdout);
		for line in reader.lines().flatten() {
			logger(&Message {
				content: format!("Script output: {}", line),
				level: 101,
			});
		}
	});

	let stderr_handle = thread::spawn(move || {
		let reader = io::BufReader::new(stderr);
		for line in reader.lines().flatten() {
			logger(&Message {
				content: format!("Script error: {}", line),
				level: 400,
			});
		}
	});

	let status = child.wait()?;

	stdout_handle.join().expect("Failed to read stdout");
	stderr_handle.join().expect("Failed to read stderr");
	if status.success() {
		logger(&Message {
			content: String::from("Script executed successfully."),
			level: 200,
		});
	} else {
		logger(&Message {
			content: format!("Script exited with status: {}", status),
			level: 400,
		});
	}

	Ok(())
}
