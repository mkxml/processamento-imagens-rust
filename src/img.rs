extern crate image;

use image::*;
use std::collections::HashMap;

// Tipo que representa uma matriz de pixels RGB
pub type PixelMatrix = Vec<Vec<Rgb<u8>>>;

// Tipo que representa uma matriz de posições com vetores [x, y]
pub type PositionMatrix = [[f64; 2]; 3];

pub type FilterMatrix = [[i32; 3]; 3];

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

pub fn grayscale(img: &mut image::RgbImage) {
    let (width, height) = img.dimensions();
    for i in 0..width {
        for j in 0..height {
            let pixel = img.get_pixel(i, j);
            let image::Rgb(rgb) = *pixel;
            let media: u8 = ((rgb[0] as u32 + rgb[1] as u32 + rgb[2] as u32) / 3) as u8;
            let new_pixel = image::Rgb([media, media, media]);
            let pixel = img.get_pixel_mut(i, j);
            *pixel = new_pixel;
        }
    }
}

fn transform_light(contrast: f32, subpixel: u8, brightness: i32) -> u8 {
    let mut out = contrast * subpixel as f32 + brightness as f32;
    if out > 255.0 {
        out = 255.0;
    }
    if out < 0.0 {
        out = 0.0;
    }
    out as u8
}

pub fn contrast(img: &mut image::RgbImage, value: f32) {
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
            let img_pixel = img.get_pixel_mut(i, j);
            *img_pixel = new_pixel;
        }
    }
}

pub fn brightness(img: &mut image::RgbImage, value: i32) {
    let (width, height) = img.dimensions();
    for i in 0..width {
        for j in 0..height {
            let pixel = img.get_pixel(i, j);
            let image::Rgb(rgb) = *pixel;
            let mut new_rgb = [rgb[0], rgb[1], rgb[2]];
            for k in 0..3 {
                new_rgb[k] = transform_light(1.0, new_rgb[k], value);
            }
            let new_pixel = image::Rgb(new_rgb);
            let img_pixel = img.get_pixel_mut(i, j);
            *img_pixel = new_pixel;
        }
    }
}

pub fn negative(img: &mut image::RgbImage) {
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
            let img_pixel = img.get_pixel_mut(i, j);
            *img_pixel = new_pixel;
        }
    }
}

fn sum_filter_matrix(matrix: FilterMatrix) -> i32 {
    let mut sum = 0;
    for i in 0..matrix.len() {
        for j in 0..matrix[0].len() {
            sum += matrix[i][j];
        }
    }
    sum
}

fn apply_average_filter_mask(
    img: &image::RgbImage,
    filter: FilterMatrix,
    x: u32,
    y: u32,
    z: i32,
) -> [u8; 3] {
    let mut r_sum = 0;
    let mut g_sum = 0;
    let mut b_sum = 0;
    for i in 0..filter.len() {
        for j in 0..filter[0].len() {
            let x_search = x as i32 + (i as i32 - 1);
            let y_search = y as i32 + (j as i32 - 1);
            let pixel = img.get_pixel(x_search as u32, y_search as u32);
            let image::Rgb(rgb) = *pixel;
            r_sum += rgb[0] as i32 * filter[i][j];
            g_sum += rgb[1] as i32 * filter[i][j];
            b_sum += rgb[2] as i32 * filter[i][j];
        }
    }
    [(r_sum / z) as u8, (g_sum / z) as u8, (b_sum / z) as u8]
}

fn find_mode(map: &HashMap<u8, u8>, original_map: &HashMap<u8, (u8, u8, u8)>) -> [u8; 3] {
    let mut count: u8 = 0;
    let mut gray_level_mode: u8 = 0;
    for key in map.keys() {
        let test = map.get(key).unwrap();
        if test > &count {
            count = *test;
            gray_level_mode = *key;
        }
    }
    let mode = original_map.get(&gray_level_mode).unwrap();
    [mode.0, mode.1, mode.2]
}

fn find_median(list: &mut Vec<u8>, original_map: &HashMap<u8, (u8, u8, u8)>) -> [u8; 3] {
    list.sort();
    let idx = (list.len() / 2) as usize;
    let median = original_map.get(&list[idx]).unwrap();
    [median.0, median.1, median.2]
}

pub fn average_filter(img: &mut image::RgbImage, canvas: &mut PixelMatrix) {
    let filter: FilterMatrix = [[1, 1, 1], [1, 1, 1], [1, 1, 1]];
    let (width, height) = img.dimensions();
    let z = sum_filter_matrix(filter);
    for x in 1..(width - 1) {
        for y in 1..(height - 1) {
            let new_rgb = apply_average_filter_mask(img, filter, x, y, z);
            let new_pixel = image::Rgb(new_rgb);
            canvas[x as usize][y as usize] = new_pixel;
        }
    }
}

pub fn gaussian_filter(img: &mut image::RgbImage) {
    let filter: FilterMatrix = [[1, 2, 1], [2, 4, 2], [1, 2, 1]];
    let (width, height) = img.dimensions();
    let z = sum_filter_matrix(filter);
    for x in 1..(width - 1) {
        for y in 1..(height - 1) {
            let new_rgb = apply_average_filter_mask(img, filter, x, y, z);
            let new_pixel = image::Rgb(new_rgb);
            let pixel = img.get_pixel_mut(x, y);
            *pixel = new_pixel;
        }
    }
}

pub fn mode_filter(img: &mut image::RgbImage, canvas: &mut PixelMatrix) {
    let filter: FilterMatrix = [[1, 1, 1], [1, 1, 1], [1, 1, 1]];
    let (width, height) = img.dimensions();
    for x in 1..(width - 1) {
        for y in 1..(height - 1) {
            let mut gray_levels: HashMap<u8, u8> = HashMap::new();
            let mut original_pixels: HashMap<u8, (u8, u8, u8)> = HashMap::new();
            for i in 0..filter.len() {
                for j in 0..filter[0].len() {
                    let x_search = x as i32 + (i as i32 - 1);
                    let y_search = y as i32 + (j as i32 - 1);
                    let pixel = img.get_pixel(x_search as u32, y_search as u32);
                    let image::Rgb(rgb) = *pixel;
                    let gray_level = ((rgb[0] as u32 + rgb[1] as u32 + rgb[2] as u32) / 3) as u8;
                    let pixel_count = match gray_levels.get(&gray_level) {
                        None => 0,
                        Some(v) => *v,
                    };
                    gray_levels.insert(gray_level, pixel_count + 1);
                    original_pixels.insert(gray_level, (rgb[0], rgb[1], rgb[2]));
                }
            }
            let new_rgb = find_mode(&gray_levels, &original_pixels);
            let new_pixel = image::Rgb(new_rgb);
            canvas[x as usize][y as usize] = new_pixel;
        }
    }
}

pub fn median_filter(img: &mut image::RgbImage) {
    let filter: FilterMatrix = [[1, 1, 1], [1, 1, 1], [1, 1, 1]];
    let (width, height) = img.dimensions();
    for x in 1..(width - 1) {
        for y in 1..(height - 1) {
            let mut gray_levels: Vec<u8> = Vec::new();
            let mut original_pixels: HashMap<u8, (u8, u8, u8)> = HashMap::new();
            for i in 0..filter.len() {
                for j in 0..filter[0].len() {
                    let x_search = x as i32 + (i as i32 - 1);
                    let y_search = y as i32 + (j as i32 - 1);
                    let pixel = img.get_pixel(x_search as u32, y_search as u32);
                    let image::Rgb(rgb) = *pixel;
                    let gray_level = ((rgb[0] as u32 + rgb[1] as u32 + rgb[2] as u32) / 3) as u8;
                    gray_levels.push(gray_level);
                    original_pixels.insert(gray_level, (rgb[0], rgb[1], rgb[2]));
                }
            }
            let new_rgb = find_median(&mut gray_levels, &original_pixels);
            let new_pixel = image::Rgb(new_rgb);
            let pixel = img.get_pixel_mut(x, y);
            *pixel = new_pixel;
        }
    }
}

pub fn border_detection(img: &image::RgbImage, canvas: &mut PixelMatrix, threshold: u32) {
    let kernel: FilterMatrix = [[0, 0, 0], [0, 0, -1], [0, 1, 0]];
    let (width, height) = img.dimensions();
    for x in 1..(width - 1) {
        for y in 1..(height - 1) {
            let mut gs = [0, 0, 0];
            for i in 0..3 {
                for j in 0..3 {
                    let x_search = x as i32 + (i as i32 - 1);
                    let y_search = y as i32 + (j as i32 - 1);
                    let pixel = img.get_pixel(x_search as u32, y_search as u32);
                    let image::Rgb(rgb) = *pixel;
                    gs[0] += rgb[0] as i32 * kernel[i][j];
                    gs[1] += rgb[1] as i32 * kernel[i][j];
                    gs[2] += rgb[2] as i32 * kernel[i][j];
                }
            }
            let mut new_rgb = [0, 0, 0];
            for k in 0..3 {
                let g = i32::pow(gs[k], 2) + i32::pow(gs[k], 2);
                let g_root = (g as f64).sqrt();
                new_rgb[k] = 0;
                if g_root as u32 > threshold {
                    new_rgb[k] = 255;
                }
            }
            let new_pixel = image::Rgb(new_rgb);
            canvas[x as usize][y as usize] = new_pixel;
        }
    }
}

pub fn threshold(img: &image::RgbImage, canvas: &mut PixelMatrix, threshold: u8) {
    let (width, height) = img.dimensions();
    for x in 0..width {
        for y in 0..height {
            let pixel = img.get_pixel(x, y);
            let image::Rgb(rgb) = *pixel;
            let gray_level = ((rgb[0] as u32 + rgb[1] as u32 + rgb[2] as u32) / 3) as u8;
            let mut new_rgb = [0, 0, 0];
            if gray_level > threshold {
                new_rgb = [255, 255, 255];
            }
            let new_pixel = image::Rgb(new_rgb);
            canvas[x as usize][y as usize] = new_pixel;
        }
    }
}

pub fn dilation(img: &image::RgbImage, canvas: &mut PixelMatrix, mask: &FilterMatrix) {
    let (width, height) = img.dimensions();
    for x in 1..(width - 1) {
        for y in 1..(height - 1) {
            let mut value = 0 as u8;
            for i in 0..mask.len() {
                for j in 0..mask[0].len() {
                    let x_search = x as i32 + (i as i32 - 1);
                    let y_search = y as i32 + (j as i32 - 1);
                    let pixel = img.get_pixel(x_search as u32, y_search as u32);
                    let image::Rgb(rgb) = *pixel;
                    let gray_level = ((rgb[0] as u32 + rgb[1] as u32 + rgb[2] as u32) / 3) as u8;
                    let mut pixel_value = gray_level as i32 + mask[i][j];
                    if pixel_value > 255 {
                        pixel_value = 255;
                    }
                    if pixel_value > value as i32 {
                        value = pixel_value as u8;
                    }
                }
            }
            let new_rgb = [value, value, value];
            let new_pixel = image::Rgb(new_rgb);
            canvas[x as usize][y as usize] = new_pixel;
        }
    }
}

pub fn erosion(img: &image::RgbImage, canvas: &mut PixelMatrix, mask: &FilterMatrix) {
    let (width, height) = img.dimensions();
    for x in 1..(width - 1) {
        for y in 1..(height - 1) {
            let mut value = 255 as u8;
            for i in 0..mask.len() {
                for j in 0..mask[0].len() {
                    let x_search = x as i32 + (i as i32 - 1);
                    let y_search = y as i32 + (j as i32 - 1);
                    let pixel = img.get_pixel(x_search as u32, y_search as u32);
                    let image::Rgb(rgb) = *pixel;
                    let gray_level = ((rgb[0] as u32 + rgb[1] as u32 + rgb[2] as u32) / 3) as u8;
                    let mut pixel_value = gray_level as i32 - mask[i][j];
                    if pixel_value < 0 {
                        pixel_value = 0;
                    }
                    if pixel_value < value as i32 {
                        value = pixel_value as u8;
                    }
                }
            }
            let new_rgb = [value, value, value];
            let new_pixel = image::Rgb(new_rgb);
            canvas[x as usize][y as usize] = new_pixel;
        }
    }
}
