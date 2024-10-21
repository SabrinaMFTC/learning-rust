# 01 - Ownership

Neste capítulo, discutimos um dos conceitos mais importantes do Rust: o ***ownership***, ou seja, como a linguagem gerencia a memória de forma única, sem o uso de Garbage Collector, existente em linguagens como Java, ou alocação manual, comum em linguagens como C e C++.

## Funcionamento

- Rust utiliza ***ownership*** para gerenciar a memória de forma eficiente. Cada valor na heap tem um único dono (*owner*).
- Quando uma variável é passada para uma função, o ***ownership*** do valor na heap é transferido para a função, invalidando a variável original.
- Ao sair de escopo, o valor é liberado da heap automaticamente, eliminando a necessidade de um Garbage Collector.
- Strings são alocadas na heap por serem dinâmicas e potencialmente grandes, ao contrário de valores menores, como inteiros, que são armazenados na stack.

## Comparação com outras linguagens

  - Em C++: uso de ponteiros e liberação manual de memória.
  - Em Java/C#/PHP: uso de Garbage Collector para liberar memória.

## Vantagens do modelo de gestão de memória do Rust

  - Evita overhead de um Garbage Collector.
  - Garante que a memória seja liberada imediatamente quando o valor sai de escopo.

Na próxima aula, exploraremos o conceito de ***borrowing*** (empréstimo de referências) e como podemos passar variáveis por parâmetro sem transferir o *ownership*, permitindo que elas continuem a ser usadas posteriormente.

---

# 02 - Referências e Borrowing

Neste capítulo, abordaremos um problema relacionado ao conceito de *ownership* (propriedade) em Rust.

No Rust, todo valor alocado na heap — incluindo estruturas de dados complexas — possui um único proprietário, ou seja, uma única variável que pode referenciar esse valor. Se um valor é passado como parâmetro para uma função, ocorre uma movimentação (*move*) do valor, tornando a variável original inutilizável.

### Solução 1: Retornar Valores

Uma solução inicial seria informar que a função retorna uma `String`. Após utilizar esse valor — que é um novo ponteiro para a memória alocada na heap — poderíamos retornar esse ponteiro. Assim, onde utilizamos essa função, poderíamos definir uma nova variável que se tornaria uma nova `String`, como no exemplo a seguir:

```rust
let outra_string = rouba(uma_string);
println!("{}", outra_string);
```

Dessa forma, garantimos que apenas um valor permaneça na heap. O que foi copiado entre as funções foi somente o ponteiro, enquanto o valor original na heap não é duplicado. Como já discutido, a alocação de memória na heap é um processo custoso, e por isso, apenas valores localizados na stack são copiados.

Após a compilação, não encontramos mais o erro, e ao executar, a saída exibe "Vinicius" duas vezes sem problemas. Essa é uma solução válida, porém, pode ser considerada uma abordagem que não é ideal. Caso a função precisasse retornar um valor que não fosse uma `String`, teríamos que retornar múltiplos valores ou utilizar uma estrutura de dados, o que não é conveniente.

### Solução 2: Empréstimo de Valores

Podemos adotar uma abordagem mais elegante. Apenas uma variável pode ser a proprietária de um valor na heap. Contudo, essa variável pode emprestar o valor a outra função. Quando um valor é emprestado, ele é utilizado e posteriormente devolvido ao proprietário original.

Podemos modificar o código para passar uma referência da variável ao invés do valor em si:

```rust
rouba(&uma_string);
```

Neste caso, a função recebe uma referência à `String`, modificando a assinatura da função para `fn rouba(string: &String)`. Com isso, o proprietário do valor alocado na heap continua sendo `uma_string`, que agora empresta seu valor para a função. Quando a função conclui sua execução e o parâmetro sai de escopo, o valor na heap permanece intacto, e o *ownership* é mantido com `uma_string`.

Após a compilação, não encontramos erros e a execução exibe os dois textos conforme esperado. Um ponto importante a destacar é que, da mesma forma que podemos referenciar uma variável, também podemos desreferenciar uma referência, embora isso não seja necessário para o conteúdo deste tutorial.

### Referências Imutáveis e Mutáveis

Vale ressaltar que, por padrão, tanto variáveis quanto referências são imutáveis em Rust. Caso tentemos alterar uma `String` imutável, como mostrado a seguir, a compilação falhará:

```rust
string.push_str("Dias");
```

Essa operação não é permitida, uma vez que estamos tentando modificar uma referência imutável. A mensagem de erro será semelhante a "Não posso pegar emprestado um ponteiro de string como mutável por trás de uma referência". Isso indica que a referência recebida é imutável.

