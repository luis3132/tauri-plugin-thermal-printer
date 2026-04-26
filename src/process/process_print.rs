use crate::commands_esc_pos::codes::barcode::barcode as barcode_cmd;
use crate::commands_esc_pos::codes::data_matrix::data_matrix as data_matrix_cmd;
use crate::commands_esc_pos::codes::pdf417::pdf417 as pdf417_cmd;
use crate::commands_esc_pos::codes::qr::qr as qr_cmd;
use crate::commands_esc_pos::control::printer_control::PrinterControl;
use crate::commands_esc_pos::image_escpos::image_code as image_cmd;
use crate::commands_esc_pos::image_escpos::logo as logo_cmd;
use crate::commands_esc_pos::text::encoder::TextEncoder;
use crate::commands_esc_pos::text::table as table_cmd;
use crate::commands_esc_pos::text::text_type::{
    get_styles_diff, process_line, process_subtitle, process_text, process_title,
};
use crate::models::print_job_request::PrintJobRequest;
use crate::models::print_sections::{Beep, Cut, Drawer, GlobalStyles, PrintSections};

const AUTO_BEEP_TIMES: u8 = 1;
const AUTO_BEEP_DURATION: u8 = 3;
const AUTO_CUT_FEED: u8 = 0;
const AUTO_CUT_MODE: &str = "partial";
const AUTO_DRAWER_PIN: u8 = 2;
const AUTO_DRAWER_PULSE_TIME: u16 = 100;

pub struct ProcessPrint {
    current_styles: GlobalStyles,
    print_job_context: PrintJobRequest,
}

impl ProcessPrint {
    pub fn new() -> Self {
        Self {
            current_styles: GlobalStyles::default(),
            print_job_context: PrintJobRequest {
                printer: String::new(),
                sections: Vec::new(),
                options: Default::default(),
                paper_size: crate::PaperSize::DEFAULT,
            },
        }
    }

    pub fn generate_document(&mut self, print_job: &PrintJobRequest) -> Result<Vec<u8>, String> {
        if print_job.sections.is_empty() {
            return Err("No sections to print".to_string());
        }
        if print_job.printer.is_empty() {
            return Err("Printer not specified".to_string());
        }

        self.print_job_context = print_job.clone();
        let encoder = TextEncoder::from_code_page(&print_job.options.code_page);
        let effective_sections = self.build_effective_sections(print_job);

        let mut document: Vec<u8> = Vec::new();
        document.extend(PrinterControl::initialize());
        document.extend(print_job.options.code_page.escpos_command());

        for section in &effective_sections {
            let section_data = self.process_print_section(section, &encoder)?;
            document.extend(section_data);
        }

        Ok(document)
    }

    fn build_effective_sections(&self, print_job: &PrintJobRequest) -> Vec<PrintSections> {
        let mut sections = print_job.sections.clone();

        if print_job.options.beep {
            sections.push(PrintSections::Beep(Beep {
                times: AUTO_BEEP_TIMES,
                duration: AUTO_BEEP_DURATION,
            }));
        }
        if print_job.options.cut_paper {
            sections.push(PrintSections::Cut(Cut {
                mode: AUTO_CUT_MODE.to_string(),
                feed: AUTO_CUT_FEED,
            }));
        }
        if print_job.options.open_cash_drawer {
            sections.push(PrintSections::Drawer(Drawer {
                pin: AUTO_DRAWER_PIN,
                pulse_time: AUTO_DRAWER_PULSE_TIME,
            }));
        }

        sections
    }

    fn process_print_section(
        &mut self,
        section: &PrintSections,
        encoder: &TextEncoder,
    ) -> Result<Vec<u8>, String> {
        match section {
            PrintSections::Title(title) => process_title(title, &self.current_styles, encoder),
            PrintSections::Subtitle(subtitle) => {
                process_subtitle(subtitle, &self.current_styles, encoder)
            }
            PrintSections::Text(text) => process_text(text, &self.current_styles, encoder),
            PrintSections::Line(line) => process_line(
                line,
                &self.current_styles,
                self.print_job_context.paper_size.chars_per_line(),
            ),
            PrintSections::Feed(feed) => PrinterControl::process_feed(feed),
            PrintSections::Cut(cut) => PrinterControl::process_cut(cut),
            PrintSections::Beep(beep) => PrinterControl::process_beep(beep),
            PrintSections::Drawer(drawer) => PrinterControl::process_drawer(drawer),
            PrintSections::GlobalStyles(styles) => {
                let diff = get_styles_diff(&self.current_styles, styles);
                self.current_styles = styles.clone();
                Ok(diff)
            }
            PrintSections::Qr(qr) => qr_cmd::process_section(qr, &self.current_styles),
            PrintSections::Barcode(barcode) => {
                barcode_cmd::process_section(barcode, &self.current_styles)
            }
            PrintSections::Pdf417(pdf417) => pdf417_cmd::process_section(pdf417),
            PrintSections::DataMatrix(data_matrix) => data_matrix_cmd::process_section(data_matrix),
            PrintSections::Image(imagen) => {
                image_cmd::process_section(imagen, self.print_job_context.paper_size.pixels_width())
            }
            PrintSections::Logo(logo) => logo_cmd::process_section(logo),
            PrintSections::Table(table) => table_cmd::process_section(
                table,
                self.print_job_context.paper_size.chars_per_line(),
                encoder,
            ),
        }
    }
}

#[cfg(test)]
mod tests;
