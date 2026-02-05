
use serde::{Deserialize, Serialize};
use crate::models::print_sections::PrintSections;
use crate::models::printer_options::PrinterOptions;
use crate::models::paper_size::PaperSize;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrintJobRequest {
    /// Printer name (for system printing) or connection configuration
    pub printer: String,
    pub sections: Vec<PrintSections>,
    pub options: PrinterOptions,
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
}

fn default_true() -> bool {
    true
}

fn default_false() -> bool {
    false
}