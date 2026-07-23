use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PrintSections {
    Title(Title),
    Subtitle(Subtitle),
    Text(Text),
    Feed(Feed),
    Cut(Cut),
    Beep(Beep),
    /// Generic buzzer (`ESC B n t`) — for printers that ignore the Epson `Beep` (`ESC ( A`).
    Beep2(Beep),
    Drawer(Drawer),
    GlobalStyles(GlobalStyles),
    Qr(Qr),
    Barcode(Barcode),
    Table(Table),
    DataMatrix(DataMatrixModel),
    Pdf417(Pdf417),
    Aztec(Aztec),
    Gs1Databar2d(Gs1Databar2d),
    MaxiCode(MaxiCode),
    Composite(Composite),
    Image(Image),
    Logo(Logo),
    Line(Line),
    LineSpacing(LineSpacing),
    CharSpacing(CharSpacing),
    Position(Position),
    TabStops(TabStops),
    LeftMargin(LeftMargin),
    PrintAreaWidth(PrintAreaWidth),
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
    /// Double-strike / double-print (`ESC G`). Reinforces bold on printers that
    /// render `ESC E` weakly. Widely supported on generic printers.
    pub double_strike: Option<bool>,
    /// When `true`, emits `ESC @` (initialize) and re-applies the code page,
    /// resetting the printer to defaults. Takes priority: all other style fields
    /// in the same section are ignored.
    pub reset: Option<bool>,
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
            double_strike: Some(false),
            reset: Some(false),
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
    pub align: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Barcode {
    pub data: String,
    pub barcode_type: String,
    pub width: u8,
    pub height: u8,
    pub text_position: String,
    pub align: Option<String>,
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
pub struct Aztec {
    pub data: String,
    pub mode: u8,
    pub layers: u8,
    pub size: u8,
    pub error_correction: u8,
    pub align: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Gs1Databar2d {
    pub data: String,
    pub databar_type: String,
    pub width: u8,
    pub align: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MaxiCode {
    pub data: String,
    pub mode: u8,
    pub align: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Composite {
    pub data: String,
    pub symbol_type: u8,
    pub width: u8,
    pub align: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Image {
    pub data: String,
    pub max_width: i32,
    pub align: String,
    pub dithering: bool,
    pub size: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Logo {
    /// NV bit-image key code to print (`FS p n`). Ignored when `set_logo` is set.
    /// Defaults to `1` when omitted.
    pub key_code: Option<u8>,
    /// Print mode (`normal`/`double_width`/`double_height`/`quadruple`).
    /// Ignored when `set_logo` is set. Defaults to `normal` when omitted.
    pub mode: Option<String>,
    /// When present, store this image as the NV logo (`FS q`) instead of printing.
    /// Takes priority: `key_code`/`mode` are ignored and nothing is printed — the
    /// image is downloaded to the printer's non-volatile memory (key code `1`) and
    /// can later be printed with a `Logo` section without `set_logo`.
    #[serde(default)]
    pub set_logo: Option<Image>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Line {
    pub character: String,
}

/// Vertical line spacing (`ESC 3 n` / `ESC 2`).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LineSpacing {
    /// Spacing in dots. `None` resets to the printer default (~1/6", `ESC 2`).
    pub value: Option<u8>,
}

/// Right-side character spacing (`ESC SP n`).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CharSpacing {
    /// Extra spacing to the right of each character, in dots (0–255).
    pub value: u8,
}

/// Absolute horizontal print position for the next data (`ESC $ nL nH`).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Position {
    /// Distance from the left margin, in dots.
    pub value: u16,
}

/// Horizontal tab stop positions (`ESC D ... NUL`). Emit a `\t` in text to jump.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TabStops {
    /// Tab stop columns (in characters), ascending. Up to 32 stops.
    pub positions: Vec<u8>,
}

/// Left margin in standard mode (`GS L nL nH`).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LeftMargin {
    /// Left margin, in dots.
    pub value: u16,
}

/// Printable area width in standard mode (`GS W nL nH`).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrintAreaWidth {
    /// Printable area width, in dots.
    pub value: u16,
}
