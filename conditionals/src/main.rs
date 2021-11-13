use std::io;

fn main() {
	// CONDITIONALS
	println!("Input a color: ");
	let mut color = String::new();
	io::stdin().read_line(&mut color);
	let color = color.trim().to_lowercase();

	if color == "green" {
		println!("you can pass")
	} else if color == "yellow" {
		println!("wait a moment")
	} else if color == "red" {
		println!("stop")
	} else {
		println!("inavlid color")
	}

}
