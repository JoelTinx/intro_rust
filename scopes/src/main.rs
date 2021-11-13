fn main() {
	// SCOPES
	let message = "Hello! I'm a message";
	println!("{}", message);

	{
		println!("{}", message);
		
		let message2 = "Hello! I'm a message 2";
		println!("{}", message2);
		
		let message = "Hello! I'm the same message but in other scope";
		println!("{}", message);
	}

	println!("{}", message);
	// println!("{}", message2); // does not exist in this scope


	//SCOPES 2
	let resultado = {
		println!("Hello! I'm a nested variable");
		let variable: i32 = 200;
		println!("{}", variable);
		variable // the last statement is returned
	};
	println!("{}", resultado);

	//example 2
	let score: i32 = 8;
	let mut message = String::new();

	if score > 10 {
		message = String::from("You passed 😊");
	} else {
		message = String::from("You failed 😑");
	}


	let score: i32 = 8;

	let message = if score > 10 {
		String::from("You passed 😊");
	} else {
		String::from("You failed 😑");
	};

	println!("{:?}", message);
}
