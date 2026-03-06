use std::thread;
use std::process::{ Command, Child };
use std::path::Path;
use std::io;
use std::io::{ BufRead, stdin };
use std::process::Stdio as stdio;

use logger::Message;
use crate::logger;

async fn ls() -> io::Result<()> {
	let target_dir = Path::new("scripts");

	let output = Command::new("ls")
		.arg("-l")
		.current_dir(target_dir)
		.output()?;

	if output.status.success() {
		let stdout = String::from_utf8_lossy(&output.stdout);
		println!("Directory contents:\n{}", stdout);
	} else {
		let stderr = String::from_utf8_lossy(&output.stderr);
		eprintln!("Error listing directory: {}", stderr);
	}

	Ok(())
}

pub async fn run_ip_script() -> io::Result<()> {
	let paths = ls().await?;

	println!("\nChoose script to run:");
		let mut input: String = String::new();
		stdin()
			.read_line(&mut input)
			.expect("Failed to read line");

		let input: &str = input.trim();

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
