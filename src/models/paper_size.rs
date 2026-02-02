pub enum PaperSize {
    Mm58,
    Mm80,
}

impl PaperSize {
    // Método para obtener caracteres por línea
    pub fn chars_per_line(&self) -> i32 {
        match self {
            PaperSize::Mm58 => 32,
            PaperSize::Mm80 => 48,
        }
    }

    // Método para obtener el ancho en píxeles
    pub fn pixels_width(&self) -> i32 {
        match self {
            PaperSize::Mm58 => 384,
            PaperSize::Mm80 => 576,
        }
    }

    // Método estático (Factory)
    pub fn from_string(size: &str) -> Self {
        match size.to_lowercase().as_str() {
            "58mm" | "58" | "small" => PaperSize::Mm58,
            "80mm" | "80" | "normal" | "default" => PaperSize::Mm80,
            _ => PaperSize::Mm80, // Caso por defecto
        }
    }
}