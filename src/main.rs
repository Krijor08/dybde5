mod bashrun;
mod help;
mod input;
mod ipchecker;
mod logger;
mod users;

use users::User;
use logger::Message;

use std::{env::consts};
use tokio;

use bashrun::run_script;
use help::help;
use input::input;
use ipchecker::ip;
use logger::logger;
use users::{ create_user, get_users, login, update_access_level };


#[tokio::main]
async fn main() {
	let os_type: String = consts::OS.to_string();
	let msg: Message = Message {
		content: format!("Running on OS: {}", os_type),
		level: 100,
	};
	logger(&msg);

	let msg: Message = Message {
		content: String::from("Program started successfully."),
		level: 102,
	};
	logger(&msg);

	let mut users: Vec<User> = get_users().unwrap();

	let mut current_user: Option<User> = Some(get_users().unwrap()[0].clone());

	loop {
		let command: String = input("\nEnter a command (type 'help' for a list of commands):");
	
		match command.as_str() {
			"exit"	| "e" => {
				let msg: Message = Message {
					content: String::from("Exiting program."),
					level: 102,
				};
				logger(&msg);
				return;
			},
			
			"help"	| "h" => help(),

			"ip" => {
				if check_access(&current_user.as_ref().unwrap(), 10) {
					ip();
				}
			},

			"login"	 | "l" => 	{
				current_user = Some(login(&users));
				logger(&Message {
					content: format!("User '{}' logged in.", current_user.as_ref().unwrap().username),
					level: 100,
				});
			},

			"script" | "s" => {
				if os_type != "linux" {
					let msg: Message = Message {
						content: String::from("Script execution is only supported on Linux."),
						level: 400,
					};
				logger(&msg);
				return;
				}
				if check_access(current_user.as_ref().unwrap(), 50) {
					if let Err(e) = run_script().await {
						let msg: Message = Message {
							content: format!("Failed to run script: {}", e),
							level: 500,
						};
						logger(&msg);
					}
				}
			},

			"signup" | "su" => {
				let new_user = create_user(&users, &current_user.as_ref().unwrap().access_level);
				if let Some(user) = new_user {
					logger(&Message {
						content: format!("User '{}' created successfully.", user.username),
						level: 201,
					});
				}
				users = get_users().unwrap();
			},

			"editaccesslevel" | "eal" => {
				if !check_access(current_user.as_ref().unwrap(), 100) {
					logger(&Message {
						content: String::from("You do not have permission to edit access levels."),
						level: 403,
					});
					return;
				}
				let target_username: String = input("Enter the username of the user to edit:");
				let new_access_level: String = input("Enter the new access level (0-100):");
				let level = new_access_level.parse::<u8>();
				let level = match level {
					Ok(num) => num,
					Err(_) => {
						logger(&Message {
							content: String::from("Invalid access level. Please enter a number between 0 and 100."),
							level: 400,
						});
						return;
					}
					};
				if &level > &current_user.as_ref().unwrap().access_level {
					logger(&Message {
						content: String::from("Cannot set access level higher than your own."),
						level: 403,
					});
					return;
				}
				if let Err(e) = update_access_level(&[current_user.as_ref().unwrap().clone()], &target_username, level) {
					logger(&Message {
						content: format!("Failed to edit access level: {}", e),
						level: 500,
					});
					return;
				}
				logger(&Message {
					content: format!("Access level for user '{}' updated successfully.", target_username),
					level: 200,
				});
				users = get_users().unwrap();	
			},

			_ => logger(&Message {
				content: format!("Unknown command: '{}'. Type 'help' for a list of commands.", command),
				level: 404,
			}),
		}
	}
}


fn check_access(current_user: &User, required_level: u8) -> bool {
	if current_user.access_level >= required_level {
		let msg: Message = Message {
			content: format!("Welcome, {}!", current_user.username),
			level: 100,
		};
		logger(&msg);
	} else {
		let msg: Message = Message {
			content: format!("Access denied for user: {}", current_user.username),
			level: 403,
		};
		logger(&msg);
		return false;
	}
	true
}
