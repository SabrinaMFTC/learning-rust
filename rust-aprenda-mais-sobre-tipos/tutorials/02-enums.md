# 01 - Valores pré-definidos

Neste capítulo, vamos nos concentrar na criação de tipos.

## Cenário

Imaginemos que queremos criar uma função que verifica se é fim de semana. Chamaremos essa função de `eh_fim_de_semana`, que receberá um parâmetro `dia_da_semana`. O tipo de dado que usaremos para representar um dia da semana pode ser um `string`, mas isso tornaria o código complexo. Portanto, optamos por usar inteiros.

- 0 representa domingo;
- 1 representa segunda-feira;
- 2 representa terça-feira;
- 3 representa quarta-feira;
- 4 representa quinta-feira;
- 5 representa sexta-feira;
- 6 representa sábado.

Assim, `dia_da_semana` será um número unsigned, ou seja, um inteiro de 8 bits (1 byte):

```rust
fn eh_fim_de_semana(dia_da_semana: u8) -> bool {
    // código omitido
}
```

## Implementação da Função

A função usará uma expressão `match` para determinar se é fim de semana:

```rust
fn eh_fim_de_semana(dia_da_semana: u8) -> bool {
    match dia_da_semana { 
        0 | 6 => true, 
        _ => false 
    }
}
```

**Observação:** É importante não adicionar ponto e vírgula ao final da expressão `match`, pois isso impediria que o valor fosse retornado.

## Testando a Função

Agora podemos testar a função `eh_fim_de_semana` na função `main`:

```rust
fn main() {
    println!("É fim de semana? {}", eh_fim_de_semana(0));
}
```

Passando 0 como parâmetro (domingo), o retorno deve ser `true`. Ao compilar e executar, temos o retorno `true`.

Se passarmos 3 (quarta-feira), o retorno será `false`. O código funciona, mas essa abordagem tem problemas. 

## Problemas da Abordagem Atual

O primeiro problema é a falta de clareza. Ao passar um número como 3, precisamos pensar se é quarta-feira. Um número como 6 pode representar tanto sexta-feira quanto sábado, o que não é claro.

Outro problema é que é possível passar um número inválido, como 12. Isso requereria verificações adicionais na função para garantir que `dia_da_semana` esteja entre 0 e 6. Poderíamos lançar um panic ou retornar um `Result` para lidar com erros.

### A Solução: Criando um Tipo Personalizado

Para resolver esses problemas, podemos criar um tipo personalizado. Em Rust, esse tipo pode ser criado usando **enumerações** (ou `enums`), que são ideais para representar um conjunto limitado de valores conhecidos. 

Vamos criar uma enum chamada `DiaDaSemana` logo acima da função `eh_fim_de_semana`:

```rust
enum DiaDaSemana { 
    Domingo,
    Segunda,
    Terca,
    Quarta,
    Quinta,
    Sexta,
    Sabado
}
```

Agora, mudamos o tipo do parâmetro na função `eh_fim_de_semana` para `DiaDaSemana`:

```rust
fn eh_fim_de_semana(dia_da_semana: DiaDaSemana) -> bool {
    match dia_da_semana { 
        DiaDaSemana::Domingo | DiaDaSemana::Sabado => true, 
        _ => false 
    }
}
```

Aqui, a verificação é mais clara, pois estamos utilizando valores específicos da enum.

## Atualizando a Função `main`

Na função `main`, agora passamos um valor do tipo `DiaDaSemana`, como `Quarta`:

```rust
fn main() {
    println!("É fim de semana? {}", eh_fim_de_semana(DiaDaSemana::Quarta));
}
```

Se passarmos `DiaDaSemana::Quarta`, sabemos que não é fim de semana e esperamos um retorno `false`. 

## Conclusão

Dessa forma, conseguimos definir tipos que fazem sentido no nosso domínio, facilitando a escrita do código. Agora, a enum `DiaDaSemana` possui todos os valores possíveis, e podemos criar variáveis desse tipo:

```rust
let dia: DiaDaSemana = DiaDaSemana::Sexta;
```

Podemos utilizar esses tipos personalizados em nosso código. 

No próximo capítulo, vamos explorar como podemos adicionar informações extras dentro de uma enum.

## Questão: Problemas de Abordagem

Neste capítulo, começamos armazenando o dia da semana como um inteiro e falamos sobre alguns problemas dessa abordagem.

Qual das alternativas a seguir representa um problema dessa abordagem? (*Selecione uma alternativa*)

**A.** Falta de significado e validação no tipo.

**B.** Não há problemas que um simples comentário não resolvesse.

**C.** Consumo excessivo de memória.

**Resposta**

**A.** Correta. É fácil passar um valor inválido como parâmetro nesse caso. Não havia uma representação significativa do que é um dia da semana.

**B.** Incorreta. Legibilidade do código não era o único problema.

