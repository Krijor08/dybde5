use std::io::stdin;
use serde::{Deserialize, Serialize};
use serde_json;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct User {
 	pub(crate) username: String,
 	email: String,
 	pub(crate) access_level: u8,
}

#[derive(Debug, Deserialize)]
pub struct Root {
    users: Vec<User>,
}



pub fn get_users() -> Result<Vec<User>, &'static str> {
	let content: String = std::fs::read_to_string("./users.json")
        .map_err(|_| "Could not read users.json")?;

	let root: Root = serde_json::from_str(&content)
		.map_err(|_| "users.json is not valid JSON")?;	

	Ok(root.users)
}

pub fn login(users: &[User]) -> User {
	loop {
		println!("Enter your username:");
		let mut username: String = String::new();
		stdin()
			.read_line(&mut username)
			.expect("Failed to read line");

		let username: &str = username.trim();
		for user in users {
			if user.username == username {
				return user.clone();
			}
		};
		println!("Username not found. Please try again.\n");
	}	
}