use super::barcode_text_position::BarcodeTextPosition;
use super::barcode_type::BarcodeType;
use crate::commands_esc_pos::text::text_type::get_styles_diff;
use crate::models::print_sections::{Barcode as BarcodeSection, GlobalStyles};

/// Constructor de comandos para códigos de barras
#[derive(Debug, Clone)]
pub struct Barcode {
    barcode_type: BarcodeType,
    data: String,
    height: u8,
    width: u8,
    text_position: BarcodeTextPosition,
}

impl Barcode {
    /// Crea un nuevo código de barras con valores por defecto
    ///
    /// # Arguments
    /// * `barcode_type` - Tipo de código de barras
    /// * `data` - Datos a codificar
    pub fn new(barcode_type: BarcodeType, data: String) -> Self {
        Self {
            barcode_type,
            data,
            height: 162, // Altura por defecto
            width: 3,    // Ancho por defecto
            text_position: BarcodeTextPosition::Below,
        }
    }

    /// Establece la altura del código de barras en puntos
    ///
    /// # Arguments
    /// * `height` - Altura en puntos (1-255)
    pub fn set_height(mut self, height: u8) -> Self {
        self.height = height;
        self
    }

    /// Establece el ancho del código de barras
    ///
    /// # Arguments
    /// * `width` - Ancho (2-6)
    pub fn set_width(mut self, width: u8) -> Self {
        if (2..=6).contains(&width) {
            self.width = width;
        }
        self
    }

    /// Establece la posición del texto HRI
    ///
    /// # Arguments
    /// * `position` - Posición del texto
    pub fn set_text_position(mut self, position: BarcodeTextPosition) -> Self {
        self.text_position = position;
        self
    }

    /// Genera el comando ESC/POS para imprimir el código de barras
    /// Usa el método 2 con longitud explícita (más moderno)
    pub fn get_command(&self) -> Vec<u8> {
        let mut output = Vec::new();
        let data_bytes = self.data.as_bytes();

        // Establecer altura del código de barras: GS h n
        output.push(0x1D); // GS
        output.push(0x68); // h
        output.push(self.height); // n (altura en puntos)

        // Establecer ancho del código de barras: GS w n
        output.push(0x1D); // GS
        output.push(0x77); // w
        output.push(self.width); // n (2-6)

        // Establecer posición del texto HRI: GS H n
        output.push(0x1D); // GS
        output.push(0x48); // H
        output.push(self.text_position.value()); // n (0-3)

        // Imprimir código de barras - Método 2 (con longitud explícita)
        // GS k m n d1...dn
        output.push(0x1D); // GS
        output.push(0x6B); // k
        output.push(self.barcode_type.value()); // m (tipo de código)
        output.push(data_bytes.len() as u8); // n (longitud de datos)
        output.extend_from_slice(data_bytes); // d1...dn (datos)

        output
    }
}

/// Procesa sección Barcode del modelo de impresión
pub fn process_section(
    barcode: &BarcodeSection,
    current_styles: &GlobalStyles,
) -> Result<Vec<u8>, String> {
    if barcode.data.is_empty() {
        return Err("Barcode data cannot be empty".to_string());
    }
    if barcode.height == 0 {
        return Err("Barcode height must be greater than 0".to_string());
    }

    let barcode_type = match barcode.barcode_type.as_str() {
        "UPC-A" => BarcodeType::UpcA,
        "UPC-E" => BarcodeType::UpcE,
        "EAN13" => BarcodeType::Ean13,
        "EAN8" => BarcodeType::Ean8,
        "CODE39" => BarcodeType::Code39,
        "ITF" => BarcodeType::Itf,
        "CODABAR" => BarcodeType::Codabar,
        "CODE93" => BarcodeType::Code93,
        "CODE128" => BarcodeType::Code128,
        _ => BarcodeType::Code128,
    };

    if barcode_type.requires_numeric_data() && !barcode.data.chars().all(|c| c.is_ascii_digit()) {
        return Err(format!(
            "Barcode type '{}' only accepts numeric digits",
            barcode.barcode_type
        ));
    }

    let text_position = match barcode.text_position.as_str() {
        "none" => BarcodeTextPosition::NotPrinted,
        "above" => BarcodeTextPosition::Above,
        "below" => BarcodeTextPosition::Below,
        "both" => BarcodeTextPosition::Both,
        _ => BarcodeTextPosition::NotPrinted,
    };

    let esc_pos_barcode = Barcode::new(barcode_type, barcode.data.clone())
        .set_height(barcode.height)
        .set_width(barcode.width)
        .set_text_position(text_position);

    let mut data = Vec::new();
    if let Some(ref align) = barcode.align {
        let mut temp_styles = current_styles.clone();
        temp_styles.align = Some(align.clone());
        data.extend_from_slice(&get_styles_diff(current_styles, &temp_styles));
        data.extend_from_slice(&esc_pos_barcode.get_command());
        data.extend_from_slice(b"\n");
        data.extend_from_slice(&get_styles_diff(&temp_styles, current_styles));
    } else {
        data.extend_from_slice(&esc_pos_barcode.get_command());
        data.extend_from_slice(b"\n");
    }
    Ok(data)
}

impl Barcode {
    // /// Método alternativo usando formato con NUL terminator (para impresoras antiguas)
    // /// Usa el método 1 con terminador NULL
    // pub fn get_command_legacy(&self) -> Vec<u8> {
    //     let mut output = Vec::new();
    //     let data_bytes = self.data.as_bytes();

    //     // Establecer altura
    //     output.push(0x1D); // GS
    //     output.push(0x68); // h
    //     output.push(self.height);

    //     // Establecer ancho
    //     output.push(0x1D); // GS
    //     output.push(0x77); // w
    //     output.push(self.width);

    //     // Posición del texto
    //     output.push(0x1D); // GS
    //     output.push(0x48); // H
    //     output.push(self.text_position.value());

    //     // Imprimir código de barras - Método 1 (con NUL)
    //     // GS k m d1...dk NUL
    //     output.push(0x1D); // GS
    //     output.push(0x6B); // k
    //     output.push(self.barcode_type.value()); // m
    //     output.extend_from_slice(data_bytes); // datos
    //     output.push(0x00); // NUL

    //     output
    // }
}
