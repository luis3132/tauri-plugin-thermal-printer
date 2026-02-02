/// Tipos de códigos de barras soportados
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BarcodeType {
    UpcA = 65,      // UPC-A
    UpcE = 66,      // UPC-E
    Ean13 = 67,     // EAN13
    Ean8 = 68,      // EAN8
    Code39 = 69,    // CODE39
    Itf = 70,       // ITF (Interleaved 2 of 5)
    Codabar = 71,   // CODABAR
    Code93 = 72,    // CODE93
    Code128 = 73,   // CODE128
}

impl BarcodeType {
    /// Obtiene el valor numérico del tipo de código de barras
    pub fn value(&self) -> u8 {
        *self as u8
    }
}