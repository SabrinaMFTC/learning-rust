# 01 - Apresentação

Neste treinamento, abordaremos os diversos tipos disponíveis na linguagem Rust, expandindo o conhecimento adquirido em treinamentos anteriores, que incluíram sintaxe básica, *pattern matching* e tipos primitivos como inteiros, números de ponto flutuante, booleanos e caracteres. O objetivo deste curso é aprofundar-se em tipos mais complexos.

## Conjuntos de Dados

Iniciaremos com o estudo de conjuntos de dados, que nos permitirão alocar múltiplas variáveis do mesmo tipo. Por exemplo, para armazenar notas de provas em uma única variável, utilizaremos **arrays**.

## Criação de Tipos Personalizados

Em seguida, exploraremos a criação de tipos personalizados por meio de **enums**. Com enums, poderemos definir tipos com valores delimitados e conhecidos. Criaremos uma enum representando os dias da semana e outra para cores, aprendendo a passar valores para estas enums.

Abordaremos duas maneiras de passar valores para enums: por meio de **tuplas**, onde informamos apenas os tipos, e através de **structs**, onde especificamos um nome e um valor.

## Enum Option

Discutiremos a enum **Option**, que é similar à enum **Result** estudada anteriormente. Neste contexto, compreenderemos a utilidade desse tipo de enum.

## Vetores

Avançaremos para uma estrutura chamada **Vec**, que é um array dinâmico. Com os vetores, não precisamos conhecer o tamanho prévio, permitindo a alocação conforme necessário. Abordaremos também questões de performance e estratégias para evitar realocações de memória.

## Agrupamento de Dados

Após a compreensão dos tópicos anteriores, aprenderemos a agrupar dados que fazem sentido juntos. Por exemplo, ao representar uma conta-corrente, como agruparemos o titular e o saldo? Para isso, utilizaremos **structs** e aprenderemos a criar métodos associados a estas estruturas.

---

# 02 - Múltiplos Valores

Neste capítulo, revisaremos o projeto criado anteriormente utilizando o Cargo, que atualmente contém uma função `main` que exibe um simples "Hello World". Este será o ponto de partida para nosso desenvolvimento.

## Cenário Prático

Vamos considerar o desenvolvimento de um sistema acadêmico que necessite armazenar as notas de um aluno ao longo do ano letivo, correspondente aos quatro bimestres do ensino no Brasil. Inicialmente, precisaríamos criar quatro variáveis distintas para armazenar essas notas. Por exemplo, poderíamos declarar as variáveis da seguinte forma:

```rust
let nota1: u8 = 10;
let nota2: u8 = 8;
let nota3: f32 = 9.5;
let nota4: f32 = 6.0;
```

Percebe-se que, ao usar números flutuantes, a nota 3 não pode ser atribuída a uma variável do tipo `u8`. Para manter a consistência do tipo entre as variáveis, todas deveriam ser do tipo `f32`, o que implica em uma reescrita manual de todas as definições.

## Casting de Tipos

Podemos realizar um *cast* para garantir que os valores sejam armazenados corretamente. Por exemplo:

```rust
let nota1: f32 = 10f32;
let nota2: f32 = 8f32;
let nota3: f32 = 9.5;
let nota4: f32 = 6f32;
```

Essa abordagem, embora funcione, apresenta limitações, como a necessidade de manipulação manual das variáveis. 

## Introdução aos Arrays

Para solucionar esses problemas, podemos utilizar **arrays**. Os arrays nos permitem armazenar um conjunto de dados relacionados em uma única variável. Por exemplo, podemos declarar um array para as notas da seguinte forma:

```rust
let notas = [10f32, 8f32, 9.5, 6.0];
```

Isso nos permite armazenar todas as notas em uma única variável e elimina a necessidade de múltiplas declarações.

### Acesso aos Elementos

Para acessar os elementos do array, utilizamos a notação de índice, iniciando do zero:

```rust
println!(
    "Nota1 = {}, Nota2 = {}, Nota3 = {}, Nota4 = {}", 
    notas[0], 
    notas[1], 
    notas[2], 
    notas[3]
);
```

### Definição de Tipo

Ao declarar o array, o Rust infere automaticamente o tipo e o tamanho. Podemos verificar que `notas` é do tipo `[f32; 4]`, ou seja, um array de quatro valores do tipo `f32`. Para deixar isso explícito, podemos declarar o tipo da seguinte forma:

```rust
let notas: [f32; 4] = [10f32, 8f32, 9.5, 6.0];
```

Caso não especifiquemos o tipo, o Rust pode assumir um tipo diferente. Por exemplo:

```rust
let notas = [10.0, 8.0, 9.5, 6.0];
```

