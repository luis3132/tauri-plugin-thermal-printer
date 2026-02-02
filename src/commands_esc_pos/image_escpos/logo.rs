use super::image_mode::ImageMode;
use super::image_processor::ImageProcessor;

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

    /// Comando para guardar una imagen como logo en memoria NV
    /// Nota: Este comando requiere que la imagen ya esté procesada
    /// 
    /// # Arguments
    /// * `key_code` - Código de clave para el logo (1-255)
    /// * `image_data` - Datos de imagen procesados
    /// * `width` - Ancho de la imagen en píxeles
    /// * `height` - Alto de la imagen en píxeles
    pub fn get_define_command(
        key_code: u8,
        image_data: &[u8],
        width: u32,
        height: u32,
    ) -> Vec<u8> {
        let mut output = Vec::new();

        // FS q n [xL xH yL yH d1...dk]
        output.push(0x1C); // FS
        output.push(0x71); // q
        output.push(key_code); // n

        let width_bytes = ((width + 7) / 8) as u16;
        let x_l = (width_bytes & 0xFF) as u8;
        let x_h = ((width_bytes >> 8) & 0xFF) as u8;
        let y_l = (height & 0xFF) as u8;
        let y_h = ((height >> 8) & 0xFF) as u8;

        output.push(x_l);
        output.push(x_h);
        output.push(y_l);
        output.push(y_h);
        output.extend_from_slice(image_data);

        output
    }

    /// Comando para guardar un logo desde base64 en memoria NV
    /// 
    /// # Arguments
    /// * `key_code` - Código de clave para el logo
    /// * `base64_image` - Imagen en formato base64
    /// * `max_width` - Ancho máximo en píxeles
    /// * `use_dithering` - Usar dithering Floyd-Steinberg
    pub fn define_from_base64(
        key_code: u8,
        base64_image: &str,
        max_width: u32,
        use_dithering: bool,
    ) -> Result<Vec<u8>, String> {
        // Procesar la imagen
        let processed_image = ImageProcessor::process_image(
            base64_image,
            max_width,
            use_dithering,
        )?;

        let (width, height) = processed_image.dimensions();

        // Convertir a bytes
        let image_data = ImageProcessor::image_to_bytes(&processed_image);

        // Generar comando
        Ok(Self::get_define_command(key_code, &image_data, width, height))
    }

    /// Comando para borrar un logo de la memoria NV
    /// FS q n - Delete NV graphics
    pub fn get_delete_command(key_code: u8) -> Vec<u8> {
        vec![
            0x1C, // FS
            0x71, // q
            key_code, // n
            0x00, // Enviar 0 bytes para borrar
        ]
    }

    /// Comando para borrar todos los logos de la memoria NV
    pub fn get_delete_all_command() -> Vec<u8> {
        vec![
            0x1C, // FS
            0x71, // q
            0x00, // n = 0 (todos)
        ]
    }

    /// Getters
    pub fn key_code(&self) -> u8 {
        self.key_code
    }

    pub fn mode(&self) -> ImageMode {
        self.mode
    }
}