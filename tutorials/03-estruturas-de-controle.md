# 01 - Estruturas Condicionais

Neste capítulo, exploraremos como controlar o fluxo de execução em Rust utilizando estruturas condicionais. Vamos entender como usar `if`, `else`, e outras construções para guiar o comportamento do programa.

## Estrutura Básica do `if`

A forma mais simples de controlar o fluxo em linguagens procedurais é através do `if`. Vamos começar com um exemplo básico:

```rust
fn main() {
    let idade: u8 = 24; // Variável que armazena a idade
    if idade > 18 {
        println!("Pode entrar na balada");
    }
}
```

Aqui, verificamos se a idade é maior que 18 e exibimos uma mensagem correspondente. Ao compilar e executar o código, a mensagem "Pode entrar na balada" será exibida, caso a condição seja verdadeira. Se mudarmos a idade para 17, nada será exibido, pois a condição não será satisfeita.

## Adicionando o `else`

Podemos adicionar um bloco `else` para lidar com o caso em que a condição não é atendida:

```rust
fn main() {
    let idade: u8 = 17;
    if idade > 18 {
        println!("Pode entrar na balada");
    } else {
        println!("Não pode entrar na balada");
    }
}
```

Nesse caso, se a idade for menor ou igual a 18, a mensagem "Não pode entrar na balada" será exibida.

## Utilizando `else if`

Agora, vamos introduzir uma condição adicional, verificando se a entrada na balada é permitida com a autorização de um responsável:

```rust
fn main() {
    let idade: u8 = 17;
    let responsavel_autorizou = true;

    if idade > 18 {
        println!("Pode entrar na balada");
    } else if idade > 16 && responsavel_autorizou {
        println!("Pode entrar com a assinatura do responsável");
    } else {
        println!("Não pode entrar na balada");
    }
}
```

Aqui, utilizamos o operador lógico `&&` para verificar se a idade está entre 16 e 18 anos e se o responsável autorizou a entrada. Caso as duas condições sejam verdadeiras, a mensagem "Pode entrar com a assinatura do responsável" será exibida.

## Uso de Condições em Variáveis

Também podemos atribuir o resultado de uma expressão `if` a uma variável:

```rust
fn main() {
    let idade: u8 = 19;
    let condicao = if idade >= 18 { "maior" } else { "menor" };
    println!("É {} de idade", condicao);
}
```

Aqui, utilizamos o `if` como uma expressão, atribuindo "maior" ou "menor" à variável `condicao` com base na idade.

## Operadores Lógicos

Rust oferece suporte aos operadores lógicos que já conhecemos de outras linguagens:

- `>`: Maior que
- `<`: Menor que
- `>=`: Maior ou igual
- `<=`: Menor ou igual
- `==`: Igual a
- `!=`: Diferente de

Um exemplo de uso prático desses operadores:

```rust
fn main() {
    let idade: u8 = 18;
    let eh_maior = idade >= 18;
    let condicao = if eh_maior { "maior" } else { "menor" };
    println!("É {} de idade", condicao);
}
```

Neste código, verificamos se a idade é maior ou igual a 18. O resultado será armazenado em `eh_maior`, e o `if` definirá se a pessoa é maior ou menor de idade com base nesse valor booleano.

## Resumo

- O `if` em Rust é uma expressão que pode retornar valores.
- Não há necessidade de parênteses ao redor das condições.
- A última expressão em um bloco de `if`, `else if`, ou `else` sem ponto e vírgula define o valor retornado pela expressão.
- Podemos utilizar operadores lógicos (`>`, `<`, `>=`, `<=`, `==`, `!=`) para construir condições mais complexas.

## Organizando o Código

Podemos mover as verificações condicionais para uma função separada, deixando o código mais organizado:

```rust
fn main() {
    condicionais();
}

fn condicionais() {
    let idade: u8 = 18;
    let eh_maior = idade >= 18;
    let condicao = if eh_maior { "maior" } else { "menor" };
    println!("É {} de idade", condicao);
}
```

Esse código demonstra que podemos declarar uma função após ela ser chamada, algo que não é permitido em linguagens como C ou C++. No Rust, a ordem das funções no código não interfere em sua execução.

## Conclusão

Neste capítulo, vimos como utilizar estruturas condicionais (`if`, `else`, `else if`) em Rust para controlar o fluxo de execução de nossos programas. Aprendemos também a usar expressões condicionais e operadores lógicos. No próximo capítulo, exploraremos loops e como realizar repetições em Rust.

