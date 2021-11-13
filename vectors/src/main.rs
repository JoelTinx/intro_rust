fn main() {
	// VECTORES
	let mut vector = vec![1, 2, 3];
	println!("{:?}", vector);
	vector.push(4);
	println!("{:?}", vector);
	vector.pop();
	vector.pop();
	println!("{:?}", vector);
	vector.insert(1, 0);
	println!("{:?}", vector);
	vector.remove(0);
	println!("{:?}", vector);
	vector[0] = 1;
	println!("{:?}", vector);

	// VECTORES PART 2
	let mut vector: Vec<i32> = Vec::new(); // []
	vector.push(1);
	vector.push(2);
	vector.push(3);
	vector.push(4);
	vector.push(5);

	println!("{:?}", vector);
}
