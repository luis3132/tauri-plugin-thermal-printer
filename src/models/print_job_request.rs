
use crate::models::print_sections::PrintSections;
use crate::models::printer_options::PrinterOptions;
use crate::models::paper_size::PaperSize;

pub struct PrintJobRequest {
    pub printer: String,
    pub sections: Vec<PrintSections>,
    pub options: PrinterOptions,
    pub paper_size: PaperSize,
}