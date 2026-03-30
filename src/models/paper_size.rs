use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PaperSize {
    /// 40mm paper — print width ~32mm, 256 dots @ 203 DPI
    Mm40,
    /// 44mm paper — print width ~36mm, 288 dots @ 203 DPI
    Mm44,
    /// 58mm paper — print width ~48mm, 384 dots @ 203 DPI (most common small format)
    Mm58,
    /// 72mm paper — print width ~64mm, 512 dots @ 203 DPI
    Mm72,
    /// 80mm paper — print width ~72mm, 576 dots @ 203 DPI (most common large format)
    Mm80,
    /// 104mm paper — print width ~94mm, 752 dots @ 203 DPI (wide format)
    Mm104,
}

impl PaperSize {
    /// Default paper size used when none is specified
    pub const DEFAULT: PaperSize = PaperSize::Mm80;

    // Método para obtener caracteres por línea
    pub fn chars_per_line(&self) -> i32 {
        match self {
            PaperSize::Mm40 => 21,
            PaperSize::Mm44 => 24,
            PaperSize::Mm58 => 32,
            PaperSize::Mm72 => 42,
            PaperSize::Mm80 => 48,
            PaperSize::Mm104 => 62,
        }
    }

    // Método para obtener el ancho en píxeles
    pub fn pixels_width(&self) -> i32 {
        match self {
            PaperSize::Mm40 => 256,
            PaperSize::Mm44 => 288,
            PaperSize::Mm58 => 384,
            PaperSize::Mm72 => 512,
            PaperSize::Mm80 => 576,
            PaperSize::Mm104 => 752,
        }
    }

    // Método estático (Factory)
    pub fn from_string(size: &str) -> Self {
        match size.to_lowercase().as_str() {
            "40mm" | "40" => PaperSize::Mm40,
            "44mm" | "44" => PaperSize::Mm44,
            "58mm" | "58" | "small" => PaperSize::Mm58,
            "72mm" | "72" => PaperSize::Mm72,
            "80mm" | "80" | "normal" | "default" => PaperSize::Mm80,
            "104mm" | "104" | "wide" => PaperSize::Mm104,
            _ => PaperSize::Mm80,
        }
    }
}
