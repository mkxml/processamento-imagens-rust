// Lib externa para lidar com imagens em Rust
extern crate image;

// Módulos locais
mod img;
mod load;

// Importando tipo PixelMatrix
use img::PixelMatrix;

// Tamanho default do canvas
const DEFAULT_CANVAS_WIDTH: u32 = 2000;
const DEFAULT_CANVAS_HEIGHT: u32 = 2000;

fn main() {
    // Carregando imagens de origem
    let mut demo1 = load::open_image("exercicio1.jpg");
    let demo2 = load::open_image("exercicio2.jpg");
    let mut demo3 = load::open_image("exercicio3.jpg");
    let mut demo4 = load::open_image("exercicio4.jpg");
    let mut demo5 = load::open_image("exercicio5.jpg");

    /* Aplicando transformações */

    // Exercicio 1
    img::brightness(&mut demo1, 30);
    img::contrast(&mut demo1, 1.4);
    img::median_filter(&mut demo1);
    save("Exercicio 1", &demo1, "exercicio1_output.jpg");

    // Exercicio 2
    let (width, height) = demo2.dimensions();
    let mut canvas = img::new_canvas(width, height);
    img::border_detection(&demo2, &mut canvas, 10);
    save_as_image("Exercicio 2", &canvas, "exercicio2_output.jpg");

    // Exercicio 3
    img::brightness(&mut demo3, 50);
    img::contrast(&mut demo3, 1.25);
    save("Exercicio 3", &demo3, "exercicio3_output.jpg");

    // Exercicio 4
    img::gaussian_filter(&mut demo4);
    save("Exercicio 4", &demo4, "exercicio4_output.jpg");

    // Exercicio 5
    let (width, height) = demo5.dimensions();
    canvas = img::new_canvas(width, height);
    img::average_filter(&mut demo5, &mut canvas);
    img::threshold(
        &mut img::pixel_matrix_to_image(&canvas, width, height),
        &mut canvas,
        60,
    );
    save_as_image("Exercicio 5", &canvas, "exercicio5_output.jpg");
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

fn save(name: &str, img: &image::RgbImage, filename: &str) {
    match img.save(filename) {
        Ok(()) => println!("{:?} processado, salvo como {:?}", name, filename),
        Err(err) => println!("Erro ao salvar a imagem: {:?}", err),
    }
}
