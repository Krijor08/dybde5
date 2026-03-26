pub fn help() {
	println!("\n-------------------------  HELP  -------------------------\n");
	println!  ("Available commands:\n");
	println!  ("-  ip ----------- Validate an IP address");
	println!  ("-  login, l ----- Log in to your account");
	println!  ("-  signup, su --- Create a new user account");
	println!  ("-  eal ---------- Edit a user's access level (admin only)");
	println!  ("-  script, s ---- Run a predefined script (Linux only)");
	println!  ("-  help, h ------ Show this help message");
	println!  ("-  exit, e ------ Exit the program");
	println!  ("__________________________________________________________\n");
}			  

pub fn login_help() {
	println!("\n----------------------  LOGIN HELP  ----------------------\n");
	println!  ("Enter your username when prompted.");
	println!  ("Enter your password when prompted.");
	println!  ("If you don't have an account, type 'signup' or 'su' to create one.");
	println!  ("Type 'c' to cancel login and return to the main menu.");
	println!  ("Type 'h' to show this help message again.");
	println!  ("Username and password must be at least 3 characters long and cannot contain spaces.\n\n");
	println!  ("__________________________________________________________\n");
}

pub fn script_help() {
	println!("\n----------------------  SCRIPT HELP  ----------------------\n");
	println!  ("To run a script, simply enter the name of the script file located in the 'scripts' directory.");
	println!  ("Make sure to enter the exact name of the script, including the file extension (e.g., 'example.sh').");
	println!  ("Do not include any path information or special characters in the script name.");
	println!  ("If you need help, type 'help' or 'h' to see this message again.\n\n");
	println!  ("___________________________________________________________\n");
}