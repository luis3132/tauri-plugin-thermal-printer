#[derive(Debug, Clone, Copy)]
pub enum TextType {
    // Tamaño de texto
    Normal,
    DoubleHeight,
    DoubleWidth,
    DoubleSize,
    
    // Estilo de texto
    BoldOn,
    BoldOff,
    UnderlineOn,
    UnderlineOff,
    ItalicOn,
    ItalicOff,
    
    // Alineación
    AlignLeft,
    AlignCenter,
    AlignRight,
    
    // Otros...
    InvertOn,
    InvertOff,
    FontA,
    FontB,
    FontC,
    RotateOn,
    RotateOff,
    UpsideDownOn,
    UpsideDownOff,
}

impl TextType {
    /// Retorna el comando ESC/POS como un slice de bytes.
    /// Usamos &'static [u8] porque son constantes grabadas en el binario.
    pub fn command(&self) -> &'static [u8] {
        match self {
            // Tamaño
            Self::Normal => &[0x1B, 0x21, 0x00],
            Self::DoubleHeight => &[0x1B, 0x21, 0x10],
            Self::DoubleWidth => &[0x1B, 0x21, 0x20],
            Self::DoubleSize => &[0x1B, 0x21, 0x30],

            // Estilo
            Self::BoldOn => &[0x1B, 0x45, 0x01],
            Self::BoldOff => &[0x1B, 0x45, 0x00],
            Self::UnderlineOn => &[0x1B, 0x2D, 0x01],
            Self::UnderlineOff => &[0x1B, 0x2D, 0x00],
            Self::ItalicOn => &[0x1B, 0x34],
            Self::ItalicOff => &[0x1B, 0x35],

            // Alineación
            Self::AlignLeft => &[0x1B, 0x61, 0x00],
            Self::AlignCenter => &[0x1B, 0x61, 0x01],
            Self::AlignRight => &[0x1B, 0x61, 0x02],

            // Inversión
            Self::InvertOn => &[0x1D, 0x42, 0x01],
            Self::InvertOff => &[0x1D, 0x42, 0x00],

            // Fuente
            Self::FontA => &[0x1B, 0x4D, 0x00],
            Self::FontB => &[0x1B, 0x4D, 0x01],
            Self::FontC => &[0x1B, 0x4D, 0x02],

            // Rotación y Upside Down
            Self::RotateOn => &[0x1B, 0x56, 0x01],
            Self::RotateOff => &[0x1B, 0x56, 0x00],
            Self::UpsideDownOn => &[0x1B, 0x7B, 0x01],
            Self::UpsideDownOff => &[0x1B, 0x7B, 0x00],
        }
    }
}