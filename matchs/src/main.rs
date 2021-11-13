fn main() {
	let number: i32 = 10;

	match number {
		1 => println!("one"),
		2 => println!("two"),
		3 => println!("three"),
		4 | 5 | 6 => println!("four or five or six"),
		7..=100 => {
			println!("seven to a hundred");
			println!("the number is evaluated by a range in a and the action is a block")
		},
		_ => println!("anything"),
	}


	let message = match number {
		1 => "one",
		2 => "two",
		3 => "three",
		4 | 5 | 6 => "four or five or six",
		7..=100 => {
			let response = "seven to a hundred";
			response
		},
		_ => "anything",
	};

	println!("{}", message);


	for number in 1..=30 {
		match (number % 3, number % 5) {
			(0, 0) => println!("Fizz Buzz"),
			(0, _) => println!("Fizz"),
			(_, 0) => println!("Buzz"),
			_ => println!("{}", number),
		}
	}
}
