fn main() {
  let result = divide(12, 4);
	println!("{:?}", result.unwrap());
  let result = divide(12, 0);
	println!("{:?}", result.unwrap_or(0));
  let result = divide(12, -5);
	println!("{:?}", result.expect("Negative divide"));
}

#[derive(Debug)]
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