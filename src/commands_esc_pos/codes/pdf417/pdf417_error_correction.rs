/// Nivel de corrección de errores para PDF417
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PDF417ErrorCorrection {
    Level0 = 48, // 2 palabras de corrección
    Level1 = 49, // 4 palabras de corrección
    Level2 = 50, // 8 palabras de corrección
    Level3 = 51, // 16 palabras de corrección
    Level4 = 52, // 32 palabras de corrección
    Level5 = 53, // 64 palabras de corrección
    Level6 = 54, // 128 palabras de corrección
    Level7 = 55, // 256 palabras de corrección
    Level8 = 56, // 512 palabras de corrección
}

impl PDF417ErrorCorrection {
    /// Obtiene el valor numérico del nivel de corrección
    pub fn value(&self) -> u8 {
        *self as u8
    }
}