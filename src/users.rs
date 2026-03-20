use serde::{Deserialize, Serialize};
use serde_json;

use crate::{logger, Message};
use crate::help::login_help;
use crate::input::{input, validate_input};


#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct User {
 	pub(crate) username: String,
 	pub(crate) email: String,
	pub(crate) password: String,
 	pub(crate) access_level: u8,
	pub(crate) age: u8,
}


#[derive(Debug, Deserialize, Serialize)]
pub struct Root {
    users: Vec<User>,
}


pub fn create_user(users: &[User], current_access_level: &u8) -> Option<User> {
	let username: String = input("Enter new username:");

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

	if !validate_input(&username) {
		return None;
	}

	for user in users {
		if user.username == username {
			logger(&Message {
				content: String::from("Username already exists. Please try again."),
				level: 409,
			});
			return None;
		}
	};

	let email: String = input("Enter email address:");

	if !validate_input(&email) {
		return None;
	}

	let password: String = input("Enter password:");

	if !validate_input(&password) {
		return None;
	}

	let access_level_str: String = input("Enter access level (0-255):");
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

	if &access_level > current_access_level {
		logger(&Message {
			content: String::from("Cannot create user with higher access level than your own."),
			level: 403,
		});
		return None;
	}

	let age_str: String = input("Enter age:");

	let age: u8 = match age_str.parse() {
		Ok(num) => num,
		Err(_) => {
			logger(&Message {
				content: String::from("Invalid age. Please enter a valid number."),
				level: 400,
			});
			return None;
		}
	};

	let new_user = User {
		username: username.to_string(),
		email: email.to_string(),
		password: password.to_string(),
		access_level,
		age,
	};

	let mut all_users = users.to_vec();
	all_users.push(new_user.clone());

	match serde_json::to_string_pretty(&Root { users: all_users }) {
		Ok(json) => {
			if let Err(e) = std::fs::write("./user.json", json) {
				logger(&Message {
					content: format!("Failed to write to user.json: {}", e),
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
	let content: String = std::fs::read_to_string("./user.json")
        .map_err(|_| "Could not read user.json")?;

	let root: Root = serde_json::from_str(&content)
		.map_err(|_| "user.json is not valid JSON")?;	

	Ok(root.users)
}


pub fn login(users: &[User]) -> User {
	loop {
		let username: String = input("Enter your username:");

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

		if !validate_input(&username) {
			continue;
		}

		let password: String = input("Enter your password:");

		if !validate_input(&password) {
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

pub fn update_access_level(users: &[User], target_username: &str, new_access_level: u8) -> Result<(), &'static str> {
	let mut all_users = users.to_vec();
	let mut user_found = false;

	for user in &mut all_users {
		if user.username == target_username {
			user.access_level = new_access_level;
			user_found = true;
			break;
		}
	}

	if !user_found {
		return Err("User not found");
	}

	match serde_json::to_string_pretty(&Root { users: all_users }) {
		Ok(json) => {
			std::fs::write("./user.json", json)
				.map_err(|_| "Failed to write to user.json")?;
			Ok(())
		},
		Err(_) => Err("Failed to serialize users"),
	}
}
