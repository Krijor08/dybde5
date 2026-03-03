use crate::logger::logger;
use crate::logger::Message;
use std::io;

pub fn ip() {
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

pub fn ip_checker(mut ip: String) -> bool {
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
