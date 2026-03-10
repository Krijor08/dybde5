mod bashrun;
mod help;
mod ipchecker;
mod logger;
mod users;

use users::User;
use logger::Message;

use std::{env::consts, io::stdin};
use tokio;

use bashrun::run_script;
use help::help;
use ipchecker::ip;
use logger::logger;
use users::{ create_user, get_users, login };

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

	let users: Vec<User> = get_users().unwrap();

	let mut current_user: User = get_users().unwrap()[0].clone();

	loop {
		println!("\nEnter command:");
		let mut command: String = String::new();
		stdin()
			.read_line(&mut command)
			.expect("Failed to read line");

		let command: &str = command.trim();
	
		match command {
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
				if check_access(&current_user, 10) {
					ip();
				}
			},

			"login"	 | "l" => 	current_user = login(&users),

			"script" | "s" => {
				if os_type != "linux" {
					let msg: Message = Message {
						content: String::from("Script execution is only supported on Linux."),
						level: 400,
					};
				logger(&msg);
				return;
				}
				if check_access(&current_user, 50) {
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
				let new_user = create_user(&users, current_user.access_level);
				if let Some(user) = new_user {
					logger(&Message {
						content: format!("User '{}' created successfully.", user.username),
						level: 201,
					});
				}
			},

			_ => println!("Unknown command. Type 'help' for a list of commands."),
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
}
