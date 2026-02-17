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