## Questão: Utilizando `if`

Quais das seguintes alternativas são verdadeiras sobre `ifs` no Rust? (*Selecione 2 alternativas*)

**A.** Podemos omitir os parênteses da condição.

**B.** Não existe `else if` em Rust.

**C.** `if` é uma expressão, logo, pode ser usado como um valor qualquer.

## Resposta

**A.** Correta. Os parênteses são opcionais.

**B.** Incorreta. Existe sim, assim como praticamente todas as linguagens.

**C.** Correta. É como se o `if` pudesse retornar um valor, assim como funções.

---

# 02 - Estruturas de Repetição `while` e `loop`

Neste capítulo, vamos abordar como implementar loops em Rust. Se você já está familiarizado com outras linguagens procedurais ou imperativas, perceberá que a lógica é bastante semelhante. Vamos começar implementando uma função simples para organizar nosso código.

## Função `repeticoes()`

Criaremos uma função chamada `repeticoes()` que não recebe parâmetros e não retorna valores. O objetivo dessa função é exibir a tabuada de um número multiplicador.

```rust
fn repeticoes() {
    let multiplicador = 5;
    let mut contador: u8 = 0;
    
    while contador < 10 {
        println!("{} * {} = {}", multiplicador, contador, multiplicador * contador);
        contador += 1;
    }
}
```

### Explicação

1. **Multiplicador**: Definimos o valor 5 como multiplicador.
2. **Estrutura `while`**: A condição `while contador < 10` controla a execução do loop. Dentro do loop, exibimos a multiplicação do valor definido pelo multiplicador com o contador.
3. **Incremento**: Após cada iteração, o valor de `contador` é incrementado para evitar um loop infinito.

### Execução

Para compilar e executar o código, usamos os seguintes comandos no terminal:

```bash
rustc main.rs
./main
```

## Loop Infinito com `loop`

Em Rust, também podemos utilizar a estrutura `loop`, que funciona como um `while true` de outras linguagens, criando um loop infinito. Para evitar que o loop execute indefinidamente, utilizamos a instrução `break` para sair da execução.

```rust
fn repeticoes_loop() {
    let multiplicador = 5;
    let mut contador: u8 = 0;
    
    loop {
        if contador == 10 {
            break;
        }
        println!("{} * {} = {}", multiplicador, contador, multiplicador * contador);
        contador += 1;
    }
}
```

### Explicação

1. **Estrutura `loop`**: O loop será executado até que uma condição seja atingida.
2. **Instrução `break`**: Utilizamos `break` quando o contador atinge o valor 10, interrompendo a execução do loop.

## Instrução `continue`

Caso seja necessário pular uma iteração específica, podemos utilizar a instrução `continue`. No exemplo abaixo, não queremos exibir a multiplicação de 5 por 5, então usamos `continue` para pular essa iteração.

```rust
fn repeticoes_continue() {
    let multiplicador = 5;
    let mut contador: u8 = 0;
    
    while contador < 10 {
        if contador == 5 {
            contador += 1;
            continue;
        }
        println!("{} * {} = {}", multiplicador, contador, multiplicador * contador);
        contador += 1;
    }
}
```

### Explicação

1. **Estrutura `while` com `continue`**: A condição `continue` faz com que o loop pule a iteração atual e continue com a próxima.
2. **Verificação do contador**: Quando o contador atinge o valor 5, o loop salta essa iteração.

## Conclusão

Neste tutorial, vimos como utilizar as estruturas `while`, `loop`, e as instruções `break` e `continue` em Rust. Essas funcionalidades são comuns a diversas linguagens de programação imperativas, tornando o aprendizado mais simples para quem já tem experiência em outras linguagens. No próximo tutorial, vamos explorar a estrutura `for` em Rust.

---

# 03 - Estrutura de Repetição `for`

Nesta aula, vamos explorar a estrutura de repetição `for` em Rust. Diferente de outras linguagens, o `for` em Rust funciona de forma semelhante ao `for each`, percorrendo intervalos ou iteradores.

## Estrutura Básica do `for`

Em outras linguagens, um `for` comum pode ser escrito assim:

```rust
for let mut i = 0; i < 10; i++ {
    // código
}
```

