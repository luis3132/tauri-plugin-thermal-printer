
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
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
    DataMatrix(DataMatrixModel),
    Pdf417(Pdf417),
    Imagen(Imagen),
    Logo(Logo),
    Line(Line),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Title {
    pub text: String,
    pub styles: Option<GlobalStyles>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Subtitle {
    pub text: String,
    pub styles: Option<GlobalStyles>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Text {
    pub text: String,
    pub styles: Option<GlobalStyles>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Feed {
    pub feed_type: String,
    pub value: u8,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Cut {
    pub mode: String,
    pub feed: u8,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Beep {
    pub times: u8,
    pub duration: u8,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Drawer {
    pub pin: u8,
    pub pulse_time: u16,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GlobalStyles {
    pub bold: Option<bool>,
    pub underline: Option<bool>,
    pub align: Option<String>,
    pub italic: Option<bool>,
    pub invert: Option<bool>,
    pub font: Option<String>,
    pub rotate: Option<bool>,
    pub upside_down: Option<bool>,
    pub size: Option<String>,
}

impl Default for GlobalStyles {
    fn default() -> Self {
        Self {
            bold: Some(false),
            underline: Some(false),
            align: Some("left".to_string()),
            italic: Some(false),
            invert: Some(false),
            font: Some("A".to_string()),
            rotate: Some(false),
            upside_down: Some(false),
            size: Some("normal".to_string()),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Table {
    pub columns: u8,
    pub column_widths: Option<Vec<u8>>,
    pub header: Option<Vec<Text>>,
    pub body: Vec<Vec<Text>>,
    pub truncate: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Qr {
    pub data: String,
    pub size: u8,
    pub error_correction: String,
    pub model: u8,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Barcode {
    pub data: String,
    pub barcode_type: String,
    pub width: u8,
    pub height: u8,
    pub text_position: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataMatrixModel {
    pub data: String,
    pub size: u8,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Pdf417 {
    pub data: String,
    pub columns: u8,
    pub rows: u8,
    pub width: u8,
    pub height: u8,
    pub error_correction: u8,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Imagen {
    pub data: String,
    pub max_width: i32,
    pub align: String,
    pub dithering: bool,
    pub size: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Logo {
    pub key_code: u8,
    pub mode: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Line {
    pub character: String,
}