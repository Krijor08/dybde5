mod users;
mod logger;
mod ipchecker;

use std::io;
use users::User;
use logger::Message;
use users::{get_users, login};
use logger::logger;
use ipchecker::ip_checker;

fn main() {
	let users: Vec<User> = get_users().unwrap();

	let current_user: User = login(&users);

	if current_user.access_level > 50 {
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
		return;
	}

	let msg: Message = Message {
		content: String::from("Program started successfully."),
		level: 102,
	};
	logger(&msg);

	let mut ip: String = String::new();
	println!("Enter an IP address to validate:");
	io::stdin()
        .read_line(&mut ip)
        .expect("Failed to read line");

	if ip_checker(ip) {
		let success: Message = Message {
			content: String::from("Valid IP address."),
			level: 200,
		};
		logger(&success);
	} else {
		let fail: Message = Message {
			content: String::from("Invalid IP address."),
			level: 400,
		};
		logger(&fail);
	}
}

