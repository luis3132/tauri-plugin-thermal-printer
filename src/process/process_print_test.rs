use crate::commands_esc_pos::codes::aztec::Aztec;
use crate::commands_esc_pos::codes::barcode::{Barcode, BarcodeTextPosition, BarcodeType};
use crate::commands_esc_pos::codes::composite::Composite;
use crate::commands_esc_pos::codes::gs1_databar_2d::{Gs1Databar2d, Gs1Databar2dType};
use crate::commands_esc_pos::codes::maxicode::MaxiCode;
use crate::commands_esc_pos::codes::qr::{QRErrorCorrection, QRModel, QRSize, QR};
use crate::commands_esc_pos::control::printer_control::PrinterControl;
use crate::commands_esc_pos::image_escpos::logo::Logo as NvLogo;
use crate::commands_esc_pos::image_escpos::{Image, ImageAlignment, ImageMode};
use crate::commands_esc_pos::text::encoder::TextEncoder;
use crate::commands_esc_pos::text::table;
use crate::commands_esc_pos::text::text_type::TextType;
use crate::models::print_job_request::PrintJobRequest;
use crate::models::print_sections::{Image as ImageSection, Table, Text};
use crate::TestPrintRequest;

pub struct TestPrinter {
    print_job_context: TestPrintRequest,
}

impl TestPrinter {
    pub fn new() -> Self {
        Self {
            print_job_context: TestPrintRequest {
                printer_info: PrintJobRequest {
                    printer: "".to_string(),
                    sections: vec![],
                    options: Default::default(),
                    paper_size: crate::PaperSize::DEFAULT,
                },
                include_text: true,
                include_custom_text: false,
                custom_text: None,
                include_text_styles: true,
                include_alignment: true,
                include_columns: true,
                include_separators: true,
                include_barcode: true,
                include_barcode_types: false,
                include_qr: true,
                include_image: false,
                image_base64: None,
                test_all_fonts: true,
                test_invert: true,
                test_rotate: true,
                test_double_strike: true,
                test_spacing: true,
                test_positioning: true,
                test_beep2: true,
                test_feed: true,
                test_cash_drawer: true,
                test_logo: true,
                include_beep: true,
                cut_paper: true,
            },
        }
    }

