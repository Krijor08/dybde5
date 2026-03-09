pub fn help() {
	println!("Available commands:");
	println!("ip - Validate an IP address");
	println!("login (or l) - Log in with your username");
	println!("script (or s) - Run a predefined script (Linux only)");
	println!("help (or h) - Show this help message");
	println!("exit (or e) - Exit the program");
}

pub fn login_help() {
	println!("Login Help:");
	println!("Enter your username when prompted.");
	println!("Enter your password when prompted.");
	println!("If you don't have an account, please contact your administrator.");
	println!("Type 'c' to cancel login and exit the program.");
	println!("Type 'h' to show this help message again.");
	println!("Usernames must be at least 3 characters long and cannot contain spaces.");
}