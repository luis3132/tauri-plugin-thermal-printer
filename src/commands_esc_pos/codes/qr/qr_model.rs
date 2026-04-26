/// Modelo de código QR
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum QRModel {
    Model1 = 49, // Modelo 1
    Model2 = 50, // Modelo 2 (recomendado)
}

impl QRModel {
    /// Obtiene el valor numérico del modelo
    pub fn value(&self) -> u8 {
        *self as u8
    }
}
