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
    let demo = load::open_image("letras.jpg");

    /* Aplicando erosão e dilatação */
    let (width, height) = demo.dimensions();
    let mut canvas = img::new_canvas(width, height);
    let filter = [[1, 1, 1], [1, 1, 1], [1, 1, 1]];
    img::threshold(&demo, &mut canvas, 128);
    img::erosion(
        &img::pixel_matrix_to_image(&canvas, width, height),
        &mut canvas,
        &filter,
    );
    img::erosion(
        &img::pixel_matrix_to_image(&canvas, width, height),
        &mut canvas,
        &filter,
    );
    img::dilation(
        &img::pixel_matrix_to_image(&canvas, width, height),
        &mut canvas,
        &filter,
    );
    save_as_image("Letras transformadas", &canvas, "letras_output.jpg");
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
