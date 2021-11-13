fn main() {
	// ARRAYS
	let numbers = [1, 2, 3, 4, 5];
	println!("First Element: {:?}", numbers);
	println!("First Element: {}", numbers[0]);
	println!("Last Element: {}", numbers[numbers.len() - 1]);

	let mut numbers = [1, 2, 3, 4, 5];
	numbers[2] = 30;
	println!("First Element: {:?}", numbers);

	let numbers: [i32; 5] = [2, 4, 6, 8, 10];
	println!("odd numbers: {:?}", numbers);
	
	let values = [1.8; 5];
}
