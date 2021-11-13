fn main() {
	let message = Some("Hello World");

	// match message {
	// 	Some(s) => println!("{}", s),
	// 	None => (),
	// }

	match message {
		Some("Hello World") => println!("The message is: 'Hello World'"),
		Some("Good bye") => println!("The message is: 'Good bye'"),
		Some(_) => println!("Some other message"),
		None => println!("There is not a value"),
	}
}
