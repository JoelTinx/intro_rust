fn main() {
	// TUPLES
	let tupla = (10, false, 2.4, "🙂");

	println!("{:?}", tupla);
	println!("{}", tupla.3);

	let mut tupla: (i32, bool, f64, &str) = (10, false, 2.4, "🙂");
	println!("{:?}", tupla);
	println!("First element: {:?}", tupla.0);
	println!("Last element: {:?}", tupla.3);
	tupla.3 = "😑";
	println!("Last element: {:?}", tupla.3);
}
