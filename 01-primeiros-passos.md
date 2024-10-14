# 01 - Introdução

Sejam bem-vindos ao curso de **Rust**. Neste treinamento, vamos explorar a linguagem de programação **Rust**, destacando suas características e as diferenças em relação a outras linguagens do mercado. Nosso objetivo é entender o que torna o **Rust** uma escolha especial para desenvolvimento.

## Preparação do Ambiente

O primeiro passo será configurar o ambiente de desenvolvimento. Vamos instalar o **compilador** e executar nosso código a partir dele. Esse processo é essencial para começarmos a programar em **Rust**.

## Conceitos Básicos

Com o ambiente configurado, abordaremos os conceitos fundamentais da linguagem:

- Declaração de variáveis.
- Tipos de variáveis e o controle que o **Rust** oferece sobre a quantidade de memória utilizada por cada tipo de dado.

## Sintaxe Avançada

Após entendermos as variáveis, avançaremos para aspectos mais avançados da sintaxe, como:

- Constantes.
- Conceitos de **Shadowing** e **Escopo** (incluindo **Escopo Unsafe**).
- Definição de funções, recebimento de parâmetros e retorno de valores.

## Controle de Fluxo

Nesta etapa, aprenderemos a controlar o fluxo de execução de programas:

1. Estruturas condicionais como o **if**.
2. Laços de repetição (**loops**).
3. Expressão **match** e sua utilidade na construção de estruturas de controle.

## Especificidades do Rust

A seguir, abordaremos aspectos específicos do **Rust**, como:

- **Gerenciamento de Memória**: Analisaremos os conceitos de **Ownership** e **Borrowing**, fundamentais para o controle eficiente de memória no **Rust**.

## Outros Recursos

Por fim, veremos:

- **Pattern Matching**: Uma introdução às possibilidades dessa técnica.
- Gerenciamento de projetos no ecossistema do **Rust**, incluindo o uso de dependências e o processo de **build**.
- Integração com extensões para editores de código.

## Conclusão

Este curso é uma introdução à linguagem **Rust**, oferecendo uma visão inicial sobre suas funcionalidades. Não abordaremos tópicos avançados, mas o conteúdo será suficiente para iniciar seus estudos de forma sólida.

---

# 02 - Por que utilizar Rust?

Antes de começarmos a escrever código em **Rust**, é importante entender por que utilizar essa linguagem de programação e qual é o seu propósito. Rust tem como foco **performance**, **confiabilidade** e **produtividade**.

## Comparação com outras linguagens

O **Rust** é uma linguagem **compilada**, o que a diferencia de linguagens interpretadas como **Python**, **JavaScript**, **PHP** e **Ruby**, e também de linguagens que rodam em máquinas virtuais, como **C#** e **Java**. Nesse sentido, **Rust** se assemelha a linguagens como **C** e **C++**, sendo compilada diretamente para código de máquina, gerando um binário executável.

## Confiabilidade e sistema de tipos

Uma das principais características do **Rust** é o seu sistema de tipos robusto, que oferece uma grande flexibilidade no controle da memória. A linguagem permite otimizar o uso da memória conforme necessário, proporcionando tanto eficiência quanto flexibilidade.

Além disso, o Rust é projetado para auxiliar na gestão de memória, reduzindo os erros comuns associados a vazamentos de memória que ocorrem em linguagens como **C** e **C++**. Ele automatiza a liberação de recursos quando apropriado, prevenindo problemas como a alocação indefinida de memória.

## Produtividade e gerenciamento de memória

Diferentemente de linguagens como **C++**, onde a alocação e liberação de memória precisam ser gerenciadas explicitamente, o **Rust** oferece um controle rigoroso de alocação de memória, garantindo que os recursos sejam liberados automaticamente quando não forem mais necessários. Isso aumenta a produtividade ao minimizar a necessidade de intervenção manual e ao prevenir vazamentos de memória.

O **compilador do Rust** também é elogiado por suas mensagens de erro claras e úteis, tornando o processo de desenvolvimento mais fluido e menos propenso a frustrações, especialmente em comparação com compiladores como **GCC** e **G++**.

## Ecossistema e ferramentas

O **Rust** possui um ecossistema sólido, com ferramentas como gerenciadores de dependências e uma build tool eficiente. Esse ecossistema facilita o desenvolvimento de projetos robustos sem complicações. Mesmo que alguns desses conceitos pareçam complexos no início, eles serão abordados ao longo do treinamento.

