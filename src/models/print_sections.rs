
pub enum PrintSections {
    title(Title),
    subtitle(Subtitle),
    text(Text),
    feed(Feed),
    cut(Cut),
    beep(Beep),
    drawer(Drawer),
    text_global(TextGlobal),
}

pub struct Title {
    pub text: String,
    pub bold: bool,
    pub underline: bool,
    pub italic: bool,
    pub invert: bool,
    pub font: String,
    pub rotate: bool,
    pub upside_down: bool,
}

pub struct Subtitle {
    pub text: String,
    pub underline: bool,
    pub italic: bool,
    pub invert: bool,
    pub font: String,
    pub rotate: bool,
    pub upside_down: bool,
}

pub struct Text {
    pub text: String,
    pub align: String,
    pub size: String,
    pub bold: bool,
    pub underline: bool,
    pub italic: bool,
    pub invert: bool,
    pub font: String,
    pub rotate: bool,
    pub upside_down: bool,
}

pub struct Feed {
    pub lines: u8,
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

pub struct TextGlobal {
    pub bold: bool,
    pub underline: bool,
    pub align: String,
    pub italic: bool,
    pub invert: bool,
    pub font: String,
    pub rotate: bool,
    pub upside_down: bool,
}

pub struct Table {
    pub columns: u8,
    pub column_widths: Vec<u8>,
    pub header: Vec<String>,
    pub body: Vec<Vec<String>>,
    pub truncate: bool,
}

