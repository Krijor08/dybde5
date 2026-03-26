pub struct Message {
	pub(crate) content: String,
	pub(crate) level: u16,
}


pub fn logger(message: &Message) {
	match message.level {
		100 => println!("[INFO]: {}", message.content),
		101 => println!("[OUTPUT]: {}", message.content),
		102 => println!("[PROCESSING]: {}", message.content),
		200 => println!("[OK]: {}", message.content),
		201 => println!("[CREATED]: {}", message.content),
		400 => println!("[BAD REQUEST]: {}", message.content),
		401 => println!("[UNAUTHORIZED]: {}", message.content),
		403 => println!("[FORBIDDEN]: {}", message.content),
		404 => println!("[NOT FOUND]: {}", message.content),
		409 => println!("[CONFLICT]: {}", message.content),
		418 => println!("[I'M A TEAPOT]: {}", message.content),
		500 => println!("[INTERNAL SERVER ERROR]: {}", message.content),
		501 => println!("[NOT IMPLEMENTED]: {}", message.content),
		503 => println!("[SERVICE UNAVAILABLE]: {}", message.content),
		_ => println!("[UNKNOWN]: {}", message.content),
	}
}
