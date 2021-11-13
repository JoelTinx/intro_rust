fn main() {
	// SHADOWING
	let valor: i32 = 10;

	println!("Valor 1: {}", valor);
	
	let valor = 20; // Shadowing
	println!("Valor 2: {}", valor);

	let valor = 30; // Shadowing
	println!("Valor 3: {}", valor);
}