## Aplicações do Rust

O **Rust** combina o melhor de dois mundos: a performance de linguagens compiladas como **C++** e a produtividade e segurança de linguagens que rodam em máquinas virtuais. É por isso que o **Rust** é amplamente utilizado em projetos que exigem alta performance. Um exemplo é o navegador **Firefox**, que tem partes significativas escritas em **Rust**. Há também projetos que utilizam **Rust** para o núcleo de sistemas operacionais, como o **Windows**.

Além disso, o **Rust** permite desenvolver desde ferramentas de baixo nível até aplicações web, graças à sua flexibilidade e ao suporte a diversas bibliotecas.

## Conclusão

O **Rust** é uma linguagem **compilada** focada em **performance**, **produtividade** e **confiabilidade**. Ela busca unir as vantagens de linguagens de baixo e alto nível, oferecendo um ambiente eficiente e seguro para o desenvolvimento de software. Agora que compreendemos o propósito da linguagem, vamos proceder com a instalação do **Rust** em nossa máquina para começarmos a escrever código.

## Questão: Tipo de linguagem

Qual das alternativas a seguir é verdadeira sobre Rust? (*Selecione uma alternativa*)

**A.** É uma linguagem compilada.

**B.** É uma linguagem interpretada.

**C.** É uma linguagem que roda em VM.

**Resposta**

**A.** Correta. Rust, assim como C e C++, é uma linguagem compilada.

**B.** Incorreta. Rust não é como JavaScript, PHP ou Python.

**C.** Incorreta. Rust não é como C# ou Java.

---

# 03 - Instalando Rust

Neste passo, vamos instalar o **Rust** para começarmos a escrever e executar códigos. Como já discutido anteriormente, o Rust é uma linguagem **compilada**, então precisaremos do compilador para transformar nosso código em um executável.

## Passo a passo da instalação

Para iniciar, vamos acessar a página [Getting Started](https://www.rust-lang.org/learn/get-started) do site oficial do **Rust**. A documentação é bastante amigável e funciona mais como um tutorial do que uma documentação técnica extensa.

Dependendo do seu sistema operacional, o site já oferece a forma mais adequada de instalação:

- **Windows**: O instalador é disponibilizado diretamente ou pode-se usar o **WSL**.
- **Linux** e **macOS**: São fornecidos comandos práticos para instalação via terminal.

### Instalação no Windows

No **Windows**, além de baixar o instalador do **Rust**, é necessário instalar as **Ferramentas de Build do Visual Studio**. Essas ferramentas, que incluem o suporte a **C++**, são essenciais para compilar o código no ambiente Windows. Após o download e a instalação dessas ferramentas, você pode baixar e executar o instalador do Rust. Ele verificará as dependências e realizará a instalação.

### Configurando o ambiente

Após a instalação, você pode abrir um terminal no Windows (**PowerShell**, **Git Bash**, ou outro de sua preferência) e verificar a instalação com os comandos:

```bash
rustup -V
rustc -V
```

Esses comandos exibirão a versão do rustup e do rustc instalados. Mesmo que sua versão seja diferente da mostrada no tutorial, isso não afetará o aprendizado, já que focaremos nos fundamentos da linguagem.

## Criando e executando o primeiro código
Agora que o Rust está instalado, podemos escrever nosso primeiro código, o tradicional "Hello World".

**1.** Crie um arquivo chamado main.rs com o seguinte conteúdo:

```rust
fn main() {
    println!("Ola mundo!");
}
```

**2.** Salve o arquivo e volte ao terminal. Para compilar o código, utilize o compilador do Rust, rustc:

```bash
rustc main.rs
```

**3.** Após a compilação, um executável será gerado. Para executá-lo no terminal, digite:

```bash
./main.exe
```

Isso exibirá a mensagem **"Ola mundo!"** no terminal.

## Conclusão

Com o ambiente configurado e o primeiro código compilado e executado, já estamos prontos para avançar no estudo da sintaxe do Rust e explorar suas funcionalidades.

---

# 04 - O que aprendemos?

Nesta aula, aprendemos:

- O que é e para que serve o **Rust**, além dos motivos para se utilizar essa linguagem;
- Como configurar o ambiente de desenvolvimento **Rust**;
- Como compilar o primeiro código utilizando o **rustc**.
