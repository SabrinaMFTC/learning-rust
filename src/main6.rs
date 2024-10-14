// Tanto constantes quanto variáveis globais, precisam ter seus tipos definidos
const PI: f32 = 3.14;
static VARIAVEL_GLOBAL: u8 = 1;	// static indica que se trata de uma variável global

fn main() {
	println!("PI = {}", PI);
	println!("VARIAVEL_GLOBAL = {}", VARIAVEL_GLOBAL);
}