    /// Genera el documento de prueba completo
    pub fn generate_test_document(
        &mut self,
        request: &TestPrintRequest,
    ) -> Result<Vec<u8>, String> {
        let mut document: Vec<u8> = Vec::new();

        self.print_job_context = request.clone();
        let encoder = TextEncoder::from_code_page(&request.printer_info.options);

        // Inicializar impresora y seleccionar página de código
        document.extend(PrinterControl::initialize());
        document.extend(request.printer_info.options.escpos_command());
        document.extend(PrinterControl::line_feed());

        // ==================== ENCABEZADO ====================
        if request.include_text {
            self.add_header(&mut document)?;
        }

        if request.include_custom_text {
            if let Some(custom_text) = request
                .custom_text
                .as_deref()
                .map(str::trim)
                .filter(|text| !text.is_empty())
            {
                self.add_custom_text_section(&mut document, custom_text, &encoder)?;
            }
        }

        if request.include_separators {
            self.add_double_line(&mut document);
        }

        // ==================== ESTILOS DE TEXTO ====================
        if request.include_text_styles {
            self.add_text_styles_section(&mut document)?;
        }

        // ==================== FUENTES ====================
        if request.test_all_fonts {
            self.add_fonts_section(&mut document)?;
        }

        // ==================== ALINEACIÓN ====================
        if request.include_alignment {
            self.add_alignment_section(&mut document)?;
        }

        // ==================== INVERSIÓN Y ROTACIÓN ====================
        if request.test_invert {
            self.add_invert_section(&mut document)?;
        }

        if request.test_rotate {
            self.add_rotate_section(&mut document)?;
        }

        // ==================== ESPACIADO ====================
        if request.test_spacing {
            self.add_spacing_section(&mut document)?;
        }

        // ==================== POSICIONAMIENTO ====================
        if request.test_positioning {
            self.add_positioning_section(&mut document)?;
        }

        // ==================== SEPARADORES ====================
        if request.include_separators {
            self.add_separators_section(&mut document)?;
        }

        // ==================== COLUMNAS ====================
        if request.include_columns {
            self.add_columns_section(&mut document, &encoder)?;
        }

        // ==================== CÓDIGOS DE BARRAS ====================
        if request.include_barcode {
            self.add_barcode_section(&mut document)?;
        }

        if request.include_barcode_types {
            self.add_barcode_types_section(&mut document)?;
            self.add_2d_codes_section(&mut document)?;
        }

        // ==================== CÓDIGO QR ====================
        if request.include_qr {
            self.add_qr_section(&mut document)?;
        }

        // ==================== IMAGEN ====================
        if request.include_image {
            if let Some(ref image_base64) = request.image_base64 {
                self.add_image_section(&mut document, image_base64)?;
            }
        }

        // ==================== LOGO NV (FS q / FS p) ====================
        if request.test_logo {
            if let Some(ref image_base64) = request.image_base64 {
                self.add_logo_section(&mut document, image_base64)?;
            }
        }

        // ==================== CONTROL DE IMPRESORA ====================
        if request.test_feed {
            self.add_feed_section(&mut document)?;
        }

        if request.test_cash_drawer {
            self.add_cash_drawer_section(&mut document)?;
        }

        // ==================== MENSAJE FINAL ====================
        self.add_footer(&mut document)?;

        // Beep al final si está habilitado
        if request.include_beep {
            document.extend(PrinterControl::beep_custom(3, 100));
        }

        // Buzzer genérico (ESC B) para impresoras que ignoran el beep de Epson
        if request.test_beep2 {
            document.extend(PrinterControl::beep_generic(2, 3));
        }

        // Avanzar papel antes de cortar
        let feed_lines = if request.cut_paper { 4 } else { 2 };
        document.extend(PrinterControl::feed_paper(feed_lines));

        // Cortar papel si está habilitado
        if request.cut_paper {
            document.extend(PrinterControl::cut_paper_with_feed(0, 4));
        } else {
            document.extend(PrinterControl::feed_paper(8));
        }

        Ok(document)
    }

    // ==================== SECCIONES INDIVIDUALES ====================

    fn add_header(&self, document: &mut Vec<u8>) -> Result<(), String> {
        document.extend(TextType::AlignCenter.command());
        document.extend(TextType::DoubleSize.command());
        document.extend(b"PRUEBA DE IMPRESORA\n");
        document.extend(TextType::Normal.command());
        document.extend(TextType::AlignLeft.command());
        document.extend(b"\n");
        Ok(())
    }

    fn add_custom_text_section(
        &self,
        document: &mut Vec<u8>,
        custom_text: &str,
        encoder: &TextEncoder,
    ) -> Result<(), String> {
        document.extend(TextType::BoldOn.command());
        document.extend(b">>> TEXTO PERSONALIZADO <<<\n");
        document.extend(TextType::BoldOff.command());
        document.extend(encoder.encode_text(custom_text)?);
        document.extend(b"\n\n");
        Ok(())
    }

