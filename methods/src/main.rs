fn main() {
	// METHODS
	let mut user = User {
		username: String::from("joeltinx"),
		email: String::from("joeltinx@gmail.com"),
		password: String::from("123456"),
	};

	user.greeat_message();
	user.change_password(String::from("1234567"));

	println!("{:?}", user);
}

// METHODS
#[derive(Debug)]
struct User {
	username: String,
	email: String,
	password: String,
}

impl User {
	fn greeat_message(&self) {
		println!("Hello {}", self.username);
	}

	fn change_password(&mut self, new_password: String) {
		self.password = new_password;
	}
}
