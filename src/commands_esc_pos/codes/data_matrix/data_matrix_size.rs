/// Tamaño del módulo del código DataMatrix (1-16)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DataMatrixSize {
    Size1 = 1,
    Size2 = 2,
    Size3 = 3,
    Size4 = 4,
    Size5 = 5,
    Size6 = 6,
    Size7 = 7,
    Size8 = 8,
    Size9 = 9,
    Size10 = 10,
    Size11 = 11,
    Size12 = 12,
    Size13 = 13,
    Size14 = 14,
    Size15 = 15,
    Size16 = 16,
}

impl DataMatrixSize {
    /// Obtiene el valor numérico del tamaño
    pub fn value(&self) -> u8 {
        *self as u8
    }
}