    fn add_text_styles_section(&self, document: &mut Vec<u8>) -> Result<(), String> {
        document.extend(TextType::BoldOn.command());
        document.extend(b">>> ESTILOS DE TEXTO <<<\n");
        document.extend(TextType::BoldOff.command());
        document.extend(b"\n");

        // Negrita
        document.extend(TextType::BoldOn.command());
        document.extend(b"1. Texto en Negrita\n");
        document.extend(TextType::BoldOff.command());

        // Subrayado
        document.extend(TextType::UnderlineOn.command());
        document.extend(b"2. Texto Subrayado\n");
        document.extend(TextType::UnderlineOff.command());

        // Doble altura
        document.extend(TextType::DoubleHeight.command());
        document.extend(b"3. Doble Altura\n");
        document.extend(TextType::Normal.command());

        // Doble ancho
        document.extend(TextType::DoubleWidth.command());
        document.extend(b"4. Doble Ancho\n");
        document.extend(TextType::Normal.command());

        // Doble tamaño
        document.extend(TextType::DoubleSize.command());
        document.extend(b"5. Doble Tamano\n");
        document.extend(TextType::Normal.command());

        // Doble golpe (double-strike) — refuerza la negrita en genéricas
        if self.print_job_context.test_double_strike {
            document.extend(TextType::DoubleStrikeOn.command());
            document.extend(b"6. Doble Golpe (double-strike)\n");
            document.extend(TextType::DoubleStrikeOff.command());
        }

        document.extend(b"\n");
        Ok(())
    }

    fn add_fonts_section(&self, document: &mut Vec<u8>) -> Result<(), String> {
        document.extend(TextType::BoldOn.command());
        document.extend(b">>> TIPOS DE FUENTE <<<\n");
        document.extend(TextType::BoldOff.command());

        document.extend(TextType::FontA.command());
        document.extend(b"Fuente A (por defecto)\n");

        document.extend(TextType::FontB.command());
        document.extend(b"Fuente B (mas pequena)\n");

        document.extend(TextType::FontC.command());
        document.extend(b"Fuente C (condensada)\n");

        document.extend(TextType::FontA.command());
        document.extend(b"\n");
        Ok(())
    }

    fn add_alignment_section(&self, document: &mut Vec<u8>) -> Result<(), String> {
        document.extend(TextType::BoldOn.command());
        document.extend(b">>> ALINEACION <<<\n");
        document.extend(TextType::BoldOff.command());

        document.extend(TextType::AlignLeft.command());
        document.extend(b"Izquierda\n");

        document.extend(TextType::AlignCenter.command());
        document.extend(b"Centrado\n");

        document.extend(TextType::AlignRight.command());
        document.extend(b"Derecha\n");

        document.extend(TextType::AlignLeft.command());
        document.extend(b"\n");
        Ok(())
    }

    fn add_invert_section(&self, document: &mut Vec<u8>) -> Result<(), String> {
        document.extend(TextType::BoldOn.command());
        document.extend(b">>> INVERSION <<<\n");
        document.extend(TextType::BoldOff.command());

        document.extend(TextType::InvertOn.command());
        document.extend(b"Texto Invertido (fondo negro)\n");
        document.extend(TextType::InvertOff.command());
        document.extend(b"\n");
        Ok(())
    }

    fn add_rotate_section(&self, document: &mut Vec<u8>) -> Result<(), String> {
        document.extend(TextType::BoldOn.command());
        document.extend(b">>> ROTACION <<<\n");
        document.extend(TextType::BoldOff.command());

        document.extend(TextType::RotateOn.command());
        document.extend(b"Texto Rotado 90 grados\n");
        document.extend(TextType::RotateOff.command());
        document.extend(b"\n");
        Ok(())
    }

    fn add_spacing_section(&self, document: &mut Vec<u8>) -> Result<(), String> {
        document.extend(TextType::BoldOn.command());
        document.extend(b">>> ESPACIADO <<<\n");
        document.extend(TextType::BoldOff.command());

        // Interlineado apretado (ESC 3 n)
        document.extend(b"Interlineado normal:\n");
        document.extend(b"Linea 1\nLinea 2\n");

        document.extend(PrinterControl::set_line_spacing(Some(20)));
        document.extend(b"Interlineado apretado:\n");
        document.extend(b"Linea 1\nLinea 2\n");
        document.extend(PrinterControl::set_line_spacing(None)); // reset (ESC 2)

        // Espaciado de carácter (ESC SP n)
        document.extend(PrinterControl::set_char_spacing(3));
        document.extend(b"Caracteres separados\n");
        document.extend(PrinterControl::set_char_spacing(0));

        document.extend(b"\n");
        Ok(())
    }

