/// Nivel de corrección de errores del código QR
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum QRErrorCorrection {
    L = 48, // Recupera 7% de datos
    M = 49, // Recupera 15% de datos
    Q = 50, // Recupera 25% de datos
    H = 51, // Recupera 30% de datos
}

impl QRErrorCorrection {
    /// Obtiene el valor numérico del nivel de corrección
    pub fn value(&self) -> u8 {
        *self as u8
    }

    /// Maximum numeric character capacity for this error correction level (QR spec)
    pub fn max_data_len(&self) -> usize {
        match self {
            QRErrorCorrection::L => 7089,
            QRErrorCorrection::M => 4296,
            QRErrorCorrection::Q => 2953,
            QRErrorCorrection::H => 1817,
        }
    }
}
