use crate::commands_esc_pos::text::text_type::get_styles_diff;
use crate::models::print_sections::{Aztec as AztecSection, GlobalStyles};

/// Constructor de comandos para códigos Aztec
///
/// NOTA: Aztec Code no es soportado por todas las impresoras térmicas.
/// Funciona principalmente en modelos Epson avanzados (TM-T88VI y superiores).
#[derive(Debug, Clone)]
pub struct Aztec {
    data: String,
    mode: u8, // 0 = full range, 1 = compact
    layers: u8, // 0 = automático, 1-32
    size: u8, // Tamaño del módulo (2-16)
    error_correction: u8, // Nivel de corrección (% de la capacidad, 5-95)
}

impl Aztec {
    /// Crea un nuevo código Aztec con valores por defecto
    ///
    /// # Arguments
    /// * `data` - Datos a codificar en el Aztec
    pub fn new(data: String) -> Self {
        Self {
            data,
            mode: 0,   // full range
            layers: 0, // automático
            size: 3,
            error_correction: 23, // valor por defecto Epson
        }
    }

    /// Establece el modo (0 = full range, 1 = compact)
    pub fn set_mode(mut self, mode: u8) -> Self {
        if mode <= 1 {
            self.mode = mode;
        }
        self
    }

    /// Establece el número de capas de datos (0 = automático, 1-32)
    pub fn set_layers(mut self, layers: u8) -> Self {
        if layers <= 32 {
            self.layers = layers;
        }
        self
    }

    /// Establece el tamaño del módulo (2-16)
    pub fn set_size(mut self, size: u8) -> Self {
        if (2..=16).contains(&size) {
            self.size = size;
        }
        self
    }

    /// Establece el nivel de corrección de error (% de capacidad, 5-95)
    pub fn set_error_correction(mut self, error_correction: u8) -> Self {
        if (5..=95).contains(&error_correction) {
            self.error_correction = error_correction;
        }
        self
    }

    /// Genera el comando ESC/POS para Aztec
    ///
    /// NOTA: Aztec Code no es soportado por todas las impresoras térmicas.
    pub fn get_command(&self) -> Vec<u8> {
        let mut output = Vec::new();
        let data_bytes = self.data.as_bytes();
        let data_length = (data_bytes.len() + 3) as u16;
        let p_l = (data_length & 0xFF) as u8;
        let p_h = ((data_length >> 8) & 0xFF) as u8;

        // Aztec usa cn = 53 (0x35)

        // Función 65 (0x41) - Modo y número de capas
        output.extend_from_slice(&[
            0x1D,        // GS
            0x28,        // (
            0x6B,        // k
            0x04,        // pL
            0x00,        // pH
            0x35,        // cn = 53 (Aztec)
            0x41,        // fn = 65 (0x41 = modo/capas)
            self.mode,   // n1 = modo (0 full range, 1 compact)
            self.layers, // n2 = capas (0 = automático)
        ]);

        // Función 67 (0x43) - Tamaño del módulo
        output.extend_from_slice(&[
            0x1D,      // GS
            0x28,      // (
            0x6B,      // k
            0x03,      // pL
            0x00,      // pH
            0x35,      // cn = 53
            0x43,      // fn = 67 (0x43 = tamaño)
            self.size, // n (2-16)
        ]);

        // Función 69 (0x45) - Corrección de error
        output.extend_from_slice(&[
            0x1D,                  // GS
            0x28,                  // (
            0x6B,                  // k
            0x03,                  // pL
            0x00,                  // pH
            0x35,                  // cn = 53
            0x45,                  // fn = 69 (0x45 = corrección)
            self.error_correction, // n (5-95)
        ]);

        // Función 80 (0x50) - Almacenar datos en el buffer
        output.extend_from_slice(&[
            0x1D, // GS
            0x28, // (
            0x6B, // k
            p_l,  // pL
            p_h,  // pH
            0x35, // cn = 53
            0x50, // fn = 80 (0x50 = almacenar)
            0x30, // m = 48
        ]);
        output.extend_from_slice(data_bytes); // datos

        // Función 81 (0x51) - Imprimir el símbolo Aztec
        output.extend_from_slice(&[
            0x1D, // GS
            0x28, // (
            0x6B, // k
            0x03, // pL
            0x00, // pH
            0x35, // cn = 53
            0x51, // fn = 81 (0x51 = imprimir)
            0x30, // m = 48
        ]);

        output
    }
}

/// Procesa sección Aztec del modelo de impresión
pub fn process_section(aztec: &AztecSection, current_styles: &GlobalStyles) -> Result<Vec<u8>, String> {
    if aztec.data.is_empty() {
        return Err("Aztec data cannot be empty".to_string());
    }

    let esc_pos_aztec = Aztec::new(aztec.data.clone())
        .set_mode(aztec.mode)
        .set_layers(aztec.layers)
        .set_size(aztec.size)
        .set_error_correction(aztec.error_correction);

    let mut data = Vec::new();
    if let Some(ref align) = aztec.align {
        let mut temp_styles = current_styles.clone();
        temp_styles.align = Some(align.clone());
        data.extend_from_slice(&get_styles_diff(current_styles, &temp_styles));
        data.extend_from_slice(&esc_pos_aztec.get_command());
        data.extend_from_slice(b"\n");
        data.extend_from_slice(&get_styles_diff(&temp_styles, current_styles));
    } else {
        data.extend_from_slice(&esc_pos_aztec.get_command());
        data.extend_from_slice(b"\n");
    }

    Ok(data)
}
