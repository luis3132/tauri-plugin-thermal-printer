use crate::commands_esc_pos::text::text_type::get_styles_diff;
use crate::models::print_sections::{GlobalStyles, MaxiCode as MaxiCodeSection};

/// Constructor de comandos para códigos MaxiCode
///
/// NOTA: MaxiCode no es soportado por todas las impresoras térmicas.
/// Requiere firmware compatible (modelos Epson avanzados). El símbolo tiene un
/// tamaño físico fijo (no admite tamaño de módulo).
#[derive(Debug, Clone)]
pub struct MaxiCode {
    data: String,
    mode: u8, // Modo 2-6
}

impl MaxiCode {
    /// Crea un nuevo MaxiCode con valores por defecto (modo 4)
    pub fn new(data: String) -> Self {
        Self { data, mode: 4 }
    }

    /// Establece el modo del MaxiCode (2-6). El modo 4 es el estándar general.
    pub fn set_mode(mut self, mode: u8) -> Self {
        if (2..=6).contains(&mode) {
            self.mode = mode;
        }
        self
    }

    /// Genera el comando ESC/POS para MaxiCode
    ///
    /// NOTA: no soportado por todas las impresoras térmicas.
    pub fn get_command(&self) -> Vec<u8> {
        let mut output = Vec::new();
        let data_bytes = self.data.as_bytes();
        let data_length = (data_bytes.len() + 3) as u16;
        let p_l = (data_length & 0xFF) as u8;
        let p_h = ((data_length >> 8) & 0xFF) as u8;

        // MaxiCode usa cn = 50 (0x32).
        // El parámetro n de la función 67 codifica el modo como 0x32-0x36 (modo 2-6).
        let mode_byte = 0x30 + self.mode;

        // Función 67 (0x43) - Seleccionar el modo
        output.extend_from_slice(&[
            0x1D,      // GS
            0x28,      // (
            0x6B,      // k
            0x03,      // pL
            0x00,      // pH
            0x32,      // cn = 50 (MaxiCode)
            0x43,      // fn = 67 (0x43 = modo)
            mode_byte, // n (0x32-0x36 = modo 2-6)
        ]);

        // Función 80 (0x50) - Almacenar datos en el buffer
        output.extend_from_slice(&[
            0x1D, // GS
            0x28, // (
            0x6B, // k
            p_l,  // pL
            p_h,  // pH
            0x32, // cn = 50
            0x50, // fn = 80 (0x50 = almacenar)
            0x30, // m = 48
        ]);
        output.extend_from_slice(data_bytes); // datos

        // Función 81 (0x51) - Imprimir el símbolo MaxiCode
        output.extend_from_slice(&[
            0x1D, // GS
            0x28, // (
            0x6B, // k
            0x03, // pL
            0x00, // pH
            0x32, // cn = 50
            0x51, // fn = 81 (0x51 = imprimir)
            0x30, // m = 48
        ]);

        output
    }
}

/// Procesa sección MaxiCode del modelo de impresión
pub fn process_section(
    maxicode: &MaxiCodeSection,
    current_styles: &GlobalStyles,
) -> Result<Vec<u8>, String> {
    if maxicode.data.is_empty() {
        return Err("MaxiCode data cannot be empty".to_string());
    }

    let esc_pos = MaxiCode::new(maxicode.data.clone()).set_mode(maxicode.mode);

    let mut data = Vec::new();
    if let Some(ref align) = maxicode.align {
        let mut temp_styles = current_styles.clone();
        temp_styles.align = Some(align.clone());
        data.extend_from_slice(&get_styles_diff(current_styles, &temp_styles));
        data.extend_from_slice(&esc_pos.get_command());
        data.extend_from_slice(b"\n");
        data.extend_from_slice(&get_styles_diff(&temp_styles, current_styles));
    } else {
        data.extend_from_slice(&esc_pos.get_command());
        data.extend_from_slice(b"\n");
    }

    Ok(data)
}
