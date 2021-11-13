fn main() {
	let user = User {
		email: String::from("joeltinx@gmail.com"),
		username: String::from("joeltinx"),
		active: true,
		password: String::from("password_hash"),
		// age: Some(33),
		age: None,
	};
	// let age = user.age.unwrap();

	println!("{:?}", user);
	// println!("The user's age is: {:?}", age);

	match user.age {
		Some(age) => println!("The user's age is: {}", age),
		None => println!("The user doesn't have an age"),
	}
}

#[derive(Debug)]
struct User {
	username: String,
	email: String,
	active: bool,
	password: String,
	age: Option<u32>
}