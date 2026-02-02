use image::{DynamicImage, ImageBuffer, Luma, GenericImageView};
use base64::{Engine as _, engine::general_purpose};

/// Procesador de imágenes para impresoras térmicas
pub struct ImageProcessor;

impl ImageProcessor {
    const THRESHOLD: u8 = 127; // Umbral para convertir a blanco y negro

    /// Convierte una imagen base64 a DynamicImage
    pub fn base64_to_image(base64_string: &str) -> Result<DynamicImage, String> {
        // Remover el prefijo data:image si existe
        let image_data = if base64_string.contains(',') {
            base64_string.split(',').nth(1).unwrap_or(base64_string)
        } else {
            base64_string
        };

        // Decodificar base64
        let image_bytes = general_purpose::STANDARD
            .decode(image_data)
            .map_err(|e| format!("Error decoding base64: {}", e))?;

        // Cargar imagen
        image::load_from_memory(&image_bytes)
            .map_err(|e| format!("Error loading image: {}", e))
    }

    /// Redimensiona la imagen manteniendo la relación de aspecto
    pub fn resize_image(img: &DynamicImage, max_width: u32) -> DynamicImage {
        let (width, height) = img.dimensions();

        if width <= max_width {
            return img.clone();
        }

        let new_width = max_width;
        let new_height = ((height as f64) * (max_width as f64) / (width as f64)) as u32;

        img.resize(new_width, new_height, image::imageops::FilterType::Lanczos3)
    }

    /// Convierte la imagen a escala de grises
    pub fn to_grayscale(img: &DynamicImage) -> ImageBuffer<Luma<u8>, Vec<u8>> {
        img.to_luma8()
    }

    /// Convierte la imagen a blanco y negro usando dithering Floyd-Steinberg
    pub fn to_binary_with_dithering(grayscale: &ImageBuffer<Luma<u8>, Vec<u8>>) 
        -> ImageBuffer<Luma<u8>, Vec<u8>> {
        let (width, height) = grayscale.dimensions();
        let mut pixels: Vec<Vec<i32>> = vec![vec![0; width as usize]; height as usize];

        // Copiar valores de píxeles
        for y in 0..height {
            for x in 0..width {
                let pixel = grayscale.get_pixel(x, y)[0];
                pixels[y as usize][x as usize] = pixel as i32;
            }
        }

        let mut binary = ImageBuffer::new(width, height);

        // Aplicar Floyd-Steinberg dithering
        for y in 0..height {
            for x in 0..width {
                let old_pixel = pixels[y as usize][x as usize];
                let new_pixel = if old_pixel < Self::THRESHOLD as i32 { 0 } else { 255 };
                
                binary.put_pixel(x, y, Luma([new_pixel as u8]));
                
                let error = old_pixel - new_pixel;

                // Distribuir el error a píxeles vecinos
                if x + 1 < width {
                    pixels[y as usize][(x + 1) as usize] += error * 7 / 16;
                }
                if y + 1 < height {
                    if x > 0 {
                        pixels[(y + 1) as usize][(x - 1) as usize] += error * 3 / 16;
                    }
                    pixels[(y + 1) as usize][x as usize] += error * 5 / 16;
                    if x + 1 < width {
                        pixels[(y + 1) as usize][(x + 1) as usize] += error / 16;
                    }
                }
            }
        }

        binary
    }

    /// Convierte la imagen a blanco y negro sin dithering (umbral simple)
    pub fn to_binary_simple(grayscale: &ImageBuffer<Luma<u8>, Vec<u8>>) 
        -> ImageBuffer<Luma<u8>, Vec<u8>> {
        let (width, height) = grayscale.dimensions();
        let mut binary = ImageBuffer::new(width, height);

        for y in 0..height {
            for x in 0..width {
                let pixel = grayscale.get_pixel(x, y)[0];
                let binary_value = if pixel < Self::THRESHOLD { 0 } else { 255 };
                binary.put_pixel(x, y, Luma([binary_value]));
            }
        }

        binary
    }

    /// Convierte la imagen binaria a bytes para ESC/POS
    /// ESC/POS usa: bit=1 para píxel negro (imprimir), bit=0 para píxel blanco (no imprimir)
    /// Los bytes se empaquetan en formato big-endian (MSB primero)
    pub fn image_to_bytes(binary: &ImageBuffer<Luma<u8>, Vec<u8>>) -> Vec<u8> {
        let (width, height) = binary.dimensions();
        let byte_width = ((width + 7) / 8) as usize;
        let mut image_data = vec![0u8; byte_width * height as usize];

        for y in 0..height {
            for x in 0..width {
                let pixel = binary.get_pixel(x, y)[0];
                
                // Pixel negro (valor 0) = imprimir (bit 1)
                // Pixel blanco (valor 255) = no imprimir (bit 0)
                let is_black = pixel < Self::THRESHOLD;

                if is_black {
                    let byte_index = (y as usize) * byte_width + (x as usize / 8);
                    let bit_position = 7 - (x % 8);
                    image_data[byte_index] |= 1 << bit_position;
                }
            }
        }

        image_data
    }

    /// Procesa una imagen base64 completa: resize, grayscale, binary
    pub fn process_image(base64_image: &str, max_width: u32, use_dithering: bool) 
        -> Result<ImageBuffer<Luma<u8>, Vec<u8>>, String> {
        let original = Self::base64_to_image(base64_image)?;
        let resized = Self::resize_image(&original, max_width);
        let grayscale = Self::to_grayscale(&resized);
        
        let binary = if use_dithering {
            Self::to_binary_with_dithering(&grayscale)
        } else {
            Self::to_binary_simple(&grayscale)
        };

        Ok(binary)
    }

}