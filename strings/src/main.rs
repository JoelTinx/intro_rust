fn main() {
	// STRINGS (STACK / HEAP)
	let variable_str = "Hello, Im a string. Thanks";
	let mut variable_string = String::from("Hello");

	variable_string.push(',');
	variable_string.push(' ');
	variable_string.push_str("Im a string. Thanks");

	let new_string = "Hello, Im a String. Thanks".to_string(); // str to string

	println!("{}", variable_str);
	println!("{}", variable_string);
	println!("{}", new_string);
}
