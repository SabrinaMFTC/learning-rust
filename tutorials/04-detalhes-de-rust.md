# 01 - Ownership

Neste capítulo, discutimos um dos conceitos mais importantes do Rust: o **ownership**, ou seja, como a linguagem gerencia a memória de forma única, sem o uso de Garbage Collector ou alocação manual, comum em linguagens como C e C++.

## Funcionamento

- Rust utiliza **ownership** para gerenciar a memória de forma eficiente. Cada valor na heap tem um único dono (owner).
- Quando uma variável é passada para uma função, o **ownership** do valor na heap é transferido para a função, invalidando a variável original.
- Ao sair de escopo, o valor é liberado da heap automaticamente, eliminando a necessidade de um Garbage Collector.
- Strings são alocadas na heap por serem dinâmicas e potencialmente grandes, ao contrário de valores menores, como inteiros, que são armazenados na stack.

## Comparação com outras linguagens

  - Em C++: uso de ponteiros e liberação manual de memória.
  - Em Java/C#/PHP: uso de Garbage Collector para liberar memória.

## Vantagens do modelo de gestão de memória do Rust

  - Evita overhead de um Garbage Collector.
  - Garante que a memória seja liberada imediatamente quando o valor sai de escopo.

Na próxima aula, exploraremos o conceito de **Borrowing** (empréstimo de referências) e como podemos passar variáveis por parâmetro sem transferir o ownership, permitindo que elas continuem a ser usadas posteriormente.

---

# 02 - Referências e Borrowing

Neste tutorial, abordaremos um problema relacionado ao conceito de *ownership* (propriedade) em Rust.

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

## Resposta

**A.** Incorreta. `Move semantics` é justamente o que evitamos ao utilizar `borrowing`. A ideia de `move semantics` é, basicamente, invalidar a variável original.

**B.** Correta. Ao passar a referência para uma variável que aponta para um endereço na heap, nós não precisamos copiar o dado da heap nem mover o recurso (tornando a variável original inválida).

**C.** Incorreta. Na verdade, estamos evitando cópias.

---

# 03 - Pattern Matching
