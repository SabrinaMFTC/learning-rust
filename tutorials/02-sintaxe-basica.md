# 01 - Tipos e Variáveis

Nesta aula, abordaremos os seguintes tópicos:
- O que é Rust e suas aplicações.
- Configuração do ambiente de desenvolvimento Rust.
- Compilação do primeiro código utilizando `rustc`.

## Declaração de Variáveis

Para declarar uma variável em Rust, utilizamos a palavra-chave `let`. A sintaxe básica é a seguinte:

```rust
let nome_da_variavel: tipo = valor;
```

### Exemplo 1: Declaração de um Inteiro

Vamos declarar uma variável do tipo inteiro de 8 bits (`i8`), atribuindo a ela o valor de `128`.

```rust
let variavel: i8 = 128;
println!("variavel = {}", variavel);
```

### Erros de Compilação

Se tentarmos atribuir um valor fora do intervalo permitido para `i8`, que é de `-128` a `127`, o compilador apresentará um erro. Por exemplo:

```rust
let variavel: i8 = 128; // Isso causará um erro de compilação
```

A mensagem de erro indicará que o valor literal está fora do intervalo para o tipo `i8`.

### Tipos Unsigned

Podemos utilizar um tipo unsigned, como `u8`, que permite valores de `0` a `255`. A declaração seria:

```rust
let variavel: u8 = 128; // Agora, isso está correto
```

## Tipos Primários em Rust

Rust oferece diferentes tipos primários de inteiros, que podem ser:
- `i8`, `i16`, `i32`, `i64` (inteiros com sinal).
- `u8`, `u16`, `u32`, `u64` (inteiros sem sinal).

