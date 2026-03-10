pub fn help() {
	println!("Available commands:");
	println!("ip - Validate an IP address");
	println!("login (or l) - Log in to your account");
	println!("signup (or su) - Create a new user account");
	println!("script (or s) - Run a predefined script (Linux only)");
	println!("help (or h) - Show this help message");
	println!("exit (or e) - Exit the program");
}

pub fn login_help() {
	println!("Login Help:");
	println!("Enter your username when prompted.");
	println!("Enter your password when prompted.");
	println!("If you don't have an account, type 'signup' or 'su' to create one.");
	println!("Type 'c' to cancel login and return to the main menu.");
	println!("Type 'h' to show this help message again.");
	println!("Usernames must be at least 3 characters long and cannot contain spaces.");
}