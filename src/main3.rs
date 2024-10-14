fn main() {
	// Exemplo 1
	// Se o tipo não for especificado, o rust assume um padrão
	// No caso, verificamos que o tamanho da variável é de 4 bytes, portanto, o padrão assumido foi i32
	let variavel = 300;
	println!("Variavel = {}, tamanho = {} bytes", variavel, std::mem::size_of_val(&variavel));

	// Exemplo 2
	// Se o tipo não for especificado, o rust assume um padrão
	// No caso, verificamos que o tamanho da variável é de 8 bytes, portanto, o padrão assumido foi f64
	let decimal1 = 2.5;
	println!("Decimal1 = {}, tamanho = {} bytes", decimal1, std::mem::size_of_val(&decimal1));

	// Exemplo 3
	// Especificando o tipo (f32), teremos que o tamanho da variável é de 4 bytes (32 bits)
	let decimal2: f32 = 2.5;
	println!("Decimal2 = {}, tamanho = {} bytes", decimal2, std::mem::size_of_val(&decimal2));
}