Para modificar um valor, devemos transformar a referência em uma referência mutável, utilizando a palavra-chave `mut`. Além disso, a variável original também deve ser declarada como mutável. O código deve ser alterado para:

```rust
let mut uma_string = String::from("Vinicius");
rouba(&mut uma_string);
```

Ao compilar este código, o valor alterado será refletido fora da função, permitindo que modificações sejam feitas na heap sem alterar o proprietário original.

### Conclusão

Neste tutorial, abordamos dois conceitos fundamentais em Rust: *ownership* e *borrowing* (empréstimo). As referências, por padrão, são imutáveis, mas podem ser convertidas em referências mutáveis quando necessário. Compreender esses conceitos é crucial para o gerenciamento eficiente da memória em Rust.

Em tutoriais futuros, exploraremos outros tópicos importantes que, embora não relacionados diretamente ao gerenciamento de memória, são essenciais para a compreensão mais ampla da linguagem Rust.

## Questão: Emprestando recursos

No que consiste o princípio de `borrowing` em Rust? (*Selecione uma alternativa*)

**A.** Aplicar um outro princípio chamado de `move semantics`.

**B.** Passar uma referência para evitar cópia de dados.

**C.** Realizar cópia de dados na heap de forma mais rápida.

**Resposta**

**A.** Incorreta. `Move semantics` é justamente o que evitamos ao utilizar `borrowing`. A ideia de `move semantics` é, basicamente, invalidar a variável original.

**B.** Correta. Ao passar a referência para uma variável que aponta para um endereço na heap, nós não precisamos copiar o dado da heap nem mover o recurso (tornando a variável original inválida).

**C.** Incorreta. Na verdade, estamos evitando cópias.

---

# 03 - Pattern Matching

No conteúdo anterior, abordamos conceitos densos. Agora, vamos simplificar um pouco, mantendo a relevância do tema, e explorar o **Pattern Matching** utilizando o **Match Statement**. Esse conceito é muito útil e, talvez, já seja familiar para você.

## Criando a Função `pattern_matching()`

Vamos criar uma nova função chamada `pattern_matching()` para trabalharmos fora da função `main`. Inicialmente, faremos um **Pattern Matching** com números, mas esse conceito pode ser aplicado a vários outros tipos de dados.

```rust
fn pattern_matching() {
    for x in 1..=20 {
        println!("{}: {}", x, match x {
            1 => "Pouco",
            2 | 3 => "Um pouquinho",
            4..=10 => "Um bocado",
            _ if x % 2 == 0 => "Uma boa quantidade",
            _ => "Muito",
        });
    }
}
```

Neste exemplo:

- Para o valor **1**, exibimos "Pouco".
- Para os valores **2** ou **3**, exibimos "Um pouquinho".
- Para o intervalo de **4** a **10**, exibimos "Um bocado".
- Para números pares acima de **10**, exibimos "Uma boa quantidade".
- Para qualquer outro valor, exibimos "Muito".

### Compilação e Execução

Ao compilar e executar o código (`rustc main.rs` e `./main`), o comportamento esperado é o seguinte:

- O valor **1** resulta em "Pouco".
- Os valores **2** e **3** resultam em "Um pouquinho".
- Os valores de **4** a **10** resultam em "Um bocado".
- Números pares acima de **10** (como **12**, **14**, **16**, **18**, **20**) resultam em "Uma boa quantidade".
- Números ímpares resultam em "Muito".

## Sintaxe de Intervalos Inclusivos e Exclusivos

Neste exemplo, utilizamos a sintaxe de intervalos inclusivos (`4..=10`). Vale destacar que a sintaxe de intervalos exclusivos ainda é experimental no Rust e pode gerar erros de compilação sem a devida configuração adicional. Existe uma _issue_ aberta no GitHub sobre isso, que pode ser acompanhada para mais informações.

## Usando `if` no Pattern Matching

Também podemos adicionar condições extras aos padrões. Por exemplo, no código acima, usamos a condição `_ if x % 2 == 0` para casar apenas com números pares.

## Pattern Matching com Estruturas de Dados

Além de números, o **Pattern Matching** pode ser utilizado em estruturas de dados. Por exemplo, com tuplas:

```rust
let point = (x, y);
match point {
    (0, 0) => "Origem",
    (0, _) => "Eixo X",
    (_, 0) => "Eixo Y",
    _ => "Outro ponto",
}
```

Neste exemplo:

- O ponto `(0, 0)` representa a origem.
- O ponto `(0, y)` indica que o valor está no **eixo X**.
- O ponto `(x, 0)` indica que o valor está no **eixo Y**.

## Considerações Finais

O **Match Statement** no Rust vai além de um simples **Switch**, permitindo trabalhar com padrões mais sofisticados, como intervalos, tuplas e condições adicionais, como `if`. Isso torna o **Pattern Matching** uma ferramenta extremamente poderosa na linguagem.