    fn add_positioning_section(&self, document: &mut Vec<u8>) -> Result<(), String> {
        document.extend(TextType::BoldOn.command());
        document.extend(b">>> POSICIONAMIENTO <<<\n");
        document.extend(TextType::BoldOff.command());

        // Tabuladores (ESC D ... NUL) + HT (\t)
        document.extend(PrinterControl::set_tab_stops(&[10, 24]));
        document.extend(b"A\tB\tC\n");

        // Posición absoluta (ESC $)
        document.extend(b"Pos 0:");
        document.extend(PrinterControl::set_absolute_position(200));
        document.extend(b"Pos 200 dots\n");

        // Margen izquierdo y ancho de área (GS L / GS W)
        document.extend(PrinterControl::set_left_margin(60));
        document.extend(b"Con margen izquierdo\n");
        document.extend(PrinterControl::set_left_margin(0));

        document.extend(b"\n");
        Ok(())
    }

    fn add_separators_section(&self, document: &mut Vec<u8>) -> Result<(), String> {
        document.extend(TextType::BoldOn.command());
        document.extend(b">>> LINEAS SEPARADORAS <<<\n");
        document.extend(TextType::BoldOff.command());

        document.extend(b"Guiones:\n");
        self.add_dashed_line(document);

        document.extend(b"Igual:\n");
        self.add_double_line(document);

        document.extend(b"Asteriscos:\n");
        self.add_star_line(document);

        document.extend(b"\n");
        Ok(())
    }

    fn add_columns_section(
        &self,
        document: &mut Vec<u8>,
        encoder: &TextEncoder,
    ) -> Result<(), String> {
        document.extend(TextType::BoldOn.command());
        document.extend(b">>> TABLA CON COLUMNAS <<<\n");
        document.extend(TextType::BoldOff.command());

        let table = Table {
            columns: 3,
            truncate: true,
            word_wrap: None,
            column_widths: Some(vec![25, 8, 15]),
            header: Some(vec![
                Text {
                    text: "Producto".to_string(),
                    styles: None,
                },
                Text {
                    text: "Cant".to_string(),
                    styles: None,
                },
                Text {
                    text: "Precio".to_string(),
                    styles: None,
                },
            ]),
            body: vec![
                vec![
                    Text {
                        text: "Producto A".to_string(),
                        styles: None,
                    },
                    Text {
                        text: "2".to_string(),
                        styles: None,
                    },
                    Text {
                        text: "$10.50".to_string(),
                        styles: None,
                    },
                ],
                vec![
                    Text {
                        text: "Producto B".to_string(),
                        styles: None,
                    },
                    Text {
                        text: "1".to_string(),
                        styles: None,
                    },
                    Text {
                        text: "$25.00".to_string(),
                        styles: None,
                    },
                ],
                vec![
                    Text {
                        text: "Producto C Largo".to_string(),
                        styles: None,
                    },
                    Text {
                        text: "5".to_string(),
                        styles: None,
                    },
                    Text {
                        text: "$8.99".to_string(),
                        styles: None,
                    },
                ],
            ],
        };

        document.extend(table::process_table(
            &table,
            self.print_job_context
                .printer_info
                .paper_size
                .chars_per_line(),
            table.truncate,
            encoder,
        )?);
        self.add_dashed_line(document);

        // Totales
        let subtotal = self.create_receipt_line("SUBTOTAL:", "$44.49");
        document.extend(subtotal.as_bytes());
        document.extend(b"\n");

        let tax = self.create_receipt_line("IVA (16%):", "$7.12");
        document.extend(tax.as_bytes());
        document.extend(b"\n");

        self.add_double_line(document);

        let total = self.create_receipt_line("TOTAL:", "$51.61");
        document.extend(TextType::BoldOn.command());
        document.extend(TextType::DoubleHeight.command());
        document.extend(total.as_bytes());
        document.extend(b"\n");
        document.extend(TextType::Normal.command());

        document.extend(b"\n");
        Ok(())
    }

