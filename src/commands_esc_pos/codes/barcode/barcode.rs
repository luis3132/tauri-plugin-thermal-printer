use super::barcode_type::BarcodeType;
use super::barcode_text_position::BarcodeTextPosition;

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
            height: 162,  // Altura por defecto
            width: 3,     // Ancho por defecto
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