Neste caso, o Rust interpretaria como `[f64; 4]`, pois não tem certeza sobre os tamanhos dos números.

### Conclusão

Os arrays em Rust oferecem uma forma eficiente de agrupar dados relacionados, garantindo que o tamanho e o tipo sejam consistentes. No próximo vídeo, abordaremos como manipular e trabalhar com esses arrays.

## Questão: Tamanho

Vimos nesta aula como podemos declarar e inicializar um array de determinado tipo. Quanto espaço na memória ocuparia um array do tipo [u8; 5]? (*Selecione uma alternativa*)

**A.** 8 bytes

**B.** 5 bytes

**C.** 40 bytes

**Resposta**

**A.** Incorreta. Não usamos 8 bytes para armazenar este vetor.

**B.** Correta. Cada `u8` ocupa 1 byte (8 bits). Se temos 5 valores do tipo `u8`, temos 40 bits, ou 5 bytes, ocupados.

**C.** Incorreta. Nós utilizamos 40 bits, não bytes.

---

# 03 - Manipulando Arrays

## Introdução

Nesta aula, veremos algumas formas de manipular arrays e explorá-los. Vamos começar aprendendo como percorrer um array de forma dinâmica, em vez de acessar cada item individualmente.

## Usando o Loop `for`

Podemos remover o `println!()` e utilizar um loop `for`, que é ideal para percorrer elementos dentro de um conjunto, como um iterador. Os arrays em Rust já possuem um iterador embutido, o que torna essa tarefa simples.

```rust
fn main() {
    let notas: [f32; 4] = [10.0, 8.0, 9.5, 6.0];

    for nota in notas {
        println!("A nota é = {}", nota);
    }
}
```

Ao executar o código, veremos as notas sendo exibidas: 10, 8, 9.5, 6.

## Percorrendo Índices

Outra forma de percorrer um array é iterar pelos índices. Podemos usar um loop que vai de 0 até 3, que corresponde aos quatro itens do array.

```rust
fn main() {
    let notas: [f32; 4] = [10.0, 8.0, 9.5, 6.0];

    for indice in 0..notas.len() {
        println!("A nota {} é = {}", indice + 1, notas[indice]);
    }
}
```

Utilizamos `notas.len()` para obter o tamanho do array, o que proporciona maior flexibilidade se decidirmos alterar o array no futuro.

## Inicializando Arrays

Se quisermos criar um array grande e inicializá-lo com o mesmo valor, podemos usar uma sintaxe específica:

```rust
let notas: [f32; 4] = [6.5; 4];
```

Isso cria um array de quatro elementos, todos com o valor 6.5.

## Arrays Multidimensionais

Agora, vamos explorar como criar arrays multidimensionais, ou matrizes. Podemos definir uma matriz como um array de arrays:

```rust
fn matriz() {
    let matriz = [
        [0.0, 1.2, 0.1],
        [1.3, 0.3, 1.4]
    ];
}
```

Para percorrer essa matriz, utilizamos um loop aninhado:

```rust
fn matriz() {
    let matriz = [
        [0.0, 1.2, 0.1],
        [1.3, 0.3, 1.4]
    ];

    for linha in matriz {
        for item in linha {
            println!("Item = {}", item);
        }
    }
}
```

## Tipos de Dados em Matrizes

O tipo da matriz é `[[f64; 3]; 2]`, onde temos duas linhas, e cada linha contém três elementos. Se quisermos otimizar a memória, podemos trocar `f64` por `f32`:

```rust
let matriz: [[f32; 3]; 2] = [
    [0.0, 1.2, 0.1],
    [1.3, 0.3, 1.4]
];
```

## Acesso a Elementos de Arrays

Por fim, vamos explorar como acessar elementos de um array usando uma variável de índice:

```rust
fn main() {
    let notas: [f32; 4] = [10.0, 8.0, 9.5, 6.0];
    let inteiro: i64 = 0;

    println!("{}", notas[inteiro]);

    for indice in 0..notas.len() {
        println!("A nota {} é = {}", indice + 1, notas[indice]);
    }
}
```

Ao executar este código, uma mensagem de erro pode surgir: "*slice indices are of type usize*". Vamos explorar mais sobre essa mensagem de erro na próxima aula.

## Questão: Definição do Tipo

Como seria a definição de tipo de uma matriz de 2 linhas e 3 colunas de inteiros (`u8`)? (*Selecione uma alternativa*)

**A.** [[u8; 3]; 2]

**B.** [[u8; 2; 3]]

**C.** [[u8; 2]; 3]

**Resposta**

**A.** Correta. Com essa definição, temos um array de 2 linhas e 3 colunas de `u8`.

**B.** Incorreta. Essa sintaxe é inválida.

