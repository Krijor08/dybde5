use std::io::stdin;
use serde::{Deserialize, Serialize};
use serde_json;

use crate::logger;
use crate::help::login_help;

use crate::logger::Message;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct User {
 	pub(crate) username: String,
 	pub(crate) email: String,
	pub(crate) password: String,
 	pub(crate) access_level: u8,
}

#[derive(Debug, Deserialize)]
pub struct Root {
    users: Vec<User>,
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

		if username.is_empty() {
			logger(&Message {
				content: String::from("Username cannot be empty. Please try again."),
				level: 400,
			});
			continue;
		}

		if username.contains(' ') {
			logger(&Message {
				content: String::from("Username cannot contain spaces. Please try again."),
				level: 400,
			});
			continue;
		}

		if username.len() < 3 {
			logger(&Message {
				content: String::from("Username must be at least 3 characters long. Please try again."),
				level: 400,
			});
			continue;
		}

		if username == "h" {
			login_help();
			continue;
		}

		if username == "c" {
			logger(&Message {
				content: String::from("Login cancelled. Exiting program."),
				level: 200,
			});
			return User {
				username: String::from(""),
				email: String::from(""),
				password: String::from(""),
				access_level: 0,
			};
		}			

		for user in users {
			if user.username == username {
				logger(&crate::logger::Message {
					content: format!("User '{}' logged in successfully.", username),
					level: 200,
				});
				return user.clone();
			}
		};

		logger(&Message {
			content: String::from("Username not found. Please try again."),
			level: 404,
		});
	}	
}
