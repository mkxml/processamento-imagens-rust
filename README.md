# Processamento Digital de Imagens em Rust

Este código é usado para estudos na disciplina de processamento digital de imagens na [Universidade Feevale](https://feevale.br).

A ideia de usar a linguagem [Rust](https://www.rust-lang.org/) para estes estudos se tornou interessante, pois parece ser uma linguagem legal para aprender, visto que está ficando popular para uso em programação de sistemas.

## Escopo

No momento atual o código possibilita a manipulação de imagens utilizando transformações geométricas.

O projeto utiliza a biblioteca [image](https://docs.rs/image/0.23.1/image/) para trabalhar com as imagens.

Entretanto, para melhor compreender como funcionam as técnicas de transformação de imagens, não foi foram utilizados os métodos de transformação prontos da biblioteca. Todas as transformações realizadas foram codificadas no próprio projeto.

A biblioteca em questão é utilizada somente para carregar e salvar imagens, pois ela abstrai algumas questões como o formato da imagem.

### Funções atuais

Atualmente, o código conta com as seguintes funções básicas de transformações geométricas:

- **scale**, que permite alterar a escala da imagem, permitindo aumentar e diminuir.
- **translate**, translada a imagem em um espaço pré-determinado.
- **rotate**, rotaciona a imagem no eixo Z.
- **flip**, espelha a imagem.

Para exemplificar o uso das mesmas o código está manipulando imagens de triângulos, veja como utilizar abaixo.

## Como utilizar

O código procura pelas seguintes imagens no diretório em contexto do aplicativo:

- `equilatero.jpg`
- `isoceles.jpg`
- `escaleno.jpg`
- `obtusangulo.jpg`
- `retangulo.jpg`

O software irá processar as imagens da seguinte forma:

- Para `equilatero.jpg` o código amplia a imagem numa escala de 2x e salva o resultado como `equilatero_transformado.jpg`;
- Para `isoceles.jpg` o código diminui a imagem numa escala de 0.5x e salva o resultado como `isoceles_transformado.jpg`;
- Para `escaleno.jpg` o código rotaciona a imagem em 90 graus no sentido anti-horário e salva o resultado como `escaleno_transformado.jpg`;
- Para `obtusangulo.jpg` o código espelha a imagem na vertical e salva o resultado como `obtusangulo_transformado.jpg`;
- Para `retangulo.jpg` o código translaga a imagem em 200 pixels em ambos os eixos e também espelha a imagem na horizontal, os resultados são salvos como `retangulo_transformado.jpg` e `retangulo_espelhado_transformado.jpg` respectivamente;

## Compilando o código

Para compilar o código basta ter o Rust e a ferramenta `Cargo` instalada, os mesmos podem ser obtidos [aqui](https://www.rust-lang.org/tools/install).

O comando para gerar um build é `cargo build`.

Para rodar diretamente basta usar `cargo run`.

### Executáveis

Existem executáveis pré-compilados para Windows e Mac no diretório [release](release) deste repositório.

## Ideias para evoluir o código

- Aceitar o caminho das imagens como argumento;
- Deixar o canvas dinâmico;
- Resolver questão do offset (deixar imagem sempre no começo);
- Implementar funções de transformação além das geométricas;
- Preencher os pixels vazios que ocorrem em transformações como scale;

## Licença

[MIT](LICENSE)

## Autor

Matheus Kautzmann
