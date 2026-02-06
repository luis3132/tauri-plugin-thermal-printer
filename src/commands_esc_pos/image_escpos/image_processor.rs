use image::{DynamicImage, ImageBuffer, Luma, GenericImageView, Pixel};
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
    /// Maneja correctamente la transparencia con fondo blanco
    pub fn resize_image(img: &DynamicImage, max_width: u32) -> DynamicImage {
        let (width, height) = img.dimensions();

        if width <= max_width {
            return img.clone();
        }

        let new_width = max_width;
        let new_height = ((height as f64) * (max_width as f64) / (width as f64)) as u32;

        // Si tiene transparencia, crear imagen con fondo blanco
        if img.color().has_alpha() {
            let mut rgba_img = img.to_rgba8();
            
            // Aplicar fondo blanco a píxeles transparentes
            for pixel in rgba_img.pixels_mut() {
                let alpha = pixel[3];
                if alpha < 255 {
                    // Blend con fondo blanco
                    let alpha_f = alpha as f32 / 255.0;
                    pixel[0] = ((pixel[0] as f32 * alpha_f) + (255.0 * (1.0 - alpha_f))) as u8;
                    pixel[1] = ((pixel[1] as f32 * alpha_f) + (255.0 * (1.0 - alpha_f))) as u8;
                    pixel[2] = ((pixel[2] as f32 * alpha_f) + (255.0 * (1.0 - alpha_f))) as u8;
                    pixel[3] = 255;
                }
            }
            
            let img_no_alpha = DynamicImage::ImageRgba8(rgba_img);
            img_no_alpha.resize(new_width, new_height, image::imageops::FilterType::Lanczos3)
        } else {
            img.resize(new_width, new_height, image::imageops::FilterType::Lanczos3)
        }
    }

    /// Convierte la imagen a escala de grises
    /// Maneja correctamente la transparencia convirtiéndola a blanco
    pub fn to_grayscale(img: &DynamicImage) -> ImageBuffer<Luma<u8>, Vec<u8>> {
        let (width, height) = img.dimensions();
        let mut grayscale = ImageBuffer::new(width, height);

        // Si tiene canal alpha, manejarlo
        if img.color().has_alpha() {
            for y in 0..height {
                for x in 0..width {
                    let pixel = img.get_pixel(x, y);
                    let channels = pixel.channels();
                    
                    let (r, g, b, a) = if channels.len() >= 4 {
                        (channels[0], channels[1], channels[2], channels[3])
                    } else {
                        (channels[0], channels[1], channels[2], 255)
                    };

                    // Si es muy transparente, tratarlo como blanco
                    if a < 128 {
                        grayscale.put_pixel(x, y, Luma([255u8]));
                    } else {
                        // Convertir a escala de grises usando luminosidad
                        let gray = (0.299 * r as f32 + 0.587 * g as f32 + 0.114 * b as f32) as u8;
                        grayscale.put_pixel(x, y, Luma([gray]));
                    }
                }
            }
        } else {
            // Sin transparencia, conversión estándar
            return img.to_luma8();
        }

        grayscale
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
                // Umbral: valores oscuros (< 127) = negro (0), valores claros (>= 127) = blanco (255)
                let binary_value = if pixel < Self::THRESHOLD { 0 } else { 255 };
                binary.put_pixel(x, y, Luma([binary_value]));
            }
        }

        binary
    }

    /// Convierte la imagen binaria a bytes para ESC/POS
    /// Solo los píxeles negros se imprimen (bit=1)
    /// Los píxeles blancos no se imprimen (bit=0)
    /// Los bytes se empaquetan en formato big-endian (MSB primero)
    pub fn image_to_bytes(binary: &ImageBuffer<Luma<u8>, Vec<u8>>) -> Vec<u8> {
        let (width, height) = binary.dimensions();
        let byte_width = ((width + 7) / 8) as usize;
        let mut image_data = vec![0u8; byte_width * height as usize];

        for y in 0..height {
            for x in 0..width {
                let pixel = binary.get_pixel(x, y)[0];
                
                // Solo marcar como negro (bit=1) si el píxel es oscuro
                // Pixel negro (valor < 127) = imprimir (bit 1)
                // Pixel blanco (valor >= 127) = no imprimir (bit 0)
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