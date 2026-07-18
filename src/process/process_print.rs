use crate::commands_esc_pos::codes::aztec::aztec as aztec_cmd;
use crate::commands_esc_pos::codes::barcode::barcode as barcode_cmd;
use crate::commands_esc_pos::codes::composite::composite as composite_cmd;
use crate::commands_esc_pos::codes::data_matrix::data_matrix as data_matrix_cmd;
use crate::commands_esc_pos::codes::gs1_databar_2d::gs1_databar_2d as gs1_databar_2d_cmd;
use crate::commands_esc_pos::codes::maxicode::maxicode as maxicode_cmd;
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
use crate::models::print_sections::{GlobalStyles, PrintSections};

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
        let encoder = TextEncoder::from_code_page(&print_job.options);

        let mut document: Vec<u8> = Vec::new();
        document.extend(PrinterControl::initialize());
        document.extend(print_job.options.escpos_command());

        for section in &print_job.sections {
            let section_data = self.process_print_section(section, &encoder)?;
            document.extend(section_data);
        }

        Ok(document)
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
            PrintSections::Beep2(beep) => PrinterControl::process_beep2(beep),
            PrintSections::Drawer(drawer) => PrinterControl::process_drawer(drawer),
            PrintSections::GlobalStyles(styles) => {
                // `reset` toma prioridad: reinicia la impresora (ESC @), vuelve a
                // aplicar el code page (que ESC @ borra) e ignora el resto de campos.
                if styles.reset.unwrap_or(false) {
                    let mut output = PrinterControl::initialize();
                    output.extend(self.print_job_context.options.escpos_command());
                    self.current_styles = GlobalStyles::default();
                    Ok(output)
                } else {
                    let diff = get_styles_diff(&self.current_styles, styles);
                    self.current_styles = styles.clone();
                    Ok(diff)
                }
            }
            PrintSections::Qr(qr) => qr_cmd::process_section(qr, &self.current_styles),
            PrintSections::Barcode(barcode) => {
                barcode_cmd::process_section(barcode, &self.current_styles)
            }
            PrintSections::Pdf417(pdf417) => pdf417_cmd::process_section(pdf417),
            PrintSections::DataMatrix(data_matrix) => data_matrix_cmd::process_section(data_matrix),
            PrintSections::Aztec(aztec) => aztec_cmd::process_section(aztec, &self.current_styles),
            PrintSections::Gs1Databar2d(databar) => {
                gs1_databar_2d_cmd::process_section(databar, &self.current_styles)
            }
            PrintSections::MaxiCode(maxicode) => {
                maxicode_cmd::process_section(maxicode, &self.current_styles)
            }
            PrintSections::Composite(composite) => {
                composite_cmd::process_section(composite, &self.current_styles)
            }
            PrintSections::Image(imagen) => {
                image_cmd::process_section(imagen, self.print_job_context.paper_size.pixels_width())
            }
            PrintSections::Logo(logo) => logo_cmd::process_section(logo),
            PrintSections::Table(table) => table_cmd::process_section(
                table,
                self.print_job_context.paper_size.chars_per_line(),
                encoder,
            ),
            PrintSections::LineSpacing(ls) => PrinterControl::process_line_spacing(ls),
            PrintSections::CharSpacing(cs) => PrinterControl::process_char_spacing(cs),
            PrintSections::Position(p) => PrinterControl::process_position(p),
            PrintSections::TabStops(t) => PrinterControl::process_tab_stops(t),
            PrintSections::LeftMargin(m) => PrinterControl::process_left_margin(m),
            PrintSections::PrintAreaWidth(w) => PrinterControl::process_print_area_width(w),
        }
    }
}
