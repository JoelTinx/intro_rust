fn main() {
	// TIPOS DE DATOS
	// i8, i16, i32, i64, i128 -> Entero con signo +/-
	// u8, u16, u32, u64, u128 -> Entero sin signo
	let number_one: i8 = -10;
	let number_two: u8 = 10;

	// CHAR -> UTF-8
	let character = "🍕";

	// f32 - f64
	let real: f32 = 12.45;

	// Boolean
	let status: bool = true;


	println!("The value of the first number is: {}", number_one);
	println!("The value of the second number is: {}", number_two);

	println!("Hello from {}", character);
	println!("Real: {}", real);
	
	println!("Boolean: {}", status);
}