**C.** Incorreta. Esse array possui 3 linhas e 2 colunas.

---

# 04 - *usize* e *isize*

Vamos tentar entender o que significa o erro: "*Slice indices are of type `usize` or ranges of `usize`*". Para simplificar, vamos substituir a palavra "slice" por "array".

## O Erro

O que a mensagem está dizendo? Estou tentando acessar um índice usando um inteiro de 64 bits. No entanto, o tipo esperado é um `usize`. Vamos consultar a [documentação do Rust](https://doc.rust-lang.org/rust-by-example/primitives.html) para entender melhor o que é o `usize`.

## O Tipo `usize`

Na lista de tipos primitivos do Rust, encontramos o `isize` e o `usize`. O `usize` é definido como um "tipo de inteiro sem sinal do tamanho de um ponteiro". Essa frase pode parecer confusa à primeira vista, mas, ao analisarmos a descrição, fica mais claro. O tamanho desse tipo primitivo é o espaço em bytes alocado na memória.

- Em sistemas de 32 bits, alocamos memória em blocos de 4 bytes.
- Em sistemas de 64 bits, como o que estou usando, alocamos em blocos de 8 bytes.

Basicamente, o `usize` tem tamanho suficiente para armazenar um endereço de memória, que em um sistema de 64 bits é um inteiro de 8 bytes.

## Uso Correto do Tipo

Embora eu esteja usando o tipo correto para a minha máquina, em um sistema de 32 bits, eu precisaria utilizar um valor diferente. Para garantir que não cometamos erros, devemos utilizar o `usize`:

```rust
let inteiro: usize = 0;
```

O `isize`, que ocupa o mesmo espaço mas permite valores negativos, não é utilizado para arrays, pois os índices não podem ser negativos.

## Funcionamento do Código

Vamos executar o código novamente para verificar se há erros. Nenhum erro aparece.

Agora, nosso código funciona sem problemas. O que entendemos até aqui? Existe um tipo específico em Rust que ocupa o espaço necessário para armazenar um ponteiro. Um sistema de 32 bits pode gerenciar cerca de 4 GB de memória, enquanto um sistema de 64 bits pode armazenar muito mais endereços.

## Endereços de Memória

Para armazenar o último endereço de memória em um sistema de 64 bits, precisamos de um número inteiro grande, ocupando 64 bits (ou 8 bytes). Portanto, em sistemas de 64 bits, a variável seria do tipo `u64`, enquanto em sistemas de 32 bits, seria `u32`. O tamanho pode variar em diferentes arquiteturas.

O compilador nos obriga a usar um valor "dinâmico". Um compilador em um sistema de 64 bits utilizará `usize` como um inteiro de 64 bits, enquanto um compilador em um sistema de 32 bits usará um inteiro de 32 bits.

## Conclusão

Precisamos de um tipo maior para representar endereços de memória em sistemas de 64 bits do que em sistemas de 32 bits. Para simplificar o acesso aos índices de arrays, utilizamos `usize`, como em:

```rust
let inteiro: usize = 0;
```

Se omitir o `usize`, será que o compilador consegue entender?

Ele consegue! Quando usamos esse inteiro como índice, o compilador automaticamente o considera como `usize`. No entanto, se tentarmos forçá-lo a usar `u64`, ele não conseguirá ajudar.

Recapitulando: para acessar endereços de memória, como índices de um array, usamos o tipo `usize`.

Agora que compreendemos essa parte sobre arrays, vamos mudar o foco e falar sobre tipos de dados, incluindo como podemos criar um tipo personalizado. Este será o tema do próximo capítulo.

## Questão: Propósito

Os tipos `usize` e `isize` possuem o tamanho necessário para armazenar um endereço de memória, ou seja, 4 bytes em sistemas 32 bits e 8 bytes em sistemas 64 bits.

Qual o propósito de se ter um tipo que possui o tamanho necessário para armazenar um endereço de memória? (*Selecione uma alternativa*)

**A.** Não há motivo real para se usar estes tipos.

**B.** Lidar com índices e ponteiros.

**C.** Ter um outro nome para inteiros.

**Resposta**

**A.** Incorreta. Há sim motivo para estes tipos existirem e serem usados.

**B.** Correta. Para garantir que conseguimos representar todos os índices possíveis de um array em memória, por exemplo, e até para lidarmos com ponteiros, precisamos usar estes tipos.

**C.** Incorreta. Não é só sobre ter um “apelido”.

---

# 05 - O que aprendemos?

Nesta aula:

- Conhecemos **arrays** em Rust;
- Vimos como definir arrays multidimensionais (**matrizes**);
- Aprendemos sobre 2 novos tipos: `usize` e `isize`.