**C.** Incorreta. Não é esse o foco aqui. Nós poderíamos poupar memória usando `u8`, por exemplo, mas isso não resolveria o problema.

---

# 02 - Informações Extra

Neste capítulo, abordaremos a passagem de valores extras para uma enum. Inicialmente, minimizaremos a enumeração `DiaDaSemana`, pois não será utilizada neste momento. O código apresenta vários warnings, uma vez que, ao criar uma enum com múltiplos tipos, espera-se que todos sejam utilizados.

Criamos todos os dias da semana, mas estamos utilizando apenas alguns, o que não representa um problema. A função `eh_fim_de_semana` também será minimizada.

## Trabalhando com Cores

Agora, iremos implementar uma função chamada `cores()` dentro da função `main`. Essa função será definida abaixo da função `matriz()`, sem retornar nada por enquanto:

```rust
fn cores() {
    
}
```

### Verificando Cores com Match

Desejamos verificar uma cor qualquer utilizando uma expressão `match` no sistema RGB (Red, Green, Blue). Se a cor for do tipo `Color::Red`, exibiremos "vermelho". Para `Color::Green`, exibiremos "verde", e para `Color::Blue", exibiremos "blue". O código será assim:

```rust
fn cores() {
    println!("Cor = {}", match cor {
        Color::Red => "vermelho",
        Color::Green => "verde",
        Color::Blue => "blue"
    });
}
```

### Declaração da Enum e Variável

Agora precisamos declarar a enum `Color` e a variável `cor`. A enum será definida com as três cores mencionadas:

```rust
enum Color {
    Red,
    Green,
    Blue
}
```

Em seguida, a variável `cor` será definida como `Color::Green`:

```rust
fn cores() {
    let cor = Color::Green;

    println!("Cor = {}", match cor {
        Color::Red => "vermelho",
        Color::Green => "verde",
        Color::Blue => "blue"
    });
}
```

Dessa forma, esperamos que a saída seja "Cor = verde". Ao executar, o resultado é "Cor = verde", o que confirma o funcionamento correto até o momento.

### Detalhes do Match

Dois pontos relevantes surgem neste ponto. Primeiramente, não utilizamos o caractere underline no `match`, e o compilador não gerou avisos. Isso se deve ao fato de que o compilador é capaz de verificar se todos os casos possíveis foram cobertos. Se, por exemplo, removêssemos o caso `Color::Blue => "blue"`, o código não compilaria mais, indicando que o caso de `blue` não foi tratado.

Outro aspecto é a possibilidade de passar valores para enums. Suponhamos que desejemos incluir cores adicionais, como uma cor RGB. Assim, podemos definir uma variante `RgbColor()` que aceita valores. O tipo desses valores deve ser um inteiro sem sinal de 8 bits (u8), que varia de 0 a 255, compatível com a representação de cores no sistema RGB:

```rust
enum Color {
    Red,
    Green,
    Blue,
    RgbColor(u8, u8, u8)
}
```

### Utilização de RgbColor

Podemos agora implementar um `match` para lidar com o caso de `RgbColor()`. Já temos três cenários — `Red`, `Green` e `Blue`. Se recebermos um `RgbColor`, trataremos como uma "cor desconhecida":

```rust
fn cores() {
    let cor = Color::Green;
    println!("Cor = {}", match cor {
        Color::Red => "vermelho",
        Color::Green => "verde",
        Color::Blue => "blue",
        Color::RgbColor() => "desconhecida"
    });
}
```

No entanto, precisamos informar os valores em `RgbColor()`. Podemos definir variáveis para representar esses valores:

```rust
fn cores() {
    let cor = Color::Green;
    println!("Cor = {}", match cor {
        Color::Red => "vermelho",
        Color::Green => "verde",
        Color::Blue => "blue",
        Color::RgbColor(red, green, blue) => format!("RGB({}, {}, {})", red, green, blue)
    });
}
```

O `match` se torna mais significativo e poderoso. No caso de um RGB, podemos optar por não processar os valores e simplesmente indicar que a cor é "RGB desconhecido":

```rust
fn cores() {
    let cor = Color::Green;
    println!("Cor = {}", match cor {
        Color::Red => "vermelho",
        Color::Green => "verde",
        Color::Blue => "blue",
        Color::RgbColor(_, _, _) => "RGB desconhecido"
    });
}
```

### Exemplos de Cores

Caso queiramos identificar a cor preta, que corresponde à ausência de valores (0, 0, 0), podemos implementar:

```rust
fn cores() {
    let cor = Color::Green;
    println!("Cor = {}", match cor {
        Color::Red => "vermelho",
        Color::Green => "verde",
        Color::Blue => "blue",
        Color::RgbColor(0, 0, 0) => "preta",
        Color::RgbColor(_, _, _) => "RGB desconhecido"
    });
}
```

Assim, conseguimos identificar a cor preta. A capacidade de utilizar valores mais específicos no `match` permite que tratemos diferentes casos de forma mais robusta.

### Criação de Exemplos

Podemos, por fim, criar diferentes cores na variável `cor`. Por exemplo:

```rust
let cor = Color::RgbColor(255, 255, 255); // Branco
```

A execução deste código resultará em "RGB desconhecido". Ao definir:

```rust
let cor = Color::RgbColor(0, 0, 0); // Preto
```

A saída será "Cor = preta". Além disso, ao definir `let cor = Color::Red`, a saída continua sendo "Cor = vermelho".

### Considerações Finais

Dessa forma, conseguimos trabalhar com valores dentro das enums. O padrão RGB é bem conhecido, onde R é vermelho, G é verde e B é azul. Entretanto, ao considerar outros padrões, como CMYK (Ciano, Magenta, Amarelo e Preto), podemos notar que o código pode se tornar menos legível.

--- 

# 03 - Nomeando Valores

Nesta aula, discutiremos a passagem de valores nomeados para uma enum, focando na estrutura `CymkColor`, que representa o modelo de cores CMYK. Inicialmente, queremos evitar confusões relacionadas ao uso de parênteses em vez de chaves, como utilizado em `RgbColor`.

## Definição da Enum

A enum `Color` será ampliada para incluir a variante `CymkColor`, que contém os componentes nomeados de cor. Os componentes ciano, magenta, amarelo e preto são definidos como inteiros sem sinal de 8 bits (u8), variando de 0 a 255:

```rust
enum Color {
    Red,
    Green,
    Blue,
    RgbColor(u8, u8, u8),
    CymkColor { cyan: u8, magenta: u8, yellow: u8, black: u8 },
}
```

Essa definição permite a criação de um tipo de dados `CymkColor` com valores nomeados.

## Criação de um `CymkColor`

Para instanciar um objeto `CymkColor`, a sintaxe correta envolve o uso de chaves e a definição explícita dos valores:

```rust
let cor = Color::CymkColor { cyan: 100, magenta: 50, yellow: 70, black: 200 };
```

No entanto, ao compilar o código, surgirá um erro devido à ausência de tratamento para a nova variante no `match`. Precisamos ajustar a função `cores()` para incluir esse novo caso.

## Atualização da Função `cores()`

Modificamos a função `cores()` para lidar com a nova variante:

```rust
fn cores() {
    let cor = Color::CymkColor { cyan: 100, magenta: 50, yellow: 70, black: 200 };

    println!("Cor = {}", match cor {
        Color::Red => "vermelho",
        Color::Green => "verde",
        Color::Blue => "blue",
        Color::RgbColor(0, 0, 0) => "preta",
        Color::RgbColor(_, _, _) => "RGB desconhecido",
        Color::CymkColor { cyan: _, magenta: _, yellow: _, black: _ } => "CYMK desconhecido",
    });
}
```

Neste código, `Color::CymkColor { cyan: _, magenta: _, yellow: _, black: _ }` indica uma cor CMYK desconhecida.

## Condições Adicionais

Para a cor preta, que pode ser representada em CMYK como (0, 0, 0, 255), ajustamos o `match`:

```rust
fn cores() {
    let cor = Color::CymkColor { cyan: 100, magenta: 50, yellow: 70, black: 200 };

    println!("Cor = {}", match cor {
        Color::Red => "vermelho",
        Color::Green => "verde",
        Color::Blue => "blue",
        Color::RgbColor(0, 0, 0) | Color::CymkColor { cyan: _, magenta: _, yellow: _, black: 255 } => "preta",
        Color::RgbColor(_, _, _) => "RGB desconhecido",
        Color::CymkColor { cyan: _, magenta: _, yellow: _, black: _ } => "CYMK desconhecido",
    });
}
```

Essa verificação permitirá identificar corretamente a cor preta.

## Ignorando Avisos

Para evitar avisos relacionados ao código não utilizado, podemos usar a anotação `#[allow(dead_code)]` em torno das definições das enums:

```rust
#[allow(dead_code)]
enum Color {
    Red,
    Green,
    Blue,
    RgbColor(u8, u8, u8),
    CymkColor { cyan: u8, magenta: u8, yellow: u8, black: u8 },
}
```

## Considerações Finais

Resumidamente, podemos criar enums limitadas que aceitam valores nomeados ou não. Utilizamos `match` para manipular essas enums de maneira eficiente. Na próxima sessão, abordaremos tipos em Rust, incluindo aqueles que podem parecer complexos, mas que já exploramos.

--- 

# 04 - O que aprendemos?

Nesta aula:

- Aprendemos a criar tipos usando `enum`;
- Vimos como passar valores para um `enum`;
- Conhecemos a sintaxe de `struct` para nomear os parâmetros da `enum`.
