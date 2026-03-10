use std::io::stdin;
use serde::{Deserialize, Serialize};
use serde_json;

use crate::{logger};
use crate::help::login_help;

use crate::logger::Message;


#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct User {
 	pub(crate) username: String,
 	pub(crate) email: String,
	pub(crate) password: String,
 	pub(crate) access_level: u8,
}


#[derive(Debug, Deserialize, Serialize)]
pub struct Root {
    users: Vec<User>,
}


pub fn create_user(users: &[User], current_access_level: u8) -> Option<User> {
	println!("Enter new username:");
	let mut username: String = String::new();
	stdin()
		.read_line(&mut username)
		.expect("Failed to read line. Type 'h' for help.");

	let username: &str = username.trim();

	if username == "c" || username == "cancel" {
		logger(&Message {
			content: String::from("User creation cancelled. Returning to main menu."),
			level: 102,
		});
		return None;
	}

	if username == "h" || username == "help" {
		login_help();
		return None;
	}

	if !validate(username) {
		return None;
	}

	for user in users {
		if user.username == username {
			logger(&Message {
				content: String::from("Username already exists. Please try again."),
				level: 400,
			});
			return None;
		}
	};

	println!("Enter email:");
	let mut email: String = String::new();
	stdin()
		.read_line(&mut email)
		.expect("Failed to read line. Type 'h' for help.");

	let email: &str = email.trim();

	if !validate(email) {
		return None;
	}

	println!("Enter password:");
	let mut password: String = String::new();
	stdin()
		.read_line(&mut password)
		.expect("Failed to read line. Type 'h' for help.");

	let password: &str = password.trim();

	if !validate(password) {
		return None;
	}

	println!("Enter access level (0-255):");
	let mut access_level_str: String = String::new();
	stdin()
		.read_line(&mut access_level_str)
		.expect("Failed to read line. Type 'h' for help.");

	let access_level_str: &str = access_level_str.trim();
	let access_level: u8 = match access_level_str.parse() {
		Ok(num) => num,
		Err(_) => {
			logger(&Message {
				content: String::from("Invalid access level. Please enter a number between 0 and 255."),
				level: 400,
			});
			return None;
		}
	};

	if access_level > 255 || access_level < 0 {
		logger(&Message {
			content: String::from("Access level must be between 0 and 255."),
			level: 400,
		});
		return None;
	}

	if access_level > current_access_level {
		logger(&Message {
			content: String::from("Cannot create user with higher access level than your own."),
			level: 403,
		});
		return None;
	}

	let new_user = User {
		username: username.to_string(),
		email: email.to_string(),
		password: password.to_string(),
		access_level,
	};

	let mut all_users = users.to_vec();
	all_users.push(new_user.clone());

	match serde_json::to_string_pretty(&Root { users: all_users }) {
		Ok(json) => {
			if let Err(e) = std::fs::write("./users.json", json) {
				logger(&Message {
					content: format!("Failed to write to users.json: {}", e),
					level: 500,
				});
				return None;
			} else {
				return Some(new_user);
			}
		},
		Err(e) => {
			logger(&Message {
				content: format!("Failed to serialize users: {}", e),
				level: 500,
			});
			return None;
		}
	}
}


pub fn get_users() -> Result<Vec<User>, &'static str> {
	let content: String = std::fs::read_to_string("./users.json")
        .map_err(|_| "Could not read users.json")?;

	let root: Root = serde_json::from_str(&content)
		.map_err(|_| "users.json is not valid JSON")?;	

	Ok(root.users)
}


pub fn login(users: &[User]) -> User {
	loop {
		println!("Enter your username:");
		let mut username: String = String::new();
		stdin()
			.read_line(&mut username)
			.expect("Failed to read line. Type 'h' for help.");

		let username: &str = username.trim();

		if username == "c" || username == "cancel" {
			logger(&Message {
				content: String::from("Login cancelled. Returning to main menu."),
				level: 102,
			});
			return users[0].clone();
		}

		if username == "h" || username == "help" {
			login_help();
			continue;
		}

		if !validate(username) {
			continue;
		}

		println!("Enter your password:");
		let mut password: String = String::new();
		stdin()
			.read_line(&mut password)
			.expect("Failed to read line. Type 'h' for help.");

		let password: &str = password.trim();

		if !validate(password) {
			continue;
		}

		for user in users {
			if user.username == username && user.password == password {
				logger(&crate::logger::Message {
					content: format!("User '{}' logged in successfully.", username),
					level: 200,
				});
				return user.clone();
			}
		};

		logger(&Message {
			content: String::from("Invalid username or password. Please try again."),
			level: 404,
		});
	}	
}


pub fn validate(input: &str) -> bool {
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
