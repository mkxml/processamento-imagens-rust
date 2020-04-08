extern crate image;

use image::*;

// Tipo que representa uma matriz de pixels RGB
pub type PixelMatrix = Vec<Vec<Rgb<u8>>>;

// Tipo que representa uma matriz de posições com vetores [x, y]
pub type PositionMatrix = [[f64; 2]; 3];

// Tipo que determina a direção para a funcão flip
pub enum Direction {
    Vertical,
    Horizontal,
}

// Funcão genérica de transformações geométricas usando matrizes
fn transform(
    transformation: &PositionMatrix,
    image: &PixelMatrix,
    destination: &mut PixelMatrix,
    offset: (usize, usize),
) {
    let (offset_x, offset_y) = offset;
    let (width, height) = (image.len(), image[0].len());
    for i in 0..width {
        for j in 0..height {
            let pixel = image[i][j];
            let x0 = i as f64 * transformation[0][0]
                + j as f64 * transformation[1][0]
                + transformation[2][0];
            let y0 = i as f64 * transformation[0][1]
                + j as f64 * transformation[1][1]
                + transformation[2][1];
            destination[(x0 + offset_x as f64) as usize][(y0 + offset_y as f64) as usize] = pixel;
        }
    }
}

// Cria um novo canvas para usar como destino das transformações em imagens
pub fn new_canvas(x: u32, y: u32) -> PixelMatrix {
    let canvas = vec![vec![image::Rgb([255 as u8, 255 as u8, 255 as u8]); y as usize]; x as usize];
    canvas
}

// Transforma uma imagem RGB da biblioteca image em uma PixelMatrix para trabalhar
fn image_to_pixel_matrix(img: &image::RgbImage) -> PixelMatrix {
    let (x, y) = img.dimensions();
    let mut output_matrix = new_canvas(x, y);
    for i in 0..x {
        for j in 0..y {
            output_matrix[i as usize][j as usize] = *img.get_pixel(i, j);
        }
    }
    output_matrix
}

// Transforma uma PixelMatrix em uma imagem RGB da biblioteca image
pub fn pixel_matrix_to_image(
    pixel_matrix: &PixelMatrix,
    width: u32,
    height: u32,
) -> image::RgbImage {
    let mut output_image = image::RgbImage::new(width, height);
    for i in 0..width {
        for j in 0..height {
            output_image.put_pixel(i, j, pixel_matrix[i as usize][j as usize]);
        }
    }
    output_image
}

// Traslada uma imagem dentro de um canvas
pub fn translate(img: &image::RgbImage, canvas: &mut PixelMatrix, x: i32, y: i32) {
    let transform_matrix = [[1.0, 0.0], [0.0, 1.0], [x as f64, y as f64]];
    let image_matrix = image_to_pixel_matrix(img);
    transform(&transform_matrix, &image_matrix, canvas, (0, 0));
}

// Escalona uma imagem dentro de um canvas
pub fn scale(img: &image::RgbImage, canvas: &mut PixelMatrix, x: f64, y: f64) {
    let transform_matrix = [[x, 0.0], [0.0, y], [0.0, 0.0]];
    let image_matrix = image_to_pixel_matrix(img);
    transform(&transform_matrix, &image_matrix, canvas, (0, 0));
}

// Rotaciona uma imagem dentro de um canvas
pub fn rotate(img: &image::RgbImage, canvas: &mut PixelMatrix, deg: f64) {
    let (width, height) = img.dimensions();
    let offset = std::cmp::max(width, height) as usize;
    let radians = deg * (std::f64::consts::PI / 180.0);
    let transform_matrix = [
        [radians.cos().round() as f64, radians.sin().round() as f64],
        [-radians.sin().round() as f64, radians.cos().round() as f64],
        [0.0, 0.0],
    ];
    let image_matrix = image_to_pixel_matrix(img);
    transform(&transform_matrix, &image_matrix, canvas, (offset, offset));
}

// Espelha uma imagem dentro de um canvas
pub fn flip(img: &image::RgbImage, canvas: &mut PixelMatrix, direction: Direction) {
    let (width, height) = img.dimensions();
    let (transform_matrix, offset) = match direction {
        Direction::Vertical => ([[1.0, 0.0], [0.0, -1.0], [0.0, 0.0]], (0, height as usize)),
        Direction::Horizontal => ([[-1.0, 0.0], [0.0, 1.0], [0.0, 0.0]], (width as usize, 0)),
    };
    let image_matrix = image_to_pixel_matrix(img);
    transform(&transform_matrix, &image_matrix, canvas, offset);
}

pub fn grayscale(img: &image::RgbImage, canvas: &mut PixelMatrix) {
    let (width, height) = img.dimensions();
    for i in 0..width {
        for j in 0..height {
            let pixel = img.get_pixel(i, j);
            let image::Rgb(rgb) = *pixel;
            let media: u8 = ((rgb[0] as u32 + rgb[1] as u32 + rgb[2] as u32) / 3) as u8;
            let new_pixel = image::Rgb([media, media, media]);
            canvas[i as usize][j as usize] = new_pixel;
        }
    }
}

fn transform_light(contrast: i32, subpixel: u8, brightness: i32) -> u8 {
    let mut out = contrast as i32 * subpixel as i32 + brightness as i32;
    if out > 255 {
        out = 255;
    }
    if out < 0 {
        out = 0;
    }
    out as u8
}

pub fn contrast(img: &image::RgbImage, canvas: &mut PixelMatrix, value: i32) {
    let (width, height) = img.dimensions();
    for i in 0..width {
        for j in 0..height {
            let pixel = img.get_pixel(i, j);
            let image::Rgb(rgb) = *pixel;
            let mut new_rgb = [rgb[0], rgb[1], rgb[2]];
            for k in 0..3 {
                new_rgb[k] = transform_light(value, new_rgb[k], 0);
            }
            let new_pixel = image::Rgb(new_rgb);
            canvas[i as usize][j as usize] = new_pixel;
        }
    }
}

pub fn brightness(img: &image::RgbImage, canvas: &mut PixelMatrix, value: i32) {
    let (width, height) = img.dimensions();
    for i in 0..width {
        for j in 0..height {
            let pixel = img.get_pixel(i, j);
            let image::Rgb(rgb) = *pixel;
            let mut new_rgb = [rgb[0], rgb[1], rgb[2]];
            for k in 0..3 {
                new_rgb[k] = transform_light(1, new_rgb[k], value);
            }
            let new_pixel = image::Rgb(new_rgb);
            canvas[i as usize][j as usize] = new_pixel;
        }
    }
}

pub fn negative(img: &image::RgbImage, canvas: &mut PixelMatrix) {
    let (width, height) = img.dimensions();
    for i in 0..width {
        for j in 0..height {
            let pixel = img.get_pixel(i, j);
            let image::Rgb(rgb) = *pixel;
            let mut new_rgb = [rgb[0], rgb[1], rgb[2]];
            for k in 0..3 {
                new_rgb[k] = 255 - new_rgb[k];
            }
            let new_pixel = image::Rgb(new_rgb);
            canvas[i as usize][j as usize] = new_pixel;
        }
    }
}
