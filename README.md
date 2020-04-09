# Processamento Digital de Imagens em Rust

Este código é usado para estudos na disciplina de processamento digital de imagens na [Universidade Feevale](https://feevale.br).

A ideia de usar a linguagem [Rust](https://www.rust-lang.org/) para estes estudos se tornou interessante, pois parece ser uma linguagem legal para aprender, visto que está ficando popular para uso em programação de sistemas.

## Escopo

No momento atual o código possibilita a manipulação de imagens utilizando transformações geométricas.

O projeto utiliza a biblioteca [image](https://docs.rs/image/0.23.1/image/) para trabalhar com as imagens.

Entretanto, para melhor compreender como funcionam as técnicas de transformação de imagens, não foi foram utilizados os métodos de transformação prontos da biblioteca. Todas as transformações realizadas foram codificadas no próprio projeto.

A biblioteca em questão é utilizada somente para carregar e salvar imagens, pois ela abstrai algumas questões como o formato da imagem.

## Como utilizar

O código procura pelas seguintes imagens no diretório em contexto do aplicativo:

- `exercicio1.jpg`
- `exercicio2.jpg`
- `exercicio3.jpg`
- `exercicio4.jpg`
- `exercicio5.jpg`

## Compilando o código

Para compilar o código basta ter o Rust e a ferramenta `Cargo` instalada, os mesmos podem ser obtidos [aqui](https://www.rust-lang.org/tools/install).

O comando para gerar um build é `cargo build`.

Para rodar diretamente basta usar `cargo run`.

### Executáveis

Existem executáveis pré-compilados para Windows e Mac no diretório [release](release) deste repositório.

## Licença

[MIT](LICENSE)

## Autor

Matheus Kautzmann
