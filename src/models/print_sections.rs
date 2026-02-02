
pub enum PrintSections {
    Title(Title),
    Subtitle(Subtitle),
    Text(Text),
    Feed(Feed),
    Cut(Cut),
    Beep(Beep),
    Drawer(Drawer),
    GlobalStyles(GlobalStyles),
    Qr(Qr),
    Barcode(Barcode),
    Table(Table),
    DataMatrix(DataMatrix),
    Pdf417(Pdf417),
    Imagen(Imagen),
}

pub struct Title {
    pub text: String,
    pub styles: GlobalStyles,
}

pub struct Subtitle {
    pub text: String,
    pub styles: GlobalStyles,
}

pub struct Text {
    pub text: String,
    pub styles: GlobalStyles,
}

pub struct Feed {
    pub feed_type: String,
    pub value: u8,
}

pub struct Cut {
    pub mode: String,
    pub feed: u8,
}

pub struct Beep {
    pub times: u8,
    pub duration: u8,
}

pub struct Drawer {
    pub pin: u8,
    pub pulse_time: u16,
}

#[derive(Clone)]
pub struct GlobalStyles {
    pub bold: bool,
    pub underline: bool,
    pub align: String,
    pub italic: bool,
    pub invert: bool,
    pub font: String,
    pub rotate: bool,
    pub upside_down: bool,
    pub size: String,
}

pub struct Table {
    pub columns: u8,
    pub column_widths: Vec<u8>,
    pub header: Vec<String>,
    pub body: Vec<Vec<Text>>,
    pub truncate: bool,
}

pub struct Qr {
    pub data: String,
    pub size: u8,
    pub error_correction: String,
    pub model: u8,
}

pub struct Barcode {
    pub data: String,
    pub barcode_type: String,
    pub width: u8,
    pub height: u8,
    pub text_position: String,
}

pub struct DataMatrix {
    pub data: String,
    pub size: u8,
}

pub struct Pdf417 {
    pub data: String,
    pub columns: u8,
    pub rows: u8,
    pub error_correction: u8,
}

pub struct Imagen {
    pub data: String,
    pub max_width: i32,
    pub align: String,
    pub dithering: bool,
    pub size: String,
}