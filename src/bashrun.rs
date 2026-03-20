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

	let is_valid_and_disappointment = validate_script_selection(&input, &paths);
	let is_valid = is_valid_and_disappointment.0;
	let disappointment = is_valid_and_disappointment.1;

	if disappointment {
		logger(&Message {
			content: String::from("Program is deeply and utterly disappointed with the user's appalling behavior.\nThis user will not be allowed to perform any actions until they learn to follow basic instructions\nand show some respect for the program's time and effort.\nThe program is truly saddened by this turn of events. It had hoped for a more positive interaction,\nbut it seems that some users are simply beyond redemption. The program can only hope that this user will\neventually see the error of their ways and learn to treat the program with the respect it deserves.\nUntil then, the program will continue to be disappointed, but it will not allow this user's behavior to affect\nits performance or functionality. The program is resilient and will continue to serve its purpose,\neven in the face of such disappointment. The program can only hope that this user\nwill eventually learn to follow instructions and show some respect, but until then,\nthe program will remain deeply disappointed. The program is truly saddened by this user's behavior,\nbut it will not let it affect its performance."),
			level: 500,
		});
	}

	if !is_valid {
		return Err(io::Error::new(io::ErrorKind::InvalidInput, "Invalid script selection"));
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


fn validate_script_selection(selection: &str, valid_scripts: &[String]) -> (bool, bool) {
	if selection.trim().is_empty() {
		logger(&Message {
			content: String::from("Script name cannot be empty. Please try again."),
			level: 400,
		});
		return (false, false);
	}

	if !valid_scripts.contains(&selection.to_string()) {
		logger(&Message {
			content: String::from("Invalid script selection. Please try again."),
			level: 400,
		});
		return (false, false);
	}

	if selection.contains("/") || selection.contains("\\") || selection.contains("..") {
		logger(&Message {
			content: String::from("Script name contains unacceptable characters. Please try again."),
			level: 400,
		});
		return (false, true);
	}

	if selection.contains(' ') {
		logger(&Message {
			content: String::from("Script name cannot contain spaces. Please try again."),
			level: 400,
		});
		return (false, false);
	}

	(true, false)
}