use std::io;

fn main() {
	// LECTURA DE DATOS
	let mut username = String::new();
	println!("Input the username: ");
	io::stdin().read_line(&mut username);
	let username = username.trim();
	
	println!("How old are you: ");
	let mut age = String::new();
	io::stdin().read_line(&mut age);
	let age = age.trim();
	let aget: i32 = age.parse().unwrap();

	println!("The name of the user is: {} and the age is: {}", username, age);
}
