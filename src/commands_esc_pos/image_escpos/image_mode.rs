/// Modo de impresión de imagen
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ImageMode {
    Normal = 0,        // 8 dots single density
    DoubleWidth = 1,   // 8 dots double width
    DoubleHeight = 2,  // 8 dots double height
    Quadruple = 3,     // 8 dots quadruple
}

impl ImageMode {
    pub fn value(&self) -> u8 {
        *self as u8
    }
}