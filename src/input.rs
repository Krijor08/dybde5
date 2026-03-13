use std::io::stdin;

pub fn input(prompt: &str) -> String {
	println!("{}", prompt);
	let mut input: String = String::new();
	stdin()
		.read_line(&mut input)
		.expect("Failed to read line. Type 'h' for help.");
	input.trim().to_string()
}