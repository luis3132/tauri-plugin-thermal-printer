use super::qr_model::QRModel;
use super::qr_size::QRSize;
use super::qr_error_correction::QRErrorCorrection;

/// Constructor de comandos para códigos QR
#[derive(Debug, Clone)]
pub struct QR {
    model: QRModel,
    size: QRSize,
    error_correction: QRErrorCorrection,
    data: String,
}

impl QR {
    /// Crea un nuevo código QR con valores por defecto
    ///
    /// # Arguments
    /// * `data` - Datos a codificar en el QR
    pub fn new(data: String) -> Self {
        Self {
            data,
            model: QRModel::Model2,
            size: QRSize::Size6,
            error_correction: QRErrorCorrection::M,
        }
    }

    /// Establece el modelo del código QR
    ///
    /// # Arguments
    /// * `model` - Modelo del QR (Model1 o Model2)
    pub fn set_model(mut self, model: QRModel) -> Self {
        self.model = model;
        self
    }

    /// Establece el tamaño del módulo del código QR
    ///
    /// # Arguments
    /// * `size` - Tamaño del módulo (1-16)
    pub fn set_size(mut self, size: QRSize) -> Self {
        self.size = size;
        self
    }

    /// Establece el nivel de corrección de errores
    ///
    /// # Arguments
    /// * `error_correction` - Nivel de corrección (L, M, Q, H)
    pub fn set_error_correction(mut self, error_correction: QRErrorCorrection) -> Self {
        self.error_correction = error_correction;
        self
    }

    /// Genera el comando ESC/POS para imprimir el código QR
    pub fn get_command(&self) -> Vec<u8> {
        let mut output = Vec::new();
        let data_bytes = self.data.as_bytes();
        let data_length = (data_bytes.len() + 3) as u16;
        let p_l = (data_length & 0xFF) as u8;
        let p_h = ((data_length >> 8) & 0xFF) as u8;

        // Función 165 - Seleccionar modelo de QR: GS ( k pL pH cn fn n1 n2
        output.extend_from_slice(&[
            0x1D, // GS
            0x28, // (
            0x6B, // k
            0x04, // pL
            0x00, // pH
            0x31, // cn = 49
            0x41, // fn = 65 (función de modelo)
            self.model.value(), // n1
            0x00, // n2
        ]);

        // Función 167 - Establecer tamaño del módulo: GS ( k pL pH cn fn n
        output.extend_from_slice(&[
            0x1D, // GS
            0x28, // (
            0x6B, // k
            0x03, // pL
            0x00, // pH
            0x31, // cn = 49
            0x43, // fn = 67 (función de tamaño)
            self.size.value(), // n
        ]);

        // Función 169 - Establecer nivel de corrección: GS ( k pL pH cn fn n
        output.extend_from_slice(&[
            0x1D, // GS
            0x28, // (
            0x6B, // k
            0x03, // pL
            0x00, // pH
            0x31, // cn = 49
            0x45, // fn = 69 (función de corrección)
            self.error_correction.value(), // n
        ]);

        // Función 180 - Almacenar datos: GS ( k pL pH cn fn m d1...dk
        output.extend_from_slice(&[
            0x1D, // GS
            0x28, // (
            0x6B, // k
            p_l,  // pL
            p_h,  // pH
            0x31, // cn = 49
            0x50, // fn = 80 (función de almacenamiento)
            0x30, // m = 48
        ]);
        output.extend_from_slice(data_bytes); // datos

        // Función 181 - Imprimir el QR: GS ( k pL pH cn fn m
        output.extend_from_slice(&[
            0x1D, // GS
            0x28, // (
            0x6B, // k
            0x03, // pL
            0x00, // pH
            0x31, // cn = 49
            0x51, // fn = 81 (función de impresión)
            0x30, // m = 48
        ]);

        output
    }

}