use crate::logger::logger;
use crate::logger::Message;


pub fn ip() {
	let ip: String = crate::input::input("Enter an IP address to check:");

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
	logger(&Message {
		content: format!("Octets: {}", octets.len().to_string()),
		level: 102,
	});
	if octets.len() != 4 {
		return false;
	}

	for octet in octets {
		logger(&Message {
			content: format!("Checking octet: {}", octet),
			level: 102,
		});
		match octet.parse::<u8>() {
			Ok(_) => continue,
			Err(_) => {
				logger(&Message {
					content: format!("Failed to parse octet: {}", octet),
					level: 400,
				});
				return false;
			}
		}
	}
	true
}
