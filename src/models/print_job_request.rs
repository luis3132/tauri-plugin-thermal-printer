use crate::models::paper_size::PaperSize;
use crate::models::print_sections::PrintSections;
use crate::commands_esc_pos::text::code_page::CodePage;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrintJobRequest {
    /// Printer name (for system printing) or connection configuration
    pub printer: String,
    pub sections: Vec<PrintSections>,
    pub options: CodePage,
    pub paper_size: PaperSize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrinterInfo {
    pub name: String,
    pub interface_type: String,
    pub identifier: String, // IP:PORT, MAC address, or USB port
    pub status: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct TestPrintRequest {
    pub printer_info: PrintJobRequest,

    // Secciones de prueba
    #[serde(default = "default_true")]
    pub include_text: bool,

    #[serde(default = "default_false")]
    pub include_custom_text: bool,

    #[serde(default)]
    pub custom_text: Option<String>,

    #[serde(default = "default_true")]
    pub include_text_styles: bool,

    #[serde(default = "default_true")]
    pub include_alignment: bool,

    #[serde(default = "default_true")]
    pub include_columns: bool,

    #[serde(default = "default_true")]
    pub include_separators: bool,

    // Códigos
    #[serde(default = "default_true")]
    pub include_barcode: bool,

    #[serde(default = "default_false")]
    pub include_barcode_types: bool,

    #[serde(default = "default_true")]
    pub include_qr: bool,

    // Imágenes
    #[serde(default = "default_false")]
    pub include_image: bool,

    #[serde(default)]
    pub image_base64: Option<String>,

    // Control
    #[serde(default = "default_true")]
    pub include_beep: bool,

    #[serde(default = "default_false")]
    pub test_cash_drawer: bool,

    #[serde(default = "default_true")]
    pub cut_paper: bool,

    #[serde(default = "default_true")]
    pub test_feed: bool,

    // Opciones avanzadas
    #[serde(default = "default_false")]
    pub test_all_fonts: bool,

    #[serde(default = "default_false")]
    pub test_invert: bool,

    #[serde(default = "default_false")]
    pub test_rotate: bool,

    /// Double-strike (`ESC G`) demo line inside the text-styles section.
    #[serde(default = "default_false")]
    pub test_double_strike: bool,

    /// Line spacing (`ESC 3`/`ESC 2`) + character spacing (`ESC SP`) demo.
    #[serde(default = "default_false")]
    pub test_spacing: bool,

    /// Tab stops (`ESC D`), absolute position (`ESC $`) and margins (`GS L`/`GS W`) demo.
    #[serde(default = "default_false")]
    pub test_positioning: bool,

    /// Generic buzzer (`ESC B`) — for printers that ignore the Epson beep (`ESC ( A`).
    #[serde(default = "default_false")]
    pub test_beep2: bool,

    /// NV logo demo: stores `image_base64` as the NV logo (`FS q`) and prints it
    /// (`FS p`). Requires `image_base64`; skipped otherwise.
    #[serde(default = "default_false")]
    pub test_logo: bool,
}

fn default_true() -> bool {
    true
}

fn default_false() -> bool {
    false
}
