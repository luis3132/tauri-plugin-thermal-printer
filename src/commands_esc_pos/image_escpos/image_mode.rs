/// Modo de impresión de imagen
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ImageMode {
    Normal = 0,        // Normal (m=0, n=0)
    DoubleWidth = 1,   // Double width (m=1, n=0)
    DoubleHeight = 2,  // Double height (m=0, n=1)
    Quadruple = 3,     // Quadruple (m=1, n=1)
}

impl ImageMode {
    pub fn value(&self) -> u8 {
        *self as u8
    }
}