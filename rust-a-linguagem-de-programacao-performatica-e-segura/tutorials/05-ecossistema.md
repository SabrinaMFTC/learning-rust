# 01 - Conhecendo o Cargo

Neste capítulo, vamos explorar o **ecossistema do Rust** e entender como projetos profissionais são organizados e gerenciados. Até agora, estivemos executando o compilador manualmente em projetos pequenos com um único arquivo, mas em cenários mais complexos, com múltiplos arquivos e dependências externas, isso não seria eficiente. É aqui que entra o **Cargo**, a ferramenta padrão para gerenciar pacotes e o build de projetos em Rust.

## Gerenciamento de Dependências e Builds

Assim como outras linguagens modernas possuem gerenciadores de pacotes (como o **NPM** para JavaScript, **Composer** para PHP ou **NuGet** para C#), o Rust também possui o **Cargo** para gerenciar dependências e fazer o build dos projetos. A principal vantagem do Cargo é que ele unifica o gerenciamento de pacotes e o processo de build em uma única ferramenta.

### Vantagens do Cargo

- **Gerenciamento de dependências**: O Cargo permite que você baixe, configure e controle versões de bibliotecas externas diretamente do repositório oficial do Rust.
- **Build automatizado**: Ele organiza os arquivos do projeto, compila o código e gera pacotes, sem a necessidade de rodar comandos manuais para cada arquivo.
- **Criação de bibliotecas**: O Cargo também facilita a criação de bibliotecas para serem distribuídas e utilizadas por outros desenvolvedores.

### Comparação com Outras Linguagens

Ao contrário de linguagens como **C++**, que não possuem uma solução oficial para gerenciar dependências e builds, o Cargo centraliza esses processos, evitando que cada projeto tenha sua própria solução customizada. Em linguagens sem essa centralização, como o C++, os desenvolvedores muitas vezes precisam baixar dependências manualmente e configurar builds com ferramentas variadas, o que pode causar inconsistências.

## Instalando e Verificando o Cargo

A boa notícia é que o **Cargo** já vem instalado junto com o **Rust**. Para verificar se ele está instalado corretamente, basta abrir o terminal e rodar o comando:

```bash
cargo --version
```

Este comando exibirá a versão do Cargo, que geralmente coincide com a versão do compilador Rust, que pode ser verificada com:

```bash
rustc --version
```

## Organização de Projetos com o Cargo

Uma das principais vantagens do Cargo é a sua capacidade de organizar os arquivos do projeto de maneira limpa e eficiente. Ao invés de ter todos os arquivos (como o código-fonte, arquivos de debug, e executáveis) na mesma pasta, o Cargo separa esses elementos, criando uma estrutura de diretórios bem definida para projetos em Rust.

No próximo capítulo, começaremos a explorar como o Cargo organiza e gerencia seus projetos, facilitando o desenvolvimento e a manutenção de projetos Rust profissionais.

## Questão: Propósito

Vimos  que o Rust fornece uma ferramenta muito interessante chamada **Cargo**. Para que serve essa ferramenta do ecossistema Rust? (*Selecione 2 alternativas*)

**A.** Para gerenciar recursos estáticos como imagens e arquivos externos.

**B.** Para realizar os builds de nossos projetos.

**C.** Para gerenciar as nossas dependências.

**Resposta**

**A.** Incorreta. Essa não é a finalidade do Cargo.

**B.** Correta. Por meio do Cargo, nós podemos construir e executar nossos projetos de forma bem simplificada.

**C.** Correta. Nós utilizamos o Cargo para baixar e gerenciar bibliotecas externas em nosso projeto.

---

# 02 - Principais Comandos

Neste capítulo, vamos aprender como utilizar o **Cargo** para organizar e gerenciar projetos no Rust. Até agora, estivemos trabalhando com o compilador manualmente. Agora, veremos como estruturar um projeto existente com o Cargo e como criar novos projetos diretamente com ele.

## Organizando um Projeto Existente

Vamos pegar um projeto que já criamos e organizá-lo utilizando as convenções do Cargo:

1. **Mover o código para a pasta "src"**: O Cargo utiliza a pasta `src` para armazenar o código-fonte. Primeiro, crie a pasta `src` e mova o arquivo `main.rs` para dentro dela.
   
   ```bash
   mkdir src
   mv main.rs src/
   ```

2. **Criar o arquivo `Cargo.toml`**: O arquivo `Cargo.toml` é usado pelo Cargo para gerenciar configurações do projeto, como nome, versão e autores. Vamos criar o arquivo na raiz do projeto com o seguinte conteúdo:

   ```toml
   [package]
   name = "alura_rust"
   version = "0.0.1"
   authors = ["Vinicius Dias <vinicius@dias.dev>"]
   ```

Com essa estrutura, o Cargo pode ser utilizado para compilar e rodar o projeto. Em vez de usar o `rustc` manualmente, basta rodar:

```bash
cargo run
```

Se houver algum erro no arquivo `Cargo.toml`, como um campo incorreto (por exemplo, `nome` em vez de `name`), o Cargo indicará o erro e será possível corrigir rapidamente.

## Criação de um Novo Projeto com Cargo

Além de organizar projetos existentes, o Cargo facilita a criação de novos projetos de forma automatizada. Vamos ver como criar um novo projeto diretamente com o Cargo:

1. **Criando um novo projeto**: Para criar um novo projeto, utilize o comando `cargo new`. Esse comando cria toda a estrutura de pastas e arquivos necessários para um projeto Rust com Cargo. Por exemplo, para criar um novo projeto chamado `alura_rust_cargo`, execute:

   ```bash
   cargo new alura_rust_cargo
   ```

   Isso cria a seguinte estrutura de diretórios:

   ```
   alura_rust_cargo/
   ├── Cargo.toml
   └── src/
       └── main.rs
   ```

2. **Executando o novo projeto**: O arquivo `main.rs` gerado pelo Cargo já contém um exemplo simples de um programa Rust que exibe "Hello, world!". Para compilar e executar o projeto, use:

   ```bash
   cargo run
   ```

   O resultado será:

   ```
   Hello, world!
   ```

## Entendendo o Arquivo `Cargo.toml`

O arquivo `Cargo.toml` gerado pelo Cargo contém algumas informações adicionais. Por exemplo, ele define a **edição** do Rust que o projeto está utilizando. A edição padrão mais recente é a **2021**:

```toml
[package]
name = "alura_rust_cargo"
version = "0.1.0"
edition = "2021"
```

Isso é similar às versões do Visual Studio (por exemplo, 2015, 2017, 2021) que definem o comportamento do ambiente de desenvolvimento.

## Comandos Úteis do Cargo

Além de `cargo run`, o Cargo oferece vários outros comandos úteis, como:

- `cargo build`: Compila o projeto, mas não o executa.
- `cargo init`: Inicializa um novo projeto no diretório atual.
- `cargo --help`: Exibe a lista completa de comandos e opções do Cargo.

Você pode explorar mais comandos utilizando `cargo --help` para descobrir todas as funcionalidades que ele oferece.

No próximo capítulo, vamos explorar como configurar o **Visual Studio Code** para melhorar a experiência de desenvolvimento em Rust e verificar se existem extensões ou ferramentas que facilitam o uso do Rust em outras IDEs e editores de texto.

---

# 03 - Editor de Código ou IDE

Agora que já conhecemos o **Cargo** e como ele facilita a organização e a execução dos nossos projetos em Rust, vamos explorar como integrar o desenvolvimento com editores de código e IDEs, utilizando as ferramentas disponíveis para maximizar a produtividade.

## Suporte a Editores de Código

O Rust oferece **suporte de primeira classe** a vários editores de código. Independentemente de qual editor ou IDE você utilize, existem ferramentas e extensões que ajudam a melhorar sua experiência de desenvolvimento com Rust. Entre os editores suportados, destacam-se:

- **Visual Studio Code**
- **Sublime Text**
- **Atom**
- **Vim**
- **Emacs**
- IDEs mais completas como **Eclipse** e **IntelliJ IDEA**

Essas ferramentas geralmente oferecem suporte a **autocompletar código**, **navegação rápida** entre funções, **formatação automática**, e muito mais.

### Extensão para Visual Studio Code

Se você usa o **Visual Studio Code**, a instalação da extensão de Rust oferece várias funcionalidades úteis, como:

- **Code completion**: Autocompleta o código à medida que você escreve.
- **Jump to definition**: Ao pressionar "Ctrl" e clicar em uma função, o editor navega diretamente para a definição daquela função.
- **Documentação inline**: Exibe a documentação ao passar o mouse sobre funções ou variáveis.
- **Formatação e refatoração de código**: Automatiza a formatação do código para mantê-lo organizado e consistente.
- **Build tasks**: Adiciona tarefas automatizadas ao projeto, facilitando o processo de build e execução do código.

### Configurando Tarefas de Build no VS Code

Com a extensão do Rust instalada no Visual Studio Code, você pode configurar e executar **tarefas de build** diretamente no editor. Veja como configurar e utilizar essa funcionalidade:

1. **Instalando os componentes adicionais**: Ao abrir um projeto Rust, o Visual Studio Code pode solicitar a instalação de alguns componentes adicionais. Instale-os para habilitar todas as funcionalidades.

2. **Executando tarefas no VS Code**:
   - Pressione `Ctrl + Shift + P` e digite "Tasks".
   - Selecione **Run Task**, **Run Build Task** ou **Run Test Task** para executar diferentes tipos de tarefas.
   - No menu de tarefas, escolha **Cargo** e em seguida **cargo run**. O terminal do VS Code será aberto automaticamente, o projeto será compilado e executado no mesmo ambiente.
   - Para facilitar o processo, você pode configurar uma tarefa padrão como **cargo run**.

   Após configurar, você pode simplesmente usar o atalho `Ctrl + Shift + B` para compilar e rodar seu projeto sem precisar ir ao terminal.

### Outras Funcionalidades

Além de tarefas de build, a extensão também oferece outras funcionalidades úteis, como:

- **Jump to definition**: Ao pressionar "Ctrl" e clicar em uma função, você é levado diretamente para a definição da função no código.
- **Documentação inline**: Ao passar o mouse sobre funções ou variáveis, a documentação é exibida automaticamente.
- **Navegação eficiente**: Com "Ctrl" + clique, você pode navegar rapidamente entre diferentes partes do código.

Essas funcionalidades facilitam a vida do desenvolvedor e aumentam a produtividade, eliminando a necessidade de alternar entre terminal e editor de código. Vale a pena configurar essas ferramentas, especialmente se você está utilizando uma IDE ou editor de código que oferece suporte ao Rust.

A própria equipe do Rust fornece diversas ferramentas para que possamos trabalhar de forma profissional. Na página abaixo, você pode buscar extensões ou plugins para sua ferramenta preferida:

- [Tools](https://www.rust-lang.org/tools)

---

# 04 - O que aprendemos?

Nesta aula:

- Conhecemos o **Cargo**, ferramenta de build oficial do Rust;
- Configuramos nosso projeto para ser executado usando o **Cargo**;
- Criamos um novo projeto com `cargo new`;
- Vimos como IDEs e editores de código podem nos ajudar no processo de desenvolvimento.
