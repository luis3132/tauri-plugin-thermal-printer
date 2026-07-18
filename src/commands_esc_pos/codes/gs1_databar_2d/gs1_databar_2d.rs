use super::gs1_databar_2d_type::Gs1Databar2dType;
use crate::commands_esc_pos::text::text_type::get_styles_diff;
use crate::models::print_sections::{GlobalStyles, Gs1Databar2d as Gs1Databar2dSection};

/// Constructor de comandos para GS1 DataBar bidimensional
///
/// NOTA: GS1 DataBar 2D no es soportado por todas las impresoras térmicas.
/// Requiere firmware compatible (modelos Epson avanzados).
#[derive(Debug, Clone)]
pub struct Gs1Databar2d {
    data: String,
    databar_type: Gs1Databar2dType,
    width: u8, // Ancho del módulo (2-8)
}

impl Gs1Databar2d {
    /// Crea un nuevo GS1 DataBar 2D con valores por defecto
    pub fn new(data: String, databar_type: Gs1Databar2dType) -> Self {
        Self {
            data,
            databar_type,
            width: 2,
        }
    }

    /// Establece el ancho del módulo (2-8)
    pub fn set_width(mut self, width: u8) -> Self {
        if (2..=8).contains(&width) {
            self.width = width;
        }
        self
    }

    /// Genera el comando ESC/POS para GS1 DataBar 2D
    ///
    /// NOTA: no soportado por todas las impresoras térmicas.
    pub fn get_command(&self) -> Vec<u8> {
        let mut output = Vec::new();
        let data_bytes = self.data.as_bytes();
        let data_length = (data_bytes.len() + 3) as u16;
        let p_l = (data_length & 0xFF) as u8;
        let p_h = ((data_length >> 8) & 0xFF) as u8;

        // GS1 DataBar 2D usa cn = 51 (0x33)

        // Función 67 (0x43) - Ancho del módulo
        output.extend_from_slice(&[
            0x1D,       // GS
            0x28,       // (
            0x6B,       // k
            0x03,       // pL
            0x00,       // pH
            0x33,       // cn = 51 (GS1 DataBar 2D)
            0x43,       // fn = 67 (0x43 = ancho)
            self.width, // n (2-8)
        ]);

        // Función 80 (0x50) - Almacenar datos en el buffer
        // m selecciona el subtipo (Stacked / Stacked Omni / Expanded Stacked)
        output.extend_from_slice(&[
            0x1D,                     // GS
            0x28,                     // (
            0x6B,                     // k
            p_l,                      // pL
            p_h,                      // pH
            0x33,                     // cn = 51
            0x50,                     // fn = 80 (0x50 = almacenar)
            self.databar_type.value(), // m = subtipo
        ]);
        output.extend_from_slice(data_bytes); // datos

        // Función 81 (0x51) - Imprimir el símbolo
        output.extend_from_slice(&[
            0x1D, // GS
            0x28, // (
            0x6B, // k
            0x03, // pL
            0x00, // pH
            0x33, // cn = 51
            0x51, // fn = 81 (0x51 = imprimir)
            0x30, // m = 48
        ]);

        output
    }
}

/// Procesa sección GS1 DataBar 2D del modelo de impresión
pub fn process_section(
    databar: &Gs1Databar2dSection,
    current_styles: &GlobalStyles,
) -> Result<Vec<u8>, String> {
    if databar.data.is_empty() {
        return Err("GS1 DataBar 2D data cannot be empty".to_string());
    }

    let databar_type = match databar.databar_type.as_str() {
        "STACKED" => Gs1Databar2dType::Stacked,
        "STACKED-OMNI" => Gs1Databar2dType::StackedOmni,
        "EXPANDED-STACKED" => Gs1Databar2dType::ExpandedStacked,
        _ => Gs1Databar2dType::StackedOmni,
    };

    let esc_pos = Gs1Databar2d::new(databar.data.clone(), databar_type).set_width(databar.width);

    let mut data = Vec::new();
    if let Some(ref align) = databar.align {
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