A [documentação oficial do Rust](https://doc.rust-lang.org/rust-by-example/primitives.html) fornece uma lista abrangente desses tipos.

## Inferência de Tipos

O Rust possui inferência de tipos, ou seja, se não especificarmos o tipo, o compilador determinará o tipo padrão. Por exemplo:

```rust
let variavel = 128; // O tipo será inferido como i32
```

Podemos obter o tamanho de uma variável usando a função `std::mem::size_of_val()`:

```rust
println!("tamanho = {} bytes", std::mem::size_of_val(&variavel));
```

## Variáveis de Ponto Flutuante

Além de inteiros, Rust também suporta variáveis de ponto flutuante. O tipo `f64` representa um número decimal de precisão dupla.

### Exemplo 2: Declaração de um Número Decimal

```rust
let decimal: f64 = 2.5;
println!("decimal = {}", decimal);
```

Rust também suporta o tipo `f32`, que é um número decimal de precisão simples.

## Tipos Booleanos e Caracteres

Rust também possui tipos booleanos, que podem assumir os valores `true` ou `false`, e caracteres (`char`), que representam letras. Para medir o espaço ocupado por uma variável booleana, podemos usar o seguinte código:

```rust
let booleana: bool = false;
println!("Tamanho booleana = {}", std::mem::size_of_val(&booleana));
```

O tamanho de uma variável booleana em Rust é de 1 byte, assim como um tipo `u8`. Em Rust, as variáveis são imutáveis por padrão, o que significa que seu valor não pode ser alterado após a atribuição inicial. Para criar uma variável mutável, utilizamos a palavra-chave `mut`:

```rust
let mut booleana: bool = false;
booleana = true; // Agora, o valor pode ser alterado
```

### Exemplo de Caracteres

Para declarar uma variável do tipo caractere, usamos a seguinte sintaxe:

```rust
let letra: char = 'c';
println!("Tamanho do char = {} byte", std::mem::size_of_val(&letra));
```

Embora inicialmente se espere que um caractere ocupe 1 byte, em Rust, um `char` ocupa 4 bytes, pois utiliza UTF-8 para representar caracteres Unicode.

## Conclusão

Nesta aula, aprendemos sobre a declaração de variáveis em Rust, incluindo tipos inteiros, booleanos e caracteres. Também abordamos a imutabilidade das variáveis e como usar a palavra-chave `mut` para torná-las mutáveis. Na próxima aula, discutiremos a diferença entre variáveis imutáveis e constantes, e suas aplicações em Rust.

## Questão: Variáveis

Existe uma diferença interessante entre as variáveis de outras linguagens para as variáveis em Rust. Qual é a particularidade de variáveis em Rust? (*Selecione uma alternativa*)

**A.** Variáveis em Rust são mutáveis por padrão.

**B.** Variáveis em Rust precisam ter seu tipo definido.

**C.** Variáveis em Rust são imutáveis por padrão.

**Resposta**

**A.** Incorreta. Em Rust, todas as variáveis são imutáveis por padrão. Para poder alterar seus valores, precisamos declará-las com `let mut`.

**B.** Incorreta. Além disso não ser uma particularidade de Rust (toda linguagem compilada exige a informação de tipo), é possível omitir o tipo e deixar o compilador inferir.

**C.** Correta. Em Rust, todas as variáveis são imutáveis por padrão. Para poder alterar seus valores, precisamos declará-las com `let mut`.

---

# 02 - Constantes

Nesta aula, abordaremos a diferença entre variáveis imutáveis e constantes em Rust, bem como a utilização de variáveis globais e variáveis estáticas.

## Diferença entre Variáveis Imutáveis e Constantes

A principal diferença entre uma variável imutável e uma constante é que, mesmo que uma variável seja imutável, ela ainda possui um endereço de memória alocado. Por exemplo, quando declaramos uma variável imutável, Rust aloca espaço na pilha de execução (stack frame) para armazenar o valor. Aqui está um exemplo:

```rust
let numero: i32 = 300;
```

Neste caso, Rust aloca 32 bits de memória na pilha e armazena o valor `300`.

### Constantes

Por outro lado, as constantes são valores fixos que não precisam de um endereço de memória para serem utilizados. Por exemplo, podemos criar uma constante para o valor de Pi:

```rust
const PI: f32 = 3.14;
```

Ao utilizar uma constante, o compilador substitui todas as ocorrências de `PI` pelo valor `3.14` em tempo de compilação, em vez de armazenar esse valor em memória. Portanto, não há alocação de memória para constantes, ao contrário do que acontece com variáveis.

### Exemplo de Uso de Constantes

Podemos utilizar a constante em uma função `println!` da seguinte forma:

```rust
println!("PI = {}", PI);
```

Isso resultará na saída `PI = 3.14`. O compilador substituirá `PI` pelo valor diretamente.

## Variáveis Globais

Rust permite a definição de variáveis globais, que podem ser acessadas em qualquer função dentro do arquivo. Para declarar uma variável global, utilizamos a palavra-chave `static`. É importante notar que, ao declarar uma variável estática, precisamos especificar seu tipo:

```rust
static VARIAVEL_GLOBAL: u8 = 1;
```

Podemos exibir o valor da variável global da seguinte maneira:

```rust
println!("variavel_global = {}", VARIAVEL_GLOBAL);
```

## Comparação entre `const` e `static`

Tanto `const` quanto `static` exigem que você defina um tipo e sejam inicializados com valores constantes. No entanto, as principais diferenças são:

- `const` não possui um endereço de memória associado e é substituída em tempo de compilação.
- `static` tem um endereço de memória associado e pode ser acessada globalmente.

## Variáveis Estáticas Mutáveis

As variáveis estáticas podem ser mutáveis, mas isso é considerado inseguro em Rust. Para declarar uma variável estática mutável, devemos usar a palavra-chave `static mut`. Por exemplo:

```rust
static mut CONTADOR: u32 = 0;
```

Se quisermos modificar o valor de `CONTADOR`, teremos que marcar o bloco onde fazemos isso como `unsafe`:

```rust
unsafe {
    CONTADOR += 1;
}
```

Isso indica que estamos cientes dos riscos associados à manipulação de uma variável global mutável.

## Conclusão

Nesta aula, discutimos as diferenças entre variáveis imutáveis e constantes, o uso de variáveis globais e a declaração de variáveis estáticas. Também abordamos a mutabilidade em variáveis estáticas e a necessidade de utilizar `unsafe` ao manipulá-las. Na próxima aula, exploraremos mais especificidades do Rust.

## Questão: Constantes vs Variáveis

Vimos que em Rust as variáveis são imutáveis por padrão e também aprendemos sobre constantes. Qual a diferença entre uma constante e uma variável (imutável)? (*Selecione uma alternativa*)

**A.** As variáveis não possuem um endereço de memória.

**B.** As constantes não possuem um endereço de memória.

**C.** Não há diferenças. São sinônimos.

**Resposta**

**A.** Incorreta. É exatamente o contrário.

**B.** Correta. Constantes são *inlined* em tempo de compilação, ou seja, o compilador substitui a referência para a constante pelo seu valor diretamente.

**C.** Incorreta. Existe uma importante diferença entre ambas.

---

# 03 - Recursos Específicos

