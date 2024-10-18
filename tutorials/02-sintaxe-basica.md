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

Neste tutorial, vamos explorar alguns conceitos fundamentais da linguagem Rust, como **escopo** e a peculiaridade de **redeclaração de variáveis**, com foco no comportamento de **shadowing**.

## Escopo em Rust

Em Rust, o conceito de **escopo** está diretamente relacionado a blocos de código delimitados por chaves `{}`. Sempre que um bloco é aberto e fechado, criamos um novo escopo. Variáveis declaradas dentro de um escopo só podem ser acessadas dentro desse mesmo escopo.

Por exemplo:

```rust
fn escopo() {
    let decimal = 10.5;
    // Variável 'decimal' acessível apenas dentro deste bloco
}

fn main() {
    escopo();
    // Acessar 'decimal' aqui causará um erro de compilação
}
```

Ao tentar acessar uma variável fora do seu escopo, o compilador retornará um erro informando que a variável não está disponível naquele contexto. Esse comportamento é comum em diversas linguagens de programação.

## Redeclaração de Variáveis

Uma característica interessante do Rust é a possibilidade de redeclarar variáveis, algo não muito comum em linguagens com **tipagem estática**. Veja o exemplo a seguir:

```rust
fn main() {
    let x = 100;
    let x = 200; // Redeclaração de 'x'
    println!("x = {}", x); // Exibe: x = 200
}
```

Embora o Rust permita essa prática, o compilador emitirá um aviso caso a primeira variável não seja utilizada. No entanto, o código será compilado sem erros.

Essa prática de redeclaração não transforma a variável em **mutável**. O Rust está simplesmente alocando um novo espaço de memória para a nova declaração, sem utilizar o mesmo local de memória da primeira variável.

## Shadowing (Sombras)

O **shadowing** é um comportamento particular do Rust, onde é possível redeclarar uma variável dentro de um novo escopo, "encobrindo" a variável de mesmo nome existente no escopo externo. Veja o exemplo:

```rust
fn sombra() {
    let a = 123;
    {
        let a = 777; // Variável 'a' sombreando a anterior
        println!("Dentro do escopo: a = {}", a); // Exibe: a = 777
    }
    println!("Fora do escopo: a = {}", a); // Exibe: a = 123
}
```

Nesse caso, a variável `a` dentro do bloco interno não é a mesma variável `a` declarada no escopo externo. Quando o bloco interno termina, a variável `a` definida dentro dele deixa de existir, e a variável `a` do escopo externo continua inalterada.

Esse comportamento pode ser útil, mas também requer atenção para evitar confusão e erros inesperados. O **shadowing** pode ser utilizado para criar variáveis temporárias sem modificar as variáveis do escopo externo.

## Strings em Rust

Strings em Rust são um tipo mais complexo do que simples vetores de caracteres. Quando declaramos uma string como `let minha_string = "exemplo";`, estamos lidando com uma referência para um tipo chamado **`&'static str`**. Esse tipo de string é alocado em uma área estática do programa e não pode ser manipulada como um vetor.

Por exemplo, não podemos acessar diretamente posições de uma string como em outras linguagens. Isso ocorre porque `&str` é uma fatia de uma string, ou seja, uma referência para uma sequência de caracteres. Entraremos em mais detalhes sobre strings e referências em tutoriais futuros.

## Conclusão

Neste tutorial, abordamos:

1. O conceito de **escopo** em Rust e como variáveis são limitadas ao escopo onde foram criadas.
2. A possibilidade de **redeclaração de variáveis** em Rust, sem transformá-las em mutáveis.
3. O comportamento de **shadowing**, onde uma variável de mesmo nome pode "sombrar" uma variável externa dentro de um novo escopo.

Esses conceitos são fundamentais para entender como o Rust lida com memória e organização de código. É importante nomear as variáveis com cuidado para evitar comportamentos inesperados, especialmente ao lidar com **shadowing**.

No próximo tutorial, exploraremos outros conceitos mais avançados, incluindo a manipulação de strings em Rust.

---

# 04 - Funções

Até o momento, utilizamos algumas funções sem discutir seus detalhes. Essas funções não recebem argumentos nem possuem retorno. Vamos entender como lidar com funções que recebem parâmetros e retornam valores.

## Funções sem Retorno

Se uma função não declara um retorno explícito, significa que ela não retorna nenhum valor. Ao contrário das variáveis, não podemos omitir o tipo de retorno. Para exemplificar, criaremos uma função simples de soma que recebe dois valores e retorna a soma.

```rust
fn soma(a: i32, b: i32) -> i32 {
    println!("{} + {} = {}", a, b, a + b);
    a + b // Sem ponto e vírgula para indicar retorno
}
```

Nesse exemplo, a função `soma` recebe dois inteiros de 32 bits e retorna um inteiro de 32 bits. O operador `->` indica o tipo de retorno. Além disso, a última expressão da função (sem ponto e vírgula) é o valor retornado.

### Uso de Operadores Matemáticos

Rust suporta os operadores matemáticos básicos, como soma (`+`), subtração (`-`), multiplicação (`*`), e divisão (`/`). Para operações mais avançadas, como potência ou raiz quadrada, é possível utilizar funções específicas da biblioteca padrão.

## Retornando Valores de Funções

Em Rust, praticamente tudo é uma expressão que pode resultar em um valor. Isso inclui funções, que podem ser usadas em outros contextos como expressões. No exemplo abaixo, usamos a função `soma` dentro de um `println!`:

```rust
fn main() {
    println!("Soma = {}", soma(2, 2));
}
```

Ao executar esse código, o valor 4 será exibido. Importante destacar que, para retornar o valor de uma função, a última expressão não deve ter ponto e vírgula. O ponto e vírgula em Rust não é opcional; ele possui um significado: ignorar o resultado da expressão.

### Early Return

Se for necessário retornar antecipadamente dentro de uma função, o Rust permite o uso da palavra-chave `return`. No entanto, essa abordagem é mais comumente usada em estruturas como `if` e loops.

## Recapitulando

- Funções podem receber parâmetros, e é obrigatório especificar o tipo de cada parâmetro.
- O tipo de retorno é indicado após o símbolo `->`.
- Para retornar um valor, a última expressão da função não deve ter ponto e vírgula.
- O uso do `return` é possível, mas em muitos casos a última expressão da função já é suficiente para o retorno.

## Próximos Passos

Agora que compreendemos o básico sobre funções e variáveis, é hora de avançar para o controle de fluxo em Rust. No próximo capítulo, exploraremos como utilizar `if`, `loop` e outras estruturas para controlar a execução do programa.

## Questão: Definindo funções

Qual das alternativas a seguir é verdadeira sobre funções em Rust? (*Selecione uma alternativa*)

**A.** O tipo de retorno da função é opcional.

**B.** A última expressão sempre é o retorno da função.

**C.** Para que uma função retorne algo, precisamos especificar o tipo de retorno.

**Resposta**

**A.** Incorreta. Se não definirmos o retorno, a função não poderá retornar um valor.

**B.** Incorreta. Precisamos omitir o `;` para que a expressão seja usada como retorno.

**C.** Correta. Se não definirmos o retorno, a função não poderá retornar um valor. Além disso, a última expressão não deve conter `;` para que seu valor seja usado como retorno.

---

# 05 - O que aprendemos?

Nesta aula, aprendemos:

- Os **tipos primitivos** de Rust;
- Como declarar **variáveis**;
- Sobre **escopo** e **shadowning** em Rust;
- Como declarar **funções** em Rust.