No próximo conteúdo, vamos abordar como o Rust lida com erros em tempo de execução. Fique atento!

---

# 04 - Erros

Neste capítulo, abordaremos como o **Rust** lida com erros, tanto irrecuperáveis quanto recuperáveis, e como podemos gerar erros em nosso código. Diferentemente de outras linguagens, o Rust não utiliza exceções como método de tratamento de erros. Vamos explorar isso em detalhes.

## Erros Irrecuperáveis

Vamos começar criando um exemplo de um erro irrecuperável. Para isso, utilizamos um vetor e tentamos acessar um índice inválido.

```rust
fn erros() {
    let v = vec![1, 2, 3]; // Vetor com 3 elementos
    println!("{}", v[4]);  // Acesso a um índice inválido
}
```

Ao compilar e executar o código (`rustc main.rs` e `./main`), o Rust gerará um **panic**, informando que houve um acesso fora dos limites do vetor. O erro será algo como:

```
thread 'main' panicked at 'index out of bounds: the len is 3 but the index is 4'
```

O Rust não permite o uso de estruturas como `try-catch` para esse tipo de erro. Esse tipo de erro é considerado irrecuperável, resultando em um **panic**, que interrompe a execução do programa.

### Gerando um Panic Manualmente

Também podemos gerar um **panic** manualmente em nosso código, utilizando a macro `panic!`.

```rust
fn erros() {
    panic!("Erro proposital");
}
```

Ao executar, o programa exibirá a mensagem de erro fornecida:

```
thread 'main' panicked at 'Erro proposital'
```

Caso deseje mais detalhes sobre onde o erro ocorreu, pode-se utilizar a variável de ambiente `RUST_BACKTRACE=1` para obter um **backtrace** mais detalhado.

## Erros Recuperáveis com `Result`

O Rust também oferece uma forma de tratar erros recuperáveis utilizando o tipo especial `Result`. Vamos criar uma função que retorna um `Result`, podendo ser um sucesso ou um erro.

```rust
fn resultado() -> Result<String, u8> {
    Ok(String::from("Tudo deu certo"))
}
```

Neste exemplo, a função `resultado` retorna um `Ok` com uma string no caso de sucesso e um `Err` com um número inteiro em caso de erro.

### Manipulando o Resultado com `match`

Para capturar o retorno de `Result`, podemos utilizar o `match` para tratar o sucesso e o erro de maneira apropriada.

```rust
fn main() {
    let resultado = resultado();
    
    match resultado {
        Ok(s) => println!("String de sucesso = {}", s),
        Err(numero) => println!("Código de erro = {}", numero),
    }
}
```

Se o resultado for sucesso, a string "Tudo deu certo" será exibida. Caso contrário, o código de erro será mostrado.

### Simulando um Erro

Para simular um erro, podemos alterar a função para retornar um `Err` ao invés de um `Ok`.

```rust
fn resultado() -> Result<String, u8> {
    Err(42)
}
```

Ao executar o código, veremos a seguinte saída:

```
Código de erro = 42
```

## Qual Tipo de Tratamento Utilizar?

Via de regra, é recomendado utilizar `Result` para dar ao código chamador a opção de tratar o erro da maneira que desejar. Apenas em casos onde o erro for realmente irrecuperável, ou onde o programador tiver informações que o compilador não tem, o uso de `panic!` pode ser justificado.

## Conclusão

O Rust oferece duas formas principais de lidar com erros:

- **Erros irrecuperáveis**: Utilizando a macro `panic!`, o programa interrompe sua execução ao encontrar um erro grave.
- **Erros recuperáveis**: Utilizando o tipo `Result`, podemos retornar erros e permitir que o código chamador os trate de maneira apropriada.

No próximo capítulo, exploraremos o ecossistema do Rust, incluindo ferramentas, gerenciamento de dependências, e como criar projetos de forma profissional.

## Questão: Tipos de Erro

Vimos que o Rust possui dois principais tipos de erro, ou seja, duas formas de sinalizar que algo não ocorreu conforme esperado. Qual o nome do erro que não é recuperável? (*Selecione uma alternativa*)

**A.** error!

**B.** crash!

**C.** panic!

**Resposta**

**A.** Incorreta.

**B.** Incorreta.

**C.** Correta. Inclusive, `panic!` é o nome da macro que utilizamos para emitir este tipo de erro.

---

# 05 - O que aprendemos?

Nesta aula:

- Conhecemos o conceito de ***ownership***;
- Aprendemos como o Rust **gerencia memória**;
- Vimos sobre ***pattern matching***;
- Entendemos como Rust lida com **erros**.
