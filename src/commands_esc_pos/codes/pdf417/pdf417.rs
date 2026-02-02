use super::pdf417_error_correction::PDF417ErrorCorrection;

/// Constructor de comandos para códigos PDF417
/// 
/// NOTA: PDF417 no es soportado por todas las impresoras térmicas.
/// Funciona principalmente en modelos Epson avanzados.
#[derive(Debug, Clone)]
pub struct PDF417 {
    data: String,
    columns: u8,  // Número de columnas (0 = automático, 1-30)
    rows: u8,     // Número de filas (0 = automático, 3-90)
    width: u8,    // Ancho del módulo (2-8)
    height: u8,   // Altura de la fila (2-8)
    error_correction: PDF417ErrorCorrection,
}

impl PDF417 {
    /// Crea un nuevo código PDF417 con valores por defecto
    ///
    /// # Arguments
    /// * `data` - Datos a codificar en el PDF417
    pub fn new(data: String) -> Self {
        Self {
            data,
            columns: 0,    // Automático
            rows: 0,       // Automático
            width: 3,
            height: 3,
            error_correction: PDF417ErrorCorrection::Level1,
        }
    }

    /// Establece el número de columnas
    ///
    /// # Arguments
    /// * `columns` - Número de columnas (0 = automático, 1-30)
    pub fn set_columns(mut self, columns: u8) -> Self {
        if columns <= 30 {
            self.columns = columns;
        }
        self
    }

    /// Establece el número de filas
    ///
    /// # Arguments
    /// * `rows` - Número de filas (0 = automático, 3-90)
    pub fn set_rows(mut self, rows: u8) -> Self {
        if rows == 0 || (rows >= 3 && rows <= 90) {
            self.rows = rows;
        }
        self
    }

    /// Establece el ancho del módulo
    ///
    /// # Arguments
    /// * `width` - Ancho del módulo (2-8)
    pub fn set_width(mut self, width: u8) -> Self {
        if (2..=8).contains(&width) {
            self.width = width;
        }
        self
    }

    /// Establece la altura de la fila
    ///
    /// # Arguments
    /// * `height` - Altura de la fila (2-8)
    pub fn set_height(mut self, height: u8) -> Self {
        if (2..=8).contains(&height) {
            self.height = height;
        }
        self
    }

    /// Establece el nivel de corrección de errores
    ///
    /// # Arguments
    /// * `error_correction` - Nivel de corrección (Level0-Level8)
    pub fn set_error_correction(mut self, error_correction: PDF417ErrorCorrection) -> Self {
        self.error_correction = error_correction;
        self
    }

    /// Genera el comando ESC/POS para PDF417
    /// 
    /// NOTA: PDF417 no es soportado por todas las impresoras térmicas.
    /// Funciona principalmente en modelos Epson avanzados.
    pub fn get_command(&self) -> Vec<u8> {
        let mut output = Vec::new();
        let data_bytes = self.data.as_bytes();
        let data_length = (data_bytes.len() + 3) as u16;
        let p_l = (data_length & 0xFF) as u8;
        let p_h = ((data_length >> 8) & 0xFF) as u8;

        // PDF417 usa cn = 48 (0x30) y varios códigos de función

        // Función 165 (0xA5) - Establecer número de columnas
        output.extend_from_slice(&[
            0x1D, // GS
            0x28, // (
            0x6B, // k
            0x03, // pL
            0x00, // pH
            0x30, // cn = 48 (PDF417)
            0x41, // fn = 65 (0x41 = columnas)
            self.columns, // n (0-30)
        ]);

        // Función 166 (0xA6) - Establecer número de filas
        output.extend_from_slice(&[
            0x1D, // GS
            0x28, // (
            0x6B, // k
            0x03, // pL
            0x00, // pH
            0x30, // cn = 48
            0x42, // fn = 66 (0x42 = filas)
            self.rows, // n (0, 3-90)
        ]);

        // Función 167 (0xA7) - Establecer ancho del módulo
        output.extend_from_slice(&[
            0x1D, // GS
            0x28, // (
            0x6B, // k
            0x03, // pL
            0x00, // pH
            0x30, // cn = 48
            0x43, // fn = 67 (0x43 = ancho)
            self.width, // n (2-8)
        ]);

        // Función 168 (0xA8) - Establecer altura de fila
        output.extend_from_slice(&[
            0x1D, // GS
            0x28, // (
            0x6B, // k
            0x03, // pL
            0x00, // pH
            0x30, // cn = 48
            0x44, // fn = 68 (0x44 = altura)
            self.height, // n (2-8)
        ]);

        // Función 169 (0xA9) - Establecer nivel de corrección de error
        output.extend_from_slice(&[
            0x1D, // GS
            0x28, // (
            0x6B, // k
            0x03, // pL
            0x00, // pH
            0x30, // cn = 48
            0x45, // fn = 69 (0x45 = corrección)
            self.error_correction.value(), // n
        ]);

        // Función 180 (0xB4) - Almacenar datos en el buffer
        output.extend_from_slice(&[
            0x1D, // GS
            0x28, // (
            0x6B, // k
            p_l,  // pL (parte baja de longitud)
            p_h,  // pH (parte alta de longitud)
            0x30, // cn = 48
            0x50, // fn = 80 (0x50 = almacenar)
            0x30, // m = 48
        ]);
        output.extend_from_slice(data_bytes); // datos

        // Función 181 (0xB5) - Imprimir el símbolo PDF417
        output.extend_from_slice(&[
            0x1D, // GS
            0x28, // (
            0x6B, // k
            0x03, // pL
            0x00, // pH
            0x30, // cn = 48
            0x51, // fn = 81 (0x51 = imprimir)
            0x30, // m = 48
        ]);

        output
    }

    /// Verifica si la impresora soporta PDF417
    /// Retorna un comando de consulta (no todas las impresoras responden)
    pub fn check_support() -> Vec<u8> {
        vec![
            0x1D, // GS
            0x28, // (
            0x6B, // k
            0x02, // pL
            0x00, // pH
            0x30, // cn = 48
            0x40, // fn = 64 (consultar)
        ]
    }
}