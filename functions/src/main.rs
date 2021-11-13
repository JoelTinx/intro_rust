fn main() {
	// FUNCTIONS
	hello_wold();

	let imc = compute_imc(72.0, 1.63);
	println!("{}", imc);

	let result_factorial = factorial(5);
	println!("{}", result_factorial);
}

fn hello_wold() {
	println!("Hello, world!");
}

fn compute_imc(weight: f32, height: f32) -> f32 {
	weight / (height * height)
}

fn factorial(number: u32) -> u32 {
	if number == 1 {
		1
	} else {
		number * factorial(number - 1)
	}
}
