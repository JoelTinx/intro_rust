fn main() {
	// ENUMS
	enum Response {
		Success,
		Error(u32, String),
	}

	let response = Response::Error(5001, String::from("Is not posible complete the action"));

	match response {
		Response::Success => println!("Success"),
		Response::Error(403, _) => println!("Forbidden"),
		Response::Error(404, _) => println!("Not Found"),
		Response::Error(500, _) => println!("Internal server error"),
		Response::Error(_, message) => println!("{}", message),
		// Response::Error(_) => println!("Is not posible complete the action"),
	}

}
