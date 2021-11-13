fn main() {
  match divide(12, -4) {
		Ok(result) => println!("{}", result),
		Err(DivideResult::DivideByZero) => {
			println!("Divide by zero");
		},
		Err(DivideResult::DivideNegative) => {
			println!("Divide negative");
		},
	}
}

enum DivideResult {
		DivideByZero,
		DivideNegative,
}

fn divide(dividend: i32, divisor: i32) -> Result<i32, DivideResult> {
	if divisor == 0 {
		return Err(DivideResult::DivideByZero);
	}

	if dividend < 0 || divisor < 0 {
		return Err(DivideResult::DivideNegative);
	}

	Ok(dividend / divisor)
}