    fn add_barcode_section(&self, document: &mut Vec<u8>) -> Result<(), String> {
        document.extend(TextType::BoldOn.command());
        document.extend(b">>> CODIGOS DE BARRAS <<<\n");
        document.extend(TextType::BoldOff.command());
        document.extend(b"\n");

        document.extend(TextType::AlignLeft.command());
        document.extend(b"EAN13 Left:\n");
        let barcode = Barcode::new(BarcodeType::Ean13, "123456789012".to_string())
            .set_height(60)
            .set_width(2)
            .set_text_position(BarcodeTextPosition::Below);
        document.extend(barcode.get_command());

        document.extend(b"\n");

        document.extend(TextType::AlignCenter.command());
        document.extend(b"CODE128 Center:\n");

        let barcode = Barcode::new(BarcodeType::Code128, "123456789012".to_string())
            .set_height(60)
            .set_width(2)
            .set_text_position(BarcodeTextPosition::Below);
        document.extend(barcode.get_command());

        document.extend(b"\n");

        document.extend(TextType::AlignRight.command());
        document.extend(b"UPCA Right:\n");
        let barcode = Barcode::new(BarcodeType::UpcA, "123456789012".to_string())
            .set_height(60)
            .set_width(2)
            .set_text_position(BarcodeTextPosition::Below);
        document.extend(barcode.get_command());

        document.extend(b"\n\n");
        document.extend(TextType::AlignLeft.command());
        Ok(())
    }

    fn add_barcode_types_section(&self, document: &mut Vec<u8>) -> Result<(), String> {
        document.extend(TextType::AlignCenter.command());

        // EAN13
        document.extend(b"EAN13:\n");
        let ean13 = Barcode::new(BarcodeType::Ean13, "1234567890128".to_string())
            .set_height(50)
            .set_width(2)
            .set_text_position(BarcodeTextPosition::Below);
        document.extend(ean13.get_command());
        document.extend(b"\n\n");

        // CODE39
        document.extend(b"CODE39:\n");
        let code39 = Barcode::new(BarcodeType::Code39, "ABC123".to_string())
            .set_height(50)
            .set_width(2)
            .set_text_position(BarcodeTextPosition::Below);
        document.extend(code39.get_command());
        document.extend(b"\n\n");

        // GS1-128 (requiere firmware compatible)
        document.extend(b"GS1-128:\n");
        let gs1_128 = Barcode::new(BarcodeType::Gs1128, "12345678".to_string())
            .set_height(50)
            .set_width(2)
            .set_text_position(BarcodeTextPosition::Below);
        document.extend(gs1_128.get_command());
        document.extend(b"\n\n");

        // GS1 DataBar Omnidirectional (requiere firmware compatible)
        document.extend(b"GS1 DataBar Omni:\n");
        let databar = Barcode::new(BarcodeType::Gs1DatabarOmni, "1234567890123".to_string())
            .set_height(50)
            .set_width(2)
            .set_text_position(BarcodeTextPosition::Below);
        document.extend(databar.get_command());
        document.extend(b"\n\n");

        document.extend(TextType::AlignLeft.command());
        Ok(())
    }

    /// Códigos 2D avanzados (requieren firmware compatible; muchas impresoras
    /// básicas los ignoran).
    fn add_2d_codes_section(&self, document: &mut Vec<u8>) -> Result<(), String> {
        document.extend(TextType::AlignCenter.command());

        // Aztec Code
        document.extend(b"Aztec:\n");
        let aztec = Aztec::new("Aztec 123".to_string())
            .set_size(3)
            .set_error_correction(23);
        document.extend(aztec.get_command());
        document.extend(b"\n\n");

        // GS1 DataBar Stacked Omnidirectional
        document.extend(b"GS1 DataBar 2D:\n");
        let databar = Gs1Databar2d::new("1234567890123".to_string(), Gs1Databar2dType::StackedOmni)
            .set_width(2);
        document.extend(databar.get_command());
        document.extend(b"\n\n");

        // MaxiCode (modo 4)
        document.extend(b"MaxiCode:\n");
        let maxicode = MaxiCode::new("MaxiCode data".to_string()).set_mode(4);
        document.extend(maxicode.get_command());
        document.extend(b"\n\n");

        // Composite Symbology
        document.extend(b"Composite:\n");
        let composite = Composite::new("Composite data".to_string()).set_width(2);
        document.extend(composite.get_command());
        document.extend(b"\n\n");

        document.extend(TextType::AlignLeft.command());
        Ok(())
    }

