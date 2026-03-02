use crate::commands_esc_pos::codes::barcode::{
    Barcode as EscPosBarcode, BarcodeTextPosition, BarcodeType,
};
use crate::commands_esc_pos::codes::data_matrix::data_matrix::DataMatrix;
use crate::commands_esc_pos::codes::data_matrix::data_matrix_size::DataMatrixSize;
use crate::commands_esc_pos::codes::pdf417::{PDF417ErrorCorrection, PDF417};
use crate::commands_esc_pos::codes::qr::{QRErrorCorrection, QRModel, QRSize, QR};
use crate::commands_esc_pos::control::printer_control::PrinterControl;
use crate::commands_esc_pos::image_escpos::{
    Image as ImageCode, ImageAlignment, ImageMode, Logo as EscPosLogo,
};
use crate::commands_esc_pos::text::table::process_table;
use crate::commands_esc_pos::text::text_type::TextType;
use crate::models::print_job_request::PrintJobRequest;
use crate::models::print_sections::{
    Barcode, Beep, Cut, DataMatrixModel, Drawer, Feed, GlobalStyles, Image, Logo, Pdf417,
    PrintSections, Qr, Subtitle, Table, Text, Title,
};

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
                paper_size: crate::PaperSize::Mm80, // Default to 80mm
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

        let mut document: Vec<u8> = Vec::new();

        document.extend(PrinterControl::initialize());

        for section in &print_job.sections {
            let section_data = self.process_print_section(section)?;
            document.extend(section_data);
        }

        if self.print_job_context.options.cut_paper {
            document.extend(PrinterControl::feed_paper(5));
            document.extend(PrinterControl::cut_paper_with_feed(65, 0));
        }

        if self.print_job_context.options.open_cash_drawer {
            document.extend(PrinterControl::open_cash_drawer_pin2(100));
        }

        Ok(document)
    }

    fn process_print_section(&mut self, print_section: &PrintSections) -> Result<Vec<u8>, String> {
        match print_section {
            PrintSections::Title(title) => self.process_title(title),
            PrintSections::Subtitle(subtitle) => self.process_subtitle(subtitle),
            PrintSections::Text(text) => self.process_text(text),
            PrintSections::Feed(feed) => self.process_feed(feed),
            PrintSections::Cut(cut) => self.process_cut(cut),
            PrintSections::Beep(beep) => self.process_beep(beep),
            PrintSections::Drawer(drawer) => self.process_drawer(drawer),
            PrintSections::GlobalStyles(styles) => self.set_global_styles(styles),
            PrintSections::Barcode(barcode) => self.process_barcode(barcode),
            PrintSections::Qr(qr) => self.process_qr(qr),
            PrintSections::Pdf417(pdf417) => self.process_pdf417(pdf417),
            PrintSections::Image(imagen) => self.process_imagen(imagen),
            PrintSections::Logo(logo) => self.process_logo(logo),
            PrintSections::DataMatrix(data_matrix) => self.process_data_matrix(data_matrix),
            PrintSections::Table(table) => self.process_table_fn(table),
            PrintSections::Line(line) => self.process_line(line),
        }
    }

    /// Procesa encabezado (centrado, doble tamaño)
    fn process_title(&mut self, title: &Title) -> Result<Vec<u8>, String> {
        let mut output = Vec::new();

        // Usar estilos proporcionados o estilos globales actuales
        let base_styles = title
            .styles
            .as_ref()
            .cloned()
            .unwrap_or(self.current_styles.clone());

        // Forzar tamaño doble y centrado (si lo quieres obligar, pero al menos el size)
        let mut effective_styles = base_styles;
        effective_styles.size = Some("double".to_string());
        effective_styles.align = Some("center".to_string());

        let diff_on = self.get_styles_diff(&self.current_styles, &effective_styles);
        output.extend_from_slice(&diff_on);

        // Texto
        let clean_text = Self::remove_accents(&title.text);
        output.extend_from_slice(clean_text.as_bytes());
        output.extend_from_slice(b"\n");

        // Resetear a estilos globales
        let diff_off = self.get_styles_diff(&effective_styles, &self.current_styles);
        output.extend_from_slice(&diff_off);

        Ok(output)
    }

    /// Procesa subtítulo (tamaño normal, negrita)
    fn process_subtitle(&mut self, subtitle: &Subtitle) -> Result<Vec<u8>, String> {
        let mut output = Vec::new();

        // Usar estilos proporcionados o estilos globales actuales
        let base_styles = subtitle
            .styles
            .as_ref()
            .cloned()
            .unwrap_or(self.current_styles.clone());

        // Forzar estilos: tamaño height y negrita
        let mut effective_styles = base_styles;
        effective_styles.size = Some("height".to_string());
        effective_styles.bold = Some(true);

        let diff_on = self.get_styles_diff(&self.current_styles, &effective_styles);
        output.extend_from_slice(&diff_on);

        // Texto
        let clean_text = Self::remove_accents(&subtitle.text);
        output.extend_from_slice(clean_text.as_bytes());
        output.extend_from_slice(b"\n");

        // Resetear a estilos globales
        let diff_off = self.get_styles_diff(&effective_styles, &self.current_styles);
        output.extend_from_slice(&diff_off);

        Ok(output)
    }

    /// Procesa texto (estilos libres)
    fn process_text(&mut self, text: &Text) -> Result<Vec<u8>, String> {
        let mut output = Vec::new();

        // Usar estilos como están
        let effective_styles = text
            .styles
            .as_ref()
            .cloned()
            .unwrap_or(self.current_styles.clone());

        let diff_on = self.get_styles_diff(&self.current_styles, &effective_styles);
        output.extend_from_slice(&diff_on);

        // Texto
        let clean_text = Self::remove_accents(&text.text);
        output.extend_from_slice(clean_text.as_bytes());
        output.extend_from_slice(b"\n");

        // Resetear a estilos globales
        let diff_off = self.get_styles_diff(&effective_styles, &self.current_styles);
        output.extend_from_slice(&diff_off);

        Ok(output)
    }

    /// Procesa feed de papel
    fn process_feed(&mut self, feed: &Feed) -> Result<Vec<u8>, String> {
        match feed.feed_type.as_str() {
            "lines" => Ok(PrinterControl::feed_paper(feed.value)),
            "dots" => Ok(PrinterControl::feed_paper_dots(feed.value)),
            "line_feed" => Ok(PrinterControl::line_feed_multiple(feed.value as usize)),
            _ => Err("Unknown feed type".to_string()),
        }
    }

    /// Procesa corte de papel
    fn process_cut(&mut self, cut: &Cut) -> Result<Vec<u8>, String> {
        if !self.print_job_context.options.cut_paper {
            return Ok(PrinterControl::line_feed_multiple(8));
        }

        let mode_u8 = match cut.mode.as_str() {
            "full" => 66,
            "partial" => 65,
            "partial_alt" => 65,
            "partial_alt2" => 66,
            _ => 65, // default partial
        };
        Ok(PrinterControl::cut_paper_with_feed(mode_u8, cut.feed))
    }

    /// Procesa beep
    fn process_beep(&mut self, beep: &Beep) -> Result<Vec<u8>, String> {
        // Verificar si el beep está habilitado en las opciones
        if !self.print_job_context.options.beep {
            return Ok(Vec::new());
        }

        let mut times = beep.times;
        if times <= 0 {
            times = 1;
        }
        let mut duration = beep.duration;
        if duration <= 0 {
            duration = 100;
        }
        Ok(PrinterControl::beep_custom(times, duration))
    }

    /// Procesa cajón de dinero
    fn process_drawer(&mut self, drawer: &Drawer) -> Result<Vec<u8>, String> {
        if drawer.pin == 2 {
            Ok(PrinterControl::open_cash_drawer_pin2(drawer.pulse_time))
        } else {
            Ok(PrinterControl::open_cash_drawer_pin5(drawer.pulse_time))
        }
    }

    /// Procesa código de barras
    fn process_barcode(&mut self, barcode: &Barcode) -> Result<Vec<u8>, String> {
        let barcode_type = match barcode.barcode_type.as_str() {
            "UPC-A" => BarcodeType::UpcA,
            "UPC-E" => BarcodeType::UpcE,
            "EAN13" => BarcodeType::Ean13,
            "EAN8" => BarcodeType::Ean8,
            "CODE39" => BarcodeType::Code39,
            "ITF" => BarcodeType::Itf,
            "CODABAR" => BarcodeType::Codabar,
            "CODE93" => BarcodeType::Code93,
            "CODE128" => BarcodeType::Code128,
            _ => BarcodeType::Code128, // default
        };

        let text_position = match barcode.text_position.as_str() {
            "none" => BarcodeTextPosition::NotPrinted,
            "above" => BarcodeTextPosition::Above,
            "below" => BarcodeTextPosition::Below,
            "both" => BarcodeTextPosition::Both,
            _ => BarcodeTextPosition::NotPrinted, // default
        };

        let esc_pos_barcode = EscPosBarcode::new(barcode_type, barcode.data.clone())
            .set_height(barcode.height)
            .set_width(barcode.width)
            .set_text_position(text_position);

        let mut data = esc_pos_barcode.get_command();
        data.extend_from_slice(b"\n");
        Ok(data)
    }

    /// Procesa código QR
    fn process_qr(&mut self, qr: &Qr) -> Result<Vec<u8>, String> {
        let model = if qr.model == 1 {
            QRModel::Model1
        } else {
            QRModel::Model2
        };

        let size = match qr.size {
            1 => QRSize::Size1,
            2 => QRSize::Size2,
            3 => QRSize::Size3,
            4 => QRSize::Size4,
            5 => QRSize::Size5,
            6 => QRSize::Size6,
            7 => QRSize::Size7,
            8 => QRSize::Size8,
            9 => QRSize::Size9,
            10 => QRSize::Size10,
            11 => QRSize::Size11,
            12 => QRSize::Size12,
            13 => QRSize::Size13,
            14 => QRSize::Size14,
            15 => QRSize::Size15,
            16 => QRSize::Size16,
            _ => QRSize::Size6, // default
        };

        let error_correction = match qr.error_correction.as_str() {
            "L" => QRErrorCorrection::L,
            "M" => QRErrorCorrection::M,
            "Q" => QRErrorCorrection::Q,
            "H" => QRErrorCorrection::H,
            _ => QRErrorCorrection::M, // default
        };

        let esc_pos_qr = QR::new(qr.data.clone())
            .set_model(model)
            .set_size(size)
            .set_error_correction(error_correction);

        let mut temp_styles = self.current_styles.clone();
        if let Some(ref align) = qr.align {
            temp_styles.align = Some(align.clone());
        }

        let diff_on = self.get_styles_diff(&self.current_styles, &temp_styles);
        let diff_off = self.get_styles_diff(&temp_styles, &self.current_styles);

        let mut data = Vec::new();
        data.extend_from_slice(&diff_on);
        data.extend_from_slice(&esc_pos_qr.get_command());
        data.extend_from_slice(b"\n");
        data.extend_from_slice(&diff_off);

        Ok(data)
    }

    /// Procesa código PDF417
    fn process_pdf417(&mut self, pdf417: &Pdf417) -> Result<Vec<u8>, String> {
        let error_correction = match pdf417.error_correction {
            0 => PDF417ErrorCorrection::Level0,
            1 => PDF417ErrorCorrection::Level1,
            2 => PDF417ErrorCorrection::Level2,
            3 => PDF417ErrorCorrection::Level3,
            4 => PDF417ErrorCorrection::Level4,
            5 => PDF417ErrorCorrection::Level5,
            6 => PDF417ErrorCorrection::Level6,
            7 => PDF417ErrorCorrection::Level7,
            8 => PDF417ErrorCorrection::Level8,
            _ => PDF417ErrorCorrection::Level1, // default
        };

        let esc_pos_pdf417 = PDF417::new(pdf417.data.clone())
            .set_columns(pdf417.columns)
            .set_rows(pdf417.rows)
            .set_height(pdf417.height)
            .set_width(pdf417.width)
            .set_error_correction(error_correction);

        let mut data = esc_pos_pdf417.get_command();
        data.extend_from_slice(b"\n");
        Ok(data)
    }

    /// Procesa imagen
    fn process_imagen(&mut self, imagen: &Image) -> Result<Vec<u8>, String> {
        let alignment = match imagen.align.as_str() {
            "left" => ImageAlignment::Left,
            "center" => ImageAlignment::Center,
            "right" => ImageAlignment::Right,
            _ => ImageAlignment::Center,
        };

        let mode = match imagen.size.as_str() {
            "normal" => ImageMode::Normal,
            "double_width" => ImageMode::DoubleWidth,
            "double_height" => ImageMode::DoubleHeight,
            "quadruple" => ImageMode::Quadruple,
            _ => ImageMode::Normal,
        };

        let max_width = if imagen.max_width > self.print_job_context.paper_size.pixels_width()
            || imagen.max_width <= 0
        {
            self.print_job_context.paper_size.pixels_width() as u32
        } else {
            imagen.max_width as u32
        };

        let image = ImageCode::new(&imagen.data, max_width)
            .map_err(|e| format!("Failed to create image: {}", e))?
            .set_alignment(alignment)
            .set_mode(mode)
            .set_use_dithering(imagen.dithering);

        let mut cmd = image.get_command()?;
        cmd.extend_from_slice(b"\n");
        Ok(cmd)
    }

    /// Procesa logo
    fn process_logo(&mut self, logo: &Logo) -> Result<Vec<u8>, String> {
        let mode = match logo.mode.as_str() {
            "normal" => ImageMode::Normal,
            "double_width" => ImageMode::DoubleWidth,
            "double_height" => ImageMode::DoubleHeight,
            "quadruple" => ImageMode::Quadruple,
            _ => ImageMode::Normal,
        };

        let esc_pos_logo = EscPosLogo::new(logo.key_code).set_mode(mode);

        let mut data = esc_pos_logo.get_print_command();
        data.extend_from_slice(b"\n");
        Ok(data)
    }

    /// Procesa código DataMatrix
    fn process_data_matrix(&mut self, data_matrix: &DataMatrixModel) -> Result<Vec<u8>, String> {
        let size = match data_matrix.size {
            1 => DataMatrixSize::Size1,
            2 => DataMatrixSize::Size2,
            3 => DataMatrixSize::Size3,
            4 => DataMatrixSize::Size4,
            5 => DataMatrixSize::Size5,
            6 => DataMatrixSize::Size6,
            7 => DataMatrixSize::Size7,
            8 => DataMatrixSize::Size8,
            9 => DataMatrixSize::Size9,
            10 => DataMatrixSize::Size10,
            11 => DataMatrixSize::Size11,
            12 => DataMatrixSize::Size12,
            13 => DataMatrixSize::Size13,
            14 => DataMatrixSize::Size14,
            15 => DataMatrixSize::Size15,
            16 => DataMatrixSize::Size16,
            _ => DataMatrixSize::Size6,
        };
        let esc_pos_data_matrix = DataMatrix::new(data_matrix.data.clone()).set_size(size);

        let mut data = esc_pos_data_matrix.get_command();
        data.extend_from_slice(b"\n");
        Ok(data)
    }

    fn process_table_fn(&mut self, table: &Table) -> Result<Vec<u8>, String> {
        process_table(
            table,
            self.print_job_context.paper_size.pixels_width(),
            table.truncate,
        )
    }

    /// Procesa línea horizontal
    fn process_line(
        &mut self,
        line: &crate::models::print_sections::Line,
    ) -> Result<Vec<u8>, String> {
        let mut output = Vec::new();

        // Calcular el número de caracteres según el ancho del papel y el tamaño de fuente
        let char_count = self.calculate_line_width();

        // Obtener el primer carácter (o usar '-' por defecto)
        let character = line.character.chars().next().unwrap_or('-');

        // Crear la línea repitiendo el carácter
        let line_text = character.to_string().repeat(char_count);

        // Usar los estilos globales actuales
        output.extend_from_slice(line_text.as_bytes());
        output.extend_from_slice(b"\n");

        Ok(output)
    }

    /// Calcula el ancho de línea en caracteres según el papel y fuente actual
    fn calculate_line_width(&self) -> usize {
        // Ajustar según el tamaño de fuente actual
        let current_size = self
            .current_styles
            .size
            .as_deref()
            .unwrap_or("normal")
            .to_lowercase();
        let width_multiplier = match current_size.as_str() {
            "width" | "double" => 0.5, // DoubleWidth or DoubleSize reduce characters per line
            _ => 1.0,
        };

        // Ajustar según el tipo de fuente
        let current_font = self
            .current_styles
            .font
            .as_deref()
            .unwrap_or("a")
            .to_lowercase();
        let font_multiplier = match current_font.as_str() {
            "b" => 1.3, // Font B es más pequeña, más caracteres
            "c" => 1.5, // Font C es aún más pequeña
            _ => 1.0,   // Font A
        };

        let calculated_width = (self.print_job_context.paper_size.chars_per_line() as f32
            * width_multiplier
            * font_multiplier) as usize;

        // Asegurar al menos 10 caracteres
        calculated_width.max(10)
    }

    fn set_global_styles(&mut self, styles: &GlobalStyles) -> Result<Vec<u8>, String> {
        let diff = self.get_styles_diff(&self.current_styles, styles);
        self.current_styles = styles.clone();
        Ok(diff)
    }

    fn get_styles_diff(&self, old: &GlobalStyles, new: &GlobalStyles) -> Vec<u8> {
        let mut output = Vec::new();

        // Helper functions to get effective values with defaults
        let get_bool = |opt: &Option<bool>| opt.unwrap_or(false);
        let get_string =
            |opt: &Option<String>, default: &str| opt.as_deref().unwrap_or(default).to_lowercase();

        let old_bold = get_bool(&old.bold);
        let new_bold = get_bool(&new.bold);
        if old_bold != new_bold {
            if new_bold {
                output.extend_from_slice(TextType::BoldOn.command());
            } else {
                output.extend_from_slice(TextType::BoldOff.command());
            }
        }

        let old_underline = get_bool(&old.underline);
        let new_underline = get_bool(&new.underline);
        if old_underline != new_underline {
            if new_underline {
                output.extend_from_slice(TextType::UnderlineOn.command());
            } else {
                output.extend_from_slice(TextType::UnderlineOff.command());
            }
        }

        let old_italic = get_bool(&old.italic);
        let new_italic = get_bool(&new.italic);
        if old_italic != new_italic {
            if new_italic {
                output.extend_from_slice(TextType::ItalicOn.command());
            } else {
                output.extend_from_slice(TextType::ItalicOff.command());
            }
        }

        let old_invert = get_bool(&old.invert);
        let new_invert = get_bool(&new.invert);
        if old_invert != new_invert {
            if new_invert {
                output.extend_from_slice(TextType::InvertOn.command());
            } else {
                output.extend_from_slice(TextType::InvertOff.command());
            }
        }

        let old_rotate = get_bool(&old.rotate);
        let new_rotate = get_bool(&new.rotate);
        if old_rotate != new_rotate {
            if new_rotate {
                output.extend_from_slice(TextType::RotateOn.command());
            } else {
                output.extend_from_slice(TextType::RotateOff.command());
            }
        }

        let old_upside_down = get_bool(&old.upside_down);
        let new_upside_down = get_bool(&new.upside_down);
        if old_upside_down != new_upside_down {
            if new_upside_down {
                output.extend_from_slice(TextType::UpsideDownOn.command());
            } else {
                output.extend_from_slice(TextType::UpsideDownOff.command());
            }
        }

        let old_font = get_string(&old.font, "a");
        let new_font = get_string(&new.font, "a");
        if old_font != new_font {
            if new_font == "a" {
                output.extend_from_slice(TextType::FontA.command());
            } else if new_font == "b" {
                output.extend_from_slice(TextType::FontB.command());
            } else if new_font == "c" {
                output.extend_from_slice(TextType::FontC.command());
            }
        }

        let old_size = get_string(&old.size, "normal");
        let new_size = get_string(&new.size, "normal");
        if old_size != new_size {
            if new_size == "normal" {
                output.extend_from_slice(TextType::Normal.command());
            } else if new_size == "width" {
                output.extend_from_slice(TextType::DoubleWidth.command());
            } else if new_size == "height" {
                output.extend_from_slice(TextType::DoubleHeight.command());
            } else if new_size == "double" {
                output.extend_from_slice(TextType::DoubleSize.command());
            }
        }

        let old_align = get_string(&old.align, "left");
        let new_align = get_string(&new.align, "left");
        if old_align != new_align {
            if new_align == "left" {
                output.extend_from_slice(TextType::AlignLeft.command());
            } else if new_align == "center" {
                output.extend_from_slice(TextType::AlignCenter.command());
            } else if new_align == "right" {
                output.extend_from_slice(TextType::AlignRight.command());
            }
        }

        output
    }

    /// Remueve tildes y caracteres especiales del texto
    fn remove_accents(text: &str) -> String {
        text.chars()
            .map(|c| match c {
                'á' | 'Á' => 'a',
                'é' | 'É' => 'e',
                'í' | 'Í' => 'i',
                'ó' | 'Ó' => 'o',
                'ú' | 'Ú' => 'u',
                'ñ' => 'n',
                'Ñ' => 'N',
                'ü' | 'Ü' => 'u',
                '¿' => '?',
                '¡' => '!',
                _ => c,
            })
            .collect()
    }
}
