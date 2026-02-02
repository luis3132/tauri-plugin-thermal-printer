/// Posición del texto HRI (Human Readable Interpretation)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BarcodeTextPosition {
    NotPrinted = 0,  // No imprimir texto
    Above = 1,       // Encima del código
    Below = 2,       // Debajo del código
    Both = 3,        // Ambos lados
}

impl BarcodeTextPosition {
    /// Obtiene el valor numérico de la posición del texto
    pub fn value(&self) -> u8 {
        *self as u8
    }
}