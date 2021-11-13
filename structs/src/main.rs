fn main() {
	// STRUCTS
	let user = User {
		name: String::from("John"),
		age: 20
	};

	println!("{}", user.name);
	println!("{}", user.age);


	let name = String::from("John");
	let age = 20;

	let user = User { name, age };
	
	println!("{}", user.name);
	println!("{}", user.age);
	
	let mut user2 = create_user(String::from("Jeol"), 33);

	user2.name = String::from("Joel");

	println!("{}", user2.name);
	println!("{}", user2.age);
}

// STRUCTS
struct User {
	name: String,
	age: u8
}

fn create_user(name: String, age: u8) -> User {
	User {
		name,
		age
	}
}