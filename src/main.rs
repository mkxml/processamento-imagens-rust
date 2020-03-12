// Lib externa para lidar com imagens em Rust
extern crate image;

// Módulos locais
mod img;
mod load;

// Importando tipo PixelMatrix
use img::PixelMatrix;

// Fixando o tamanho do canvas
// TODO: Para futuros exercícios, tornar o canvas dinâmico, atualmente fixo em 2000x2000
const CANVAS_WIDTH: u32 = 2000;
const CANVAS_HEIGHT: u32 = 2000;

fn main() {
    // Carregando imagens de origem
    let equilateral = load::open_image("equilatero.jpg");
    let isoceles = load::open_image("isoceles.jpg");
    let scalene = load::open_image("escaleno.jpg");
    let obtuse = load::open_image("obtusangulo.jpg");
    let right_triangle = load::open_image("retangulo.jpg");

    // Cria o primeiro canvas para armanzenar o resultado das transformações
    let mut canvas = img::new_canvas(CANVAS_WIDTH, CANVAS_HEIGHT);

    /* Aplicando transformações */

    // Amplia o equilatero em 2x, salva e zera o canvas
    img::scale(&equilateral, &mut canvas, 2.0, 2.0);
    save_as_image("Equilatero", &canvas, "equilatero_transformado.jpg");
    canvas = img::new_canvas(CANVAS_WIDTH, CANVAS_HEIGHT);

    // Diminui o isoceles pela metade
    img::scale(&isoceles, &mut canvas, 0.5, 0.5);
    save_as_image("Isóceles", &canvas, "isoceles_transformado.jpg");
    canvas = img::new_canvas(CANVAS_WIDTH, CANVAS_HEIGHT);

    // Rotaciona o escaleno em 90 graus no sentido anti-horário
    img::rotate(&scalene, &mut canvas, -90.0);
    save_as_image("Escaleno", &canvas, "escaleno_transformado.jpg");
    canvas = img::new_canvas(CANVAS_WIDTH, CANVAS_HEIGHT);

    // Transladando o retângulo em 200 pixels em cada eixo
    img::translate(&right_triangle, &mut canvas, 200, 200);
    save_as_image("Retângulo", &canvas, "retangulo_transformado.jpg");
    canvas = img::new_canvas(CANVAS_WIDTH, CANVAS_HEIGHT);

    // Espelhando o obtusângulo na vertical
    img::flip(&obtuse, &mut canvas, img::Direction::Vertical);
    save_as_image("Obtusângulo", &canvas, "obtusangulo_transformado.jpg");
    canvas = img::new_canvas(CANVAS_WIDTH, CANVAS_HEIGHT);

    // Espelhando o triângulo retângulo na vertical
    img::flip(&right_triangle, &mut canvas, img::Direction::Horizontal);
    save_as_image(
        "Retângulo espelhado",
        &canvas,
        "retangulo_espelhado_transformado.jpg",
    );
}

fn save_as_image(name: &str, canvas: &PixelMatrix, filename: &str) {
    let width = canvas.len();
    let height = canvas[0].len();
    let output_image = img::pixel_matrix_to_image(canvas, width as u32, height as u32);
    match output_image.save(filename) {
        Ok(()) => println!("{:?} processado, salvo como {:?}", name, filename),
        Err(err) => println!("Erro ao salvar a imagem: {:?}", err),
    }
}
