fn main() {
	// O tamanho de um char é de 4 bytes
	let letra: char = 'C';
	println!("Tamanho do char = {} bytes", std::mem::size_of_val(&letra));
}
