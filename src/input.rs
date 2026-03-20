use std::io::stdin;

use crate::{logger, Message};

pub fn input(prompt: &str) -> String {
	println!("{}", prompt);
	let mut input: String = String::new();
	stdin()
		.read_line(&mut input)
		.expect("Failed to read line. Type 'h' for help.");
	input.trim().to_string()
}


pub fn validate_input(input: &str) -> bool {
	if input.is_empty() {
			logger(&Message {
				content: String::from("Input cannot be empty. Please try again."),
				level: 400,
			});
			return false;
		}

		if input.contains(' ') {
			logger(&Message {
				content: String::from("Input cannot contain spaces. Please try again."),
				level: 400,
			});
			return false;
		}

		if input.len() < 3 {
			logger(&Message {
				content: String::from("Input must be at least 3 characters long. Please try again."),
				level: 400,
			});
			return false;
		}

	true
}
