fn main() {
	let booleana = false;
	println!("Tamanho booleana = {} byte", std::mem::size_of_val(&booleana));

	// booleana = true; --> erro: por padrão, variáveis em rust são imutáveis
	// Para que o valor da variável seja mutável, é necessário acrescentar "mut" antes do nome da variável

	let mut booleana_variavel = true;
	println!("Tamanho booleana_variável (true) = {} byte", std::mem::size_of_val(&booleana_variavel));

	booleana_variavel = false;
	println!("Tamanho booleana_variável (false) = {} byte", std::mem::size_of_val(&booleana_variavel));
}
