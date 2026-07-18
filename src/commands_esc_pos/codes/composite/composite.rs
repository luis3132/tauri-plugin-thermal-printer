use crate::commands_esc_pos::text::text_type::get_styles_diff;
use crate::models::print_sections::{Composite as CompositeSection, GlobalStyles};

/// Constructor de comandos para Composite Symbology (GS1 Composite: CC-A/CC-B/CC-C).
///
/// NOTA: Composite Symbology no es soportado por todas las impresoras térmicas.
/// Requiere firmware compatible (modelos Epson avanzados).
///
/// El byte `symbol_type` (parámetro `m` de la función 80) selecciona el
/// componente 1D anfitrión + el componente 2D. Su valor exacto depende del
/// modelo/spec de la impresora: consúltalo en la referencia ESC/POS del equipo.
#[derive(Debug, Clone)]
pub struct Composite {
    data: String,
    symbol_type: u8, // m de la función 80 (tipo de símbolo, depende del modelo)
    width: u8,       // Ancho del módulo (2-8)
}

impl Composite {
    /// Crea un nuevo Composite con valores por defecto
    pub fn new(data: String) -> Self {
        Self {
            data,
            symbol_type: 48, // valor por defecto
            width: 2,
        }
    }

    /// Establece el tipo de símbolo (parámetro `m`, específico del modelo)
    pub fn set_symbol_type(mut self, symbol_type: u8) -> Self {
        self.symbol_type = symbol_type;
        self
    }

    /// Establece el ancho del módulo (2-8)
    pub fn set_width(mut self, width: u8) -> Self {
        if (2..=8).contains(&width) {
            self.width = width;
        }
        self
    }

    /// Genera el comando ESC/POS para Composite Symbology
    ///
    /// NOTA: no soportado por todas las impresoras térmicas.
    pub fn get_command(&self) -> Vec<u8> {
        let mut output = Vec::new();
        let data_bytes = self.data.as_bytes();
        let data_length = (data_bytes.len() + 3) as u16;
        let p_l = (data_length & 0xFF) as u8;
        let p_h = ((data_length >> 8) & 0xFF) as u8;

        // Composite Symbology usa cn = 52 (0x34)

        // Función 67 (0x43) - Ancho del módulo
        output.extend_from_slice(&[
            0x1D,       // GS
            0x28,       // (
            0x6B,       // k
            0x03,       // pL
            0x00,       // pH
            0x34,       // cn = 52 (Composite)
            0x43,       // fn = 67 (0x43 = ancho)
            self.width, // n (2-8)
        ]);

        // Función 80 (0x50) - Almacenar datos en el buffer
        // m = symbol_type selecciona el tipo de componente (depende del modelo)
        output.extend_from_slice(&[
            0x1D,             // GS
            0x28,             // (
            0x6B,             // k
            p_l,              // pL
            p_h,              // pH
            0x34,             // cn = 52
            0x50,             // fn = 80 (0x50 = almacenar)
            self.symbol_type, // m = tipo de símbolo
        ]);
        output.extend_from_slice(data_bytes); // datos

        // Función 81 (0x51) - Imprimir el símbolo
        output.extend_from_slice(&[
            0x1D, // GS
            0x28, // (
            0x6B, // k
            0x03, // pL
            0x00, // pH
            0x34, // cn = 52
            0x51, // fn = 81 (0x51 = imprimir)
            0x30, // m = 48
        ]);

        output
    }
}

/// Procesa sección Composite del modelo de impresión
pub fn process_section(
    composite: &CompositeSection,
    current_styles: &GlobalStyles,
) -> Result<Vec<u8>, String> {
    if composite.data.is_empty() {
        return Err("Composite data cannot be empty".to_string());
    }

    let esc_pos = Composite::new(composite.data.clone())
        .set_symbol_type(composite.symbol_type)
        .set_width(composite.width);

    let mut data = Vec::new();
    if let Some(ref align) = composite.align {
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
