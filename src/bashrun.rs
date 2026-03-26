use std::{ thread, process };
use std::process::{ Child, ChildStderr, ChildStdout, Command, Stdio as stdio };
use std::path::{Path, PathBuf};
use std::io::{ BufRead, BufReader, Error, ErrorKind, Result };

use logger::Message;
use crate::{input, logger};
use crate::help::script_help;

async fn ls() -> Result<Vec<String>> {
	let target_dir: &Path = Path::new("scripts");

	let output: process::Output = Command::new("ls")
		.current_dir(target_dir)
		.output()?;

	if output.status.success() {
		let stdout: std::borrow::Cow<'_, str> = String::from_utf8_lossy(&output.stdout);
		let files: Vec<String> = stdout
			.lines()
			.filter(|path| !path.starts_with("."))
			.map(|line| line
				.to_string())
				.collect();
			

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
		Err(Error::new(ErrorKind::Other, "Failed to list directory"))
	}
}


pub async fn run_script() -> Result<()> {
	let paths: Vec<String> = ls().await?;

	let input: String = input("Enter the name of the script to run (type 'h' for help):");

	let result: (bool, bool) = validate_script_selection(input.clone(), &paths);
	let is_valid: bool = result.0;
	let disappointment: bool = result.1;

	if !is_valid {
		return Err(Error::new(ErrorKind::InvalidInput, "Invalid script selection"));
	}

	if disappointment {
		logger(&Message {
			content: String::from("Program is deeply and utterly disappointed with the user's appalling behavior.\nThis user will not be allowed to perform any actions until they learn to follow basic instructions\nand show some respect for the program's time and effort.\nThe program is truly saddened by this turn of events. It had hoped for a more positive interaction,\nbut it seems that some users are simply beyond redemption. The program can only hope that this user will\neventually see the error of their ways and learn to treat the program with the respect it deserves.\nUntil then, the program will continue to be disappointed, but it will not allow this user's behavior to affect\nits performance or functionality. The program is resilient and will continue to serve its purpose,\neven in the face of such disappointment. The program can only hope that this user\nwill eventually learn to follow instructions and show some respect, but until then,\nthe program will remain deeply disappointed. The program is truly saddened by this user's behavior,\nbut it will not let it affect its performance."),
			level: 403,
		});
		return Err(Error::new(ErrorKind::PermissionDenied, "User behavior is unacceptable"));
	}

	let script_path: PathBuf = Path::new("scripts").join(input);

	let mut child: Child = Command::new("bash")
		.arg(script_path.display().to_string())
		.stdout(stdio::piped())
		.stderr(stdio::piped())
		.spawn()?;

	let stdout: ChildStdout = child.stdout.take().expect("Failed to capture stdout");
	let stderr: ChildStderr = child.stderr.take().expect("Failed to capture stderr");

	let stdout_handle: thread::JoinHandle<()> = thread::spawn(move || {
		let reader: BufReader<ChildStdout> = BufReader::new(stdout);
		for line in reader.lines().flatten() {
			logger(&Message {
				content: format!("{}", line),
				level: 101,
			});
		}
	});

	let stderr_handle: thread::JoinHandle<()> = thread::spawn(move || {
		let reader: BufReader<ChildStderr> = BufReader::new(stderr);
		for line in reader.lines().flatten() {
			logger(&Message {
				content: format!("[Script error]: {}", line),
				level: 400,
			});
		}
	});

	let status: process::ExitStatus = child.wait()?;

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


fn validate_script_selection(mut selection: String, valid_scripts: &[String]) -> (bool, bool) {
	const UNACCEPTABLE_CHARS: [&str; 11] = ["/", "\\", "..", "*", "?", "|", "\"", "<", ">", "\"",":"];

	if selection.trim().is_empty() {
		logger(&Message {
			content: String::from("Script name cannot be empty. Please try again."),
			level: 400,
		});
		return (false, false);
	}

	for &str in &UNACCEPTABLE_CHARS {
		if selection.contains(str) {
			logger(&Message {
				content: String::from("Script name contains unacceptable characters. Please try again."),
				level: 400,
			});
			return (false, true);
		}
	}

	if selection.contains(' ') {
		logger(&Message {
			content: String::from("Script name cannot contain spaces. Please try again."),
			level: 400,
		});
		return (false, false);
	}

	if selection == "h" || selection == "help" {
		script_help();

		selection = input("Enter the name of the script to run (type 'h' for help):");
		return validate_script_selection(selection, valid_scripts);
	}

	if !valid_scripts.contains(&selection.to_string()) {
		logger(&Message {
			content: String::from("Invalid script selection. Please try again."),
			level: 400,
		});
		return (false, false);
	}

	(true, false)
}