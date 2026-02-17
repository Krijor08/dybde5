use std::io::{self, stdin};
use serde::{Deserialize, Serialize};
use serde_json;

struct Message {
	content: String,
	level: u16,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
struct User {
 	username: String,
 	email: String,
 	access_level: u8,
}

#[derive(Debug, Deserialize)]
struct Root {
    users: Vec<User>,
}

fn get_users() -> Result<Vec<User>, &'static str> {
	let content = std::fs::read_to_string("./users.json")
        .map_err(|_| "Could not read users.json")?;

	let root: Root = serde_json::from_str(&content)
		.map_err(|_| "users.json is not valid JSON")?;	

	Ok(root.users)
}

fn get_current_user(users: &[User]) -> User {
	println!("Enter your username:");
	let mut username = String::new();
	stdin().read_line(&mut username).expect("Failed to read line");
	let username = username.trim();
	for user in users {
		if user.username == username {
			return user.clone();
		}
	}
	panic!("User not found");
}

fn logger(message: &Message) {
	match message.level {
		100 => println!("[INFO]: {}", message.content),
		102 => println!("[PROCESSING]: {}", message.content),
		200 => println!("[OK]: {}", message.content),
		201 => println!("[CREATED]: {}", message.content),
		400 => println!("[BAD REQUEST]: {}", message.content),
		401 => println!("[UNAUTHORIZED]: {}", message.content),
		403 => println!("[FORBIDDEN]: {}", message.content),
		418 => println!("[I'M A TEAPOT]: {}", message.content),
		500 => println!("[INTERNAL SERVER ERROR]: {}", message.content),
		_ => println!("[UNKNOWN]: {}", message.content),
	}
}

fn ip_checker(mut ip: String) -> bool {
	ip = ip.trim().to_string();
	let octets: Vec<&str> = ip.split('.').collect();
	println!("Octets: {}", octets.len().to_string());
	if octets.len() != 4 {
		return false;
	}
	for octet in octets {
		println!("Checking octet: {}", octet);
		match octet.parse::<u8>() {
			Ok(_) => continue,
			Err(_) => {
				println!("Failed to parse octet: {}", octet);
				return false;
			}
		}
	}
	true
}

fn main() {
	let users = get_users().unwrap();

	let current_user = get_current_user(&users);

	if current_user.access_level > 50 {
		let msg = Message {
			content: format!("Welcome, {}!", current_user.username),
			level: 100,
		};
		logger(&msg);
	} else {
		let msg = Message {
			content: format!("Access denied for user: {}", current_user.username),
			level: 403,
		};
		logger(&msg);
		return;
	}

	let msg = Message {
		content: String::from("Program started successfully."),
		level: 102,
	};
	logger(&msg);

	let mut ip = String::new();
	println!("Enter an IP address to validate:");
	io::stdin()
        .read_line(&mut ip)
        .expect("Failed to read line");

	if ip_checker(ip) {
		let success = Message {
			content: String::from("Valid IP address."),
			level: 200,
		};
		logger(&success);
	} else {
		let fail = Message {
			content: String::from("Invalid IP address."),
			level: 400,
		};
		logger(&fail);
	}
}

