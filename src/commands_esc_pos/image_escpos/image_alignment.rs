/// Alineación de imagen
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ImageAlignment {
    Left = 0,
    Center = 1,
    Right = 2,
}

impl ImageAlignment {
    pub fn value(&self) -> u8 {
        *self as u8
    }
}