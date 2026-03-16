use std::{ thread };
use std::process::{ Command, Child };
use std::path::Path;
use std::io;
use std::io::{ BufRead };
use std::process::Stdio as stdio;

use logger::Message;
use crate::{input, logger};

async fn ls() -> io::Result<Vec<String>> {
	let target_dir = Path::new("scripts");

	let output = Command::new("ls")
		.current_dir(target_dir)
		.output()?;

	if output.status.success() {
		let stdout = String::from_utf8_lossy(&output.stdout);
		let files: Vec<String> = stdout.lines().map(|line| line.to_string()).collect();
		logger(&Message {
			content: format!("Listed directory successfully: {:?}", files),
			level: 101,
		});
		Ok(files)
	} else {
		let stderr = String::from_utf8_lossy(&output.stderr);
		logger(&Message {
			content: format!("Failed to list directory: {}", stderr),
			level: 400,
		});
		Err(io::Error::new(io::ErrorKind::Other, "Failed to list directory"))
	}
}

pub async fn run_script() -> io::Result<()> {
	let paths = ls().await?;

	let input = input("Enter the name of the script to run (type 'h' for help):");

	if !paths.contains(&input.to_string()) {
		return Err(io::Error::new(io::ErrorKind::Other, "Invalid script selection"));
	}

	let script_path = Path::new("scripts").join(input);

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
				content: format!("{}", line),
				level: 101,
			});
		}
	});

	let stderr_handle = thread::spawn(move || {
		let reader = io::BufReader::new(stderr);
		for line in reader.lines().flatten() {
			logger(&Message {
				content: format!("[Script error]: {}", line),
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
