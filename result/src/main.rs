fn main() {
	// let value = divide(12, 0);
	// println!("{:?}", value.unwrap_or(0));

	match divide(12, 0) {
		Ok(value) => println!("{:?}", value),
		Err(message) => println!("{:?}", message),
	}
}

// enum Result<T, E> {
// 		Ok(T), // When everything is ok
// 		Err(E), // When there is an error
// }

fn divide(dividend: i32, divisor: i32) -> Result<i32, String> {
	if divisor == 0 {
		Err(String::from("Division by zero"))
	} else {
		Ok(dividend / divisor)
	}
}