    fn add_qr_section(&self, document: &mut Vec<u8>) -> Result<(), String> {
        document.extend(TextType::BoldOn.command());
        document.extend(b">>> CODIGO QR <<<\n");
        document.extend(TextType::BoldOff.command());
        document.extend(b"\n");

        document.extend(TextType::AlignCenter.command());
        document.extend(b"Escanea el QR:\n");

        let qr = QR::new("https://github.com/luis3132/tauri-plugin-thermal-printer".to_string())
            .set_size(QRSize::Size6)
            .set_error_correction(QRErrorCorrection::M)
            .set_model(QRModel::Model2);
        document.extend(qr.get_command());

        document.extend(b"\n");
        document.extend(TextType::AlignLeft.command());
        Ok(())
    }

    fn add_image_section(&self, document: &mut Vec<u8>, image_base64: &str) -> Result<(), String> {
        document.extend(TextType::BoldOn.command());
        document.extend(b">>> IMAGEN <<<\n");
        document.extend(TextType::BoldOff.command());
        document.extend(b"\n");

        match Image::new(image_base64, 384) {
            Ok(img) => {
                let img = img
                    .set_alignment(ImageAlignment::Center)
                    .set_mode(ImageMode::Normal)
                    .set_use_dithering(true);

                match img.get_command() {
                    Ok(cmd) => {
                        document.extend(cmd);
                        document.extend(b"\n");
                    }
                    Err(_) => {
                        document.extend(b"Error al procesar imagen\n");
                    }
                }
            }
            Err(_) => {
                document.extend(b"Error al crear imagen\n");
            }
        }

        Ok(())
    }

    /// Guarda la imagen como logo en memoria NV (`FS q`) y luego la imprime (`FS p`).
    /// Demuestra el flujo completo de `set_logo`. Muchas impresoras genéricas soportan
    /// `FS q`/`FS p`, pero algunas no tienen memoria NV — en ese caso se ignora.
    fn add_logo_section(&self, document: &mut Vec<u8>, image_base64: &str) -> Result<(), String> {
        document.extend(TextType::BoldOn.command());
        document.extend(b">>> LOGO NV (FS q / FS p) <<<\n");
        document.extend(TextType::BoldOff.command());
        document.extend(b"\n");

        let paper_width = self
            .print_job_context
            .printer_info
            .paper_size
            .pixels_width();

        // La imagen para almacenar reutiliza el formato de sección Image.
        let logo_image = ImageSection {
            data: image_base64.to_string(),
            max_width: 0,
            align: "center".to_string(),
            dithering: true,
            size: "normal".to_string(),
        };

        // 1) Guardar en memoria NV con la clave 1 (FS q).
        match NvLogo::get_define_command(&logo_image, paper_width) {
            Ok(define_cmd) => {
                document.extend(define_cmd);

                // 2) Imprimir el logo recién guardado (FS p n=1).
                document.extend(TextType::AlignCenter.command());
                document.extend(NvLogo::new(1).get_print_command());
                document.extend(b"\n");
                document.extend(TextType::AlignLeft.command());
            }
            Err(_) => {
                document.extend(b"Error al guardar el logo\n");
            }
        }

        document.extend(b"\n");
        Ok(())
    }

    fn add_feed_section(&self, document: &mut Vec<u8>) -> Result<(), String> {
        document.extend(TextType::BoldOn.command());
        document.extend(b">>> CONTROL DE PAPEL <<<\n");
        document.extend(TextType::BoldOff.command());

        document.extend(b"Avance de 2 lineas...\n");
        document.extend(PrinterControl::feed_paper(2));

        document.extend(b"Avance de papel en puntos...\n");
        document.extend(PrinterControl::feed_paper_dots(50));
        document.extend(b"\n");
        Ok(())
    }

    fn add_cash_drawer_section(&self, document: &mut Vec<u8>) -> Result<(), String> {
        document.extend(TextType::BoldOn.command());
        document.extend(b">>> CAJON DE DINERO <<<\n");
        document.extend(TextType::BoldOff.command());

        document.extend(b"Abriendo cajon (Pin 2)...\n");
        document.extend(PrinterControl::open_cash_drawer_pin2(100));
        document.extend(b"\n");
        Ok(())
    }

    fn add_footer(&self, document: &mut Vec<u8>) -> Result<(), String> {
        document.extend(b"\n");
        document.extend(TextType::AlignCenter.command());
        self.add_star_line(document);

        document.extend(TextType::DoubleHeight.command());
        document.extend(b"PRUEBA COMPLETADA\n");
        document.extend(TextType::Normal.command());

        self.add_star_line(document);

        let datetime = format_utc_datetime();
        document.extend(datetime.as_bytes());
        document.extend(b"\n");

        document.extend(TextType::AlignLeft.command());
        Ok(())
    }

    // ==================== UTILIDADES ====================

    fn add_dashed_line(&self, document: &mut Vec<u8>) {
        let line = "-".repeat(
            self.print_job_context
                .printer_info
                .paper_size
                .chars_per_line() as usize,
        );
        document.extend(line.as_bytes());
        document.extend(b"\n");
    }

    fn add_double_line(&self, document: &mut Vec<u8>) {
        let line = "=".repeat(
            self.print_job_context
                .printer_info
                .paper_size
                .chars_per_line() as usize,
        );
        document.extend(line.as_bytes());
        document.extend(b"\n");
    }

    fn add_star_line(&self, document: &mut Vec<u8>) {
        let line = "*".repeat(
            self.print_job_context
                .printer_info
                .paper_size
                .chars_per_line() as usize,
        );
        document.extend(line.as_bytes());
        document.extend(b"\n");
    }

    fn create_receipt_line(&self, label: &str, value: &str) -> String {
        let total_width = self
            .print_job_context
            .printer_info
            .paper_size
            .chars_per_line() as usize;
        let available = total_width.saturating_sub(label.len() + value.len());
        let dots = ".".repeat(available.max(1));
        format!("{}{}{}", label, dots, value)
    }
}

/// Formats the current UTC time as "dd/mm/yyyy HH:MM:SS" using only std::time.
/// Works on all platforms supported by Rust (Linux, macOS, Windows, Android, iOS).
fn format_utc_datetime() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};

    let total_secs = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);

    let time_of_day = total_secs % 86400;
    let days = total_secs / 86400;
    let hour = (time_of_day / 3600) as u32;
    let min = ((time_of_day % 3600) / 60) as u32;
    let sec = (time_of_day % 60) as u32;

    let (year, month, day) = days_to_ymd(days);
    format!(
        "{:02}/{:02}/{} {:02}:{:02}:{:02}",
        day, month, year, hour, min, sec
    )
}

/// Converts days since Unix epoch (1970-01-01) to (year, month, day) via the
/// Proleptic Gregorian calendar algorithm by Howard Hinnant (civil_from_days).
fn days_to_ymd(days: u64) -> (u32, u32, u32) {
    let z = days + 719468;
    let era = z / 146097;
    let doe = z % 146097;
    let yoe = (doe - doe / 1460 + doe / 36524 - doe / 146096) / 365;
    let y = (yoe + era * 400) as u32;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = (doy - (153 * mp + 2) / 5 + 1) as u32;
    let m = if mp < 10 { mp + 3 } else { mp - 9 } as u32;
    let y = if m <= 2 { y + 1 } else { y };
    (y, m, d)
}
