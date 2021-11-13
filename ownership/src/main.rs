fn main() {
	// OWNERSHIP
	/*
	1.- Each value in Rust has a variable that’s called its owner.
	2.- There can only be one owner at a time.
	3.- When the owner goes out of scope, the value will be dropped.
	*/
	let rectangle = Rectangle { width: 10, height: 20 };
	let area = area(&rectangle);
	println!("{}", area);
	println!("Rectangle defined with: {} - {}", rectangle.width, rectangle.height);

	// Argument is passed by value, and the value is borrowed and moved into the function.
	// let area = area2(rectangle);
	// println!("{}", area);
	// println!("Rectangle defined with: {} - {}", rectangle.width, rectangle.height); // ERROR because rectangle is moved

	let new_rectangle = rectangle;
	// println!("{}", rectangle.width); // ERROR because rectangle is moved
	println!("{}", new_rectangle.width);

	// the variables that are defined in heap are moved, but the variables that are defined in stack are copied. 
}

// OWNERSHIP
struct Rectangle {
	width: u32,
	height: u32,
}

fn area(rect: &Rectangle) -> u32 {
	rect.width * rect.height
}

fn area2(rect: Rectangle) -> u32 {
	rect.width * rect.height
}