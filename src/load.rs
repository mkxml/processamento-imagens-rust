pub fn open_image(path: &str) -> image::RgbImage {
    let output: image::DynamicImage = match image::open(path) {
        Ok(img) => img,
        Err(err) => {
            panic!("Erro ao abrir a imagem: {:?}. Erro: {:?}", path, err);
        }
    };
    output.into_rgb()
}
