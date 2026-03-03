mod bashrun;
mod ipchecker;
mod logger;
mod users;

use users::User;
use logger::Message;

use std::{env::consts, io::stdin};

use bashrun::run_ip_script;
use ipchecker::ip;
use logger::logger;
use users::{get_users, login};


fn main() {
	let os_type: String = consts::OS.to_string();
	let msg: Message = Message {
		content: format!("Running on OS: {}", os_type),
		level: 101,
	};
	logger(&msg);

	let msg: Message = Message {
		content: String::from("Program started successfully."),
		level: 102,
	};
	logger(&msg);

	loop {
		println!("\nEnter command:");
		let mut command: String = String::new();
		stdin()
			.read_line(&mut command)
			.expect("Failed to read line");

		let command: &str = command.trim();

		let users: Vec<User> = get_users().unwrap();
		
		let mut current_user: User = User {
			username: String::from(""),
			email: String::from(""),
			access_level: 0,
		};

		match command {
			"ip" => {
				if check_access(&current_user, 10) {
					ip();
				}
			},
			"login" => 	 current_user = login(&users),
			"script" => {
				if os_type != "linux" {
					let msg: Message = Message {
						content: String::from("Script execution is only supported on Linux."),
						level: 400,
					};
				logger(&msg);
				return;
				}
				if check_access(&current_user, 50) {
					if let Err(e) = run_ip_script() {
						let msg: Message = Message {
							content: format!("Failed to run script: {}", e),
							level: 500,
						};
						logger(&msg);
					}
				}
			},
			"help" => println!("Available commands: ip, login, script, help, exit"),
			"exit" => {
				let msg: Message = Message {
					content: String::from("Exiting program."),
					level: 102,
				};
				logger(&msg);
				return;
			},
			_ => println!("Unknown command. Type 'help' for a list of commands."),
		}
		println!("Current user: {} (Access level: {})", current_user.username, current_user.access_level);
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