No entanto, em Rust, o `for` é utilizado para percorrer intervalos. Abaixo, demonstramos a sintaxe básica:

```rust
for i in 1..11 {
    println!("{}", i);
}
```

### Explicação

- **Intervalo**: A notação `1..11` cria um intervalo de números que vai de 1 a 10. A parte da direita (11) é exclusiva, ou seja, o intervalo termina antes de alcançar esse número.
- **Sintaxe alternativa**: Também podemos usar `1..=10` para tornar o intervalo inclusivo, ou seja, para incluir o número 10.

Exemplo:

```rust
for i in 1..=10 {
    println!("{} * {} = {}", 5, i, 5 * i);
}
```

Neste exemplo, percorremos o intervalo de 1 a 10 e exibimos a tabuada de 5.

## Benefícios do `for`

Uma das vantagens do `for` em Rust é que ele gerencia automaticamente o contador e o fluxo de controle. Não há necessidade de incrementar variáveis ou adicionar condições de parada manualmente, como acontece com estruturas `while` ou `loop`.

## Considerações Finais

Com o `for`, vimos como percorrer intervalos de forma simples e eficiente. Já aprendemos a utilizar as estruturas `while`, `loop`, e agora o `for`. No próximo tutorial, exploraremos outras estruturas de controle de fluxo em Rust.

---

# 04 - `Match` Statement

Neste capítulo, vamos explorar uma construção que nem todas as linguagens de programação possuem: a **Match Expression**. Linguagens como PHP possuem algo semelhante ao que veremos, mas com algumas diferenças. A **Match Expression** é uma forma avançada de estrutura condicional em Rust, funcionando de maneira similar ao `switch` em outras linguagens, porém com mais flexibilidade.

## Estrutura Básica da Match Expression

Vamos começar com um exemplo prático. Suponha que temos uma variável `linguagem` e queremos atribuir um valor a `proposito` com base no valor de `linguagem`. Podemos usar a **Match Expression** para isso:

```rust
let linguagem = "PHP";
let proposito = match linguagem {
    "PHP" => "Web",
    "Kotlin" => "Android",
    "Python" => "Data Science",
    _ => "Desconhecido",
};
println!("O propósito de {} é {}", linguagem, proposito);
```

### Explicação

1. **Sintaxe**: A sintaxe de `match` testa o valor de uma variável, neste caso, `linguagem`, e associa a ela um resultado baseado nos casos fornecidos.
2. **Padrões**: Cada braço do `match` é composto por um padrão, como `"PHP"`, seguido da setinha `=>` e do valor a ser retornado, como `"Web"`.
3. **Wildcard (`_`)**: O `_` serve como um "coringa" (wildcard), equivalente ao `default` em outras linguagens, cobrindo todos os casos não especificados. 
4. **Expressão**: Diferentemente de um `switch`, o `match` em Rust é uma expressão, o que significa que ele sempre retorna um valor.

### Comportamento Padrão

Caso algum valor de `linguagem` não seja coberto pelos casos especificados, o compilador emitirá um aviso. Para evitar esse problema, é necessário incluir o coringa `_` para lidar com valores inesperados.

### Exemplo de Execução

Se compilarmos o código com diferentes valores de `linguagem`, teremos resultados como:

```rust
let linguagem = "PHP";      // Saída: "O propósito de PHP é Web"
let linguagem = "Kotlin";   // Saída: "O propósito de Kotlin é Android"
let linguagem = "C#";       // Saída: "O propósito de C# é Desconhecido"
```

## Considerações Finais

A **Match Expression** é uma poderosa ferramenta para o controle de fluxo em Rust, especialmente quando se trata de trabalhar com múltiplos valores possíveis. Diferente de outras linguagens, ela não requer um `break` para evitar a continuação indesejada do código, e sempre garante que um valor será retornado.

No próximo capítulo, exploraremos mais estruturas específicas de Rust, incluindo o **Pattern Matching**, que está relacionado à utilização do `match` em situações mais complexas.

---

# 05 - O que aprendemos?

Nesta aula, aprendemos:

- A sintaxe da estrutura `if` em Rust;
- Que o `if` em Rust é uma expressão;
- Como trabalhar com loops (`while`, `loop`, `for`) em Rust;
- A utilização de intervalos (ranges) no `for` do Rust;
- A estrutura de controle `match` (Match Statement) em Rust.
