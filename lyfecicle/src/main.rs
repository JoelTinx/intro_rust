fn main() { // Block: 1
	// let message = "Hello, world!"; // Variable: message
	// println!("Block 1: {}", message); // Expression: println!("{}", message)

	// { // Block: 2
	// 	let message = "Hello World defined in Block 2";
	// 	println!("Block 2: {}", message); // Expression: println!("{}", message)
	// 	{ // Block: 3
	// 		let message = "Hello World defined in Block 3";
	// 		println!("Block 3: {}", message); // Expression: println!("{}", message)
	// 	} // Block: 3
	// }
	// println!("Block 1 (end): {}", message);

	let mut message = String::from("Hello, Im a variable for borrowing!");
	println!("Block 1: {}", message);
	
	{ // Block: 2
		let borrow = &message; // Borrowing (if you don't have to use the variable after the block, you can borrow it)
		println!("Block 2: {}", message);
		println!("Block 2: {}", borrow);
		// ERROR BECAUSE THE VARIABLE IS BORROWED
		// message = String::from("value changed!"); // FREEZE THE VARIABLE
		// println!("Block 2: {}", borrow);
	}
	println!("Block 1: {}", message);
}
