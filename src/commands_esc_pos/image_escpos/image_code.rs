
use super::image_alignment::ImageAlignment;
use super::image_mode::ImageMode;
use super::image_processor::ImageProcessor;

/// Constructor de comandos para imágenes
#[derive(Debug, Clone)]
pub struct Image {
    base64_image: String,
    alignment: ImageAlignment,
    mode: ImageMode,
    max_width: u32,
    use_dithering: bool,
}

impl Image {
    /// Crea una nueva imagen
    pub fn new(base64_image: &str, max_width: u32) -> Result<Self, String> {
        Ok(Self {
            base64_image: base64_image.to_string(),
            alignment: ImageAlignment::Center,
            mode: ImageMode::Normal,
            max_width: max_width,
            use_dithering: true,
        })
    }

    /// Establece la alineación
    pub fn set_alignment(mut self, alignment: ImageAlignment) -> Self {
        self.alignment = alignment;
        self
    }

    /// Establece el modo de impresión
    pub fn set_mode(mut self, mode: ImageMode) -> Self {
        self.mode = mode;
        self
    }

    /// Establece si usar dithering
    pub fn set_use_dithering(mut self, use_dithering: bool) -> Self {
        self.use_dithering = use_dithering;
        self
    }

    /// Genera el comando ESC/POS para imprimir la imagen
    /// Usa el comando GS v 0 (raster bit image)
    pub fn get_command(&self) -> Result<Vec<u8>, String> {
        let mut output = Vec::new();

        // Procesar la imagen
        let processed_image = ImageProcessor::process_image(
            &self.base64_image,
            self.max_width,
            self.use_dithering
        )?;

        let (width, height) = processed_image.dimensions();

        // Convertir imagen a bytes
        let image_data = ImageProcessor::image_to_bytes(&processed_image);

        // Establecer alineación
        output.extend_from_slice(&[0x1B, 0x61, self.alignment.value()]);

        // Calcular ancho en bytes (múltiplo de 8)
        let width_bytes = ((width + 7) / 8) as u16;
        let x_l = (width_bytes & 0xFF) as u8;
        let x_h = ((width_bytes >> 8) & 0xFF) as u8;
        let y_l = (height & 0xFF) as u8;
        let y_h = ((height >> 8) & 0xFF) as u8;

        // Comando para imprimir imagen raster: GS v 0 m xL xH yL yH d1...dk
        output.push(0x1D); // GS
        output.push(0x76); // v
        output.push(0x00); // 0 (modo raster)
        output.push(self.mode.value()); // m
        output.push(x_l);
        output.push(x_h);
        output.push(y_l);
        output.push(y_h);
        output.extend_from_slice(&image_data);

        // Restaurar alineación a la izquierda
        output.extend_from_slice(&[0x1B, 0x61, 0x00]);

        Ok(output)
    }

    // /// Método alternativo usando modo bit image (ESC *)
    // /// Útil para impresoras más antiguas que no soportan GS v 0
    // /// NOTA: Este método puede no ser compatible con todas las impresoras móviles
    // pub fn get_command_bit_image(&self) -> Result<Vec<u8>, String> {
    //     let mut output = Vec::new();

    //     let processed_image = ImageProcessor::process_image(
    //         &self.base64_image,
    //         self.max_width,
    //         self.use_dithering
    //     )?;

    //     let (width, height) = processed_image.dimensions();
    //     let width_bytes = ((width + 7) / 8) as usize;
    //     let image_data = ImageProcessor::image_to_bytes(&processed_image);

    //     // Establecer alineación
    //     output.extend_from_slice(&[0x1B, 0x61, self.alignment.value()]);

    //     // Imprimir línea por línea usando ESC * (Select bit-image mode)
    //     for y in 0..height {
    //         // ESC * m nL nH d1...dk
    //         output.push(0x1B); // ESC
    //         output.push(0x2A); // *
    //         output.push(33);   // m = 33 (24-dot double-density)

    //         let n_l = (width & 0xFF) as u8;
    //         let n_h = ((width >> 8) & 0xFF) as u8;
    //         output.push(n_l);
    //         output.push(n_h);

    //         // Escribir datos de la línea
    //         let start_index = (y as usize) * width_bytes;
    //         let end_index = (start_index + width_bytes).min(image_data.len());
    //         output.extend_from_slice(&image_data[start_index..end_index]);

    //         // Nueva línea
    //         output.push(0x0A); // LF
    //     }

    //     // Restaurar alineación
    //     output.extend_from_slice(&[0x1B, 0x61, 0x00]);

    //     Ok(output)
    // }
}