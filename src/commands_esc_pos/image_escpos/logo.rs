use super::image_mode::ImageMode;
use super::image_processor::ImageProcessor;
use crate::models::print_sections::Logo as LogoSection;

/// Clase para manejar logos guardados en la memoria de la impresora.
/// Algunas impresoras ESC/POS permiten guardar logos en memoria NV (Non-Volatile).
#[derive(Debug, Clone)]
pub struct Logo {
    key_code: u8,
    mode: ImageMode,
}

impl Logo {
    /// Crea un nuevo logo con el código de clave especificado
    pub fn new(key_code: u8) -> Self {
        Self {
            key_code,
            mode: ImageMode::Normal,
        }
    }

    /// Establece el modo de impresión del logo
    pub fn set_mode(mut self, mode: ImageMode) -> Self {
        self.mode = mode;
        self
    }

    /// Comando para imprimir logo guardado en memoria NV
    /// FS p n m - Print NV bit image
    pub fn get_print_command(&self) -> Vec<u8> {
        vec![
            0x1C,              // FS
            0x70,              // p
            self.key_code,     // n (key code)
            self.mode.value(), // m (mode)
        ]
    }
}

/// Key code fijo usado al guardar el logo en memoria NV (`FS q` solo admite n = 1).
pub const NV_LOGO_KEY_CODE: u8 = 1;

/// Procesa sección Logo del modelo de impresión.
///
/// Si `set_logo` está presente, tiene prioridad: se descarga la imagen a la memoria
/// NV de la impresora (`FS q`) y se ignoran `key_code`/`mode`. En caso contrario se
/// imprime el logo previamente guardado (`FS p`).
pub fn process_section(logo: &LogoSection, paper_width_pixels: i32) -> Result<Vec<u8>, String> {
    // `set_logo` tiene prioridad: guardar en memoria NV e ignorar el resto de campos.
    if let Some(image) = &logo.set_logo {
        return Logo::get_define_command(image, paper_width_pixels);
    }

    let mode = match logo.mode.as_deref().unwrap_or("normal") {
        "normal" => ImageMode::Normal,
        "double_width" => ImageMode::DoubleWidth,
        "double_height" => ImageMode::DoubleHeight,
        "quadruple" => ImageMode::Quadruple,
        _ => ImageMode::Normal,
    };

    let esc_pos_logo = Logo::new(logo.key_code.unwrap_or(NV_LOGO_KEY_CODE)).set_mode(mode);
    let mut data = esc_pos_logo.get_print_command();
    data.extend_from_slice(b"\n");
    Ok(data)
}

impl Logo {
    /// Genera el comando `FS q` para guardar una imagen como logo en la memoria NV.
    ///
    /// La imagen (base64) se procesa igual que una imagen normal (resize al ancho del
    /// papel, escala de grises, binarización con dithering) y luego se empaqueta en el
    /// **formato de columnas** que exige `FS q`:
    ///
    /// `FS q n xL xH yL yH d1...dk`
    /// - `n = 1` (única clave admitida por el comando)
    /// - horizontal = `(xL + xH*256) * 8` puntos → `xL/xH` son el ancho en **bytes**
    /// - vertical   = `(yL + yH*256) * 8` puntos → `yL/yH` son el alto en **bytes**
    /// - `k = x_bytes * y_bytes * 8`, con los datos en columnas: por cada columna de
    ///   puntos se emiten `y_bytes` bytes (8 puntos verticales cada uno, MSB arriba),
    ///   empezando por la columna izquierda.
    pub fn get_define_command(
        image: &crate::models::print_sections::Image,
        paper_width_pixels: i32,
    ) -> Result<Vec<u8>, String> {
        if image.data.is_empty() {
            return Err("Logo image data cannot be empty".to_string());
        }

        let max_width = if image.max_width > paper_width_pixels || image.max_width <= 0 {
            paper_width_pixels as u32
        } else {
            image.max_width as u32
        };

        let binary = ImageProcessor::process_image(&image.data, max_width, image.dithering)?;
        let (width, height) = (binary.width(), binary.height());

        let x_bytes = ((width + 7) / 8) as u16;
        let y_bytes = ((height + 7) / 8) as u16;
        if x_bytes == 0 || y_bytes == 0 {
            return Err("Logo image has no printable content".to_string());
        }

        let mut output = vec![
            0x1C, // FS
            0x71, // q
            NV_LOGO_KEY_CODE,
            (x_bytes & 0xFF) as u8,
            (x_bytes >> 8) as u8,
            (y_bytes & 0xFF) as u8,
            (y_bytes >> 8) as u8,
        ];

        // Formato de columnas: para cada columna de puntos (padded a múltiplo de 8),
        // emitir `y_bytes` bytes verticales (8 puntos por byte, MSB = punto superior).
        let padded_width = x_bytes as u32 * 8;
        for col in 0..padded_width {
            for band in 0..y_bytes as u32 {
                let mut byte = 0u8;
                for bit in 0..8u32 {
                    let row = band * 8 + bit;
                    if col < width && row < height {
                        // Píxel negro (< umbral) => punto impreso (bit 1).
                        if binary.get_pixel(col, row)[0] < 127 {
                            byte |= 1 << (7 - bit);
                        }
                    }
                }
                output.push(byte);
            }
        }

        Ok(output)
    }
}
