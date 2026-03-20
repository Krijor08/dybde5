pub fn help() {
	println!("\n---------------- HELP ----------------\n");
	println!("Available commands:\n");
	println!("-  ip _____________ Validate an IP address");
	println!("-  login, l ___ Log in to your account");
	println!("-  signup, su _ Create a new user account");
	println!("-  editaccesslevel, eal ___ Edit a user's access level (admin only)");
	println!("-  script, s __ Run a predefined script (Linux only)");
	println!("-  help, h ____ Show this help message");
	println!("-  exit, e ____ Exit the program");
}

pub fn login_help() {
	println!("\n---------------- LOGIN HELP ----------------\n");
	println!("Enter your username when prompted.");
	println!("Enter your password when prompted.");
	println!("If you don't have an account, type 'signup' or 'su' to create one.");
	println!("Type 'c' to cancel login and return to the main menu.");
	println!("Type 'h' to show this help message again.");
	println!("Usernames must be at least 3 characters long and cannot contain spaces.\n\n");
}