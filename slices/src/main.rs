fn main() {
	// slices -> Heap allocated
	// arrays -> Stack allocated

	let message = String::from("Hello from Rust");

	// let hello = &message[0..5];
	let hello = &message[..5];
	let rest_of_message = &message[5..];
	let full_message = &message[..];

	println!("{}", message);
	println!("{}", hello);
	println!("{}", rest_of_message);
	println!("{}", full_message);
}
