fn main() {
	// Option -> It is a container that can hold a value or nothing.
	// Result -> It is a container that can hold a value or an error.
	let result = get_value(false);
	// match result {
	// 	Some(value) => println!("{}", value),
	// 	None => println!("No value"),
	// }
	// let value = result.unwrap();
	// let value = result.unwrap_or("No value".to_string());
	let value = result.expect("Value hoped was an String");

	println!("{}", value);
}

// enum Option<T> {
// 	Some(T), // -> The value is present. 
// 	None, // -> The value is absent.
// }

fn get_value(flag: bool) -> Option<String> {
	if flag {
		Some(String::from("I'm a message for the tuple Some"))
	} else {
		None
	}
}
