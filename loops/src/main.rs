fn main() {
	// LOOPS
	let mut counter = 0;
	loop {
		counter += 1;
		println!("{}", counter);
		if counter == 10 {
			break;
		}
	}

	// FOR LOOPS
	let numbers: [i32; 5] = [1, 2, 3, 4, 5];
	for number in numbers.iter() {
		println!("{}", number);
	}

	for number in 1..11 {
		println!("{}", number);
	}

	for number in 1..101 {
		if number % 3 == 0 && number % 5 == 0 {
			println!("Fizz Buzz");
		} else if number % 3 == 0 {
			println!("Fizz");
		} else if number % 5 == 0 {
			println!("Buzz");
		} else {
			println!("{}", number);
		}
	}

	// WHILE LOOPS
	let mut counter = 0;

	while counter <= 10 {
		println!("{}", counter);
		counter += 1;
	}

	let mut number = 1234;
	let mut counter = 0;

	while number > 0 {
		number /= 10;
		counter += 1;
	}

	println!("the number has {} digits", counter);

}
