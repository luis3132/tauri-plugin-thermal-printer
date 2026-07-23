//! Unit tests for the shared ESC/POS generation (platform-independent).
//!
//! Run all: `cargo test`
//!
//! Configurable physical-test document dump (see `configurable_test_document`):
//! ```text
//! # default sections:
//! cargo test configurable_test_document -- --nocapture
//! # every section:
//! TEST_SECTIONS=all cargo test configurable_test_document -- --nocapture
//! # only some sections:
//! TEST_SECTIONS=test_spacing,test_positioning,test_beep2 cargo test configurable_test_document -- --nocapture
//! ```

use crate::commands_esc_pos::text::code_page::CodePage;
use crate::models::print_job_request::PrintJobRequest;
use crate::models::print_sections::*;
use crate::process::process_print::ProcessPrint;
use crate::process::process_print_test::TestPrinter;
use crate::TestPrintRequest;
use serde_json::{json, Value};
use std::collections::HashSet;

// ─── Helpers ────────────────────────────────────────────────────────────────

fn job(sections: Vec<PrintSections>) -> PrintJobRequest {
    PrintJobRequest {
        printer: "test".to_string(),
        sections,
        options: CodePage::default(),
        paper_size: crate::PaperSize::DEFAULT,
    }
}

fn gen(sections: Vec<PrintSections>) -> Vec<u8> {
    ProcessPrint::new()
        .generate_document(&job(sections))
        .expect("document generation should succeed")
}

fn contains(haystack: &[u8], needle: &[u8]) -> bool {
    haystack.windows(needle.len()).any(|w| w == needle)
}

fn count(haystack: &[u8], needle: &[u8]) -> usize {
    haystack.windows(needle.len()).filter(|w| *w == needle).count()
}

// ─── New print sections (Tier 1 / Tier 2) ───────────────────────────────────

#[test]
fn line_spacing_with_value_emits_esc_3() {
    let out = gen(vec![PrintSections::LineSpacing(LineSpacing { value: Some(30) })]);
    assert!(contains(&out, &[0x1B, 0x33, 30]));
}

#[test]
fn line_spacing_none_resets_with_esc_2() {
    let out = gen(vec![PrintSections::LineSpacing(LineSpacing { value: None })]);
    assert!(contains(&out, &[0x1B, 0x32]));
}

#[test]
fn char_spacing_emits_esc_sp() {
    let out = gen(vec![PrintSections::CharSpacing(CharSpacing { value: 5 })]);
    assert!(contains(&out, &[0x1B, 0x20, 5]));
}

#[test]
fn position_emits_esc_dollar_little_endian() {
    // 300 = 0x012C -> nL = 0x2C, nH = 0x01
    let out = gen(vec![PrintSections::Position(Position { value: 300 })]);
    assert!(contains(&out, &[0x1B, 0x24, 0x2C, 0x01]));
}

#[test]
fn tab_stops_emit_esc_d_terminated_by_nul() {
    let out = gen(vec![PrintSections::TabStops(TabStops {
        positions: vec![8, 16, 24],
    })]);
    assert!(contains(&out, &[0x1B, 0x44, 8, 16, 24, 0x00]));
}

#[test]
fn tab_stops_empty_is_rejected() {
    let result = ProcessPrint::new().generate_document(&job(vec![PrintSections::TabStops(
        TabStops { positions: vec![] },
    )]));
    assert!(result.is_err());
}

#[test]
fn left_margin_emits_gs_l() {
    let out = gen(vec![PrintSections::LeftMargin(LeftMargin { value: 20 })]);
    assert!(contains(&out, &[0x1D, 0x4C, 20, 0x00]));
}

#[test]
fn print_area_width_emits_gs_w_little_endian() {
    // 512 = 0x0200 -> nL = 0x00, nH = 0x02
    let out = gen(vec![PrintSections::PrintAreaWidth(PrintAreaWidth { value: 512 })]);
    assert!(contains(&out, &[0x1D, 0x57, 0x00, 0x02]));
}

#[test]
fn beep_uses_epson_esc_paren_a() {
    let out = gen(vec![PrintSections::Beep(Beep {
        times: 2,
        duration: 3,
    })]);
    assert!(contains(&out, &[0x1B, 0x28, 0x41]));
}

#[test]
fn beep2_uses_generic_esc_b() {
    let out = gen(vec![PrintSections::Beep2(Beep {
        times: 2,
        duration: 3,
    })]);
    assert!(contains(&out, &[0x1B, 0x42, 2, 3]));
}

#[test]
fn double_strike_style_emits_esc_g() {
    let styles = GlobalStyles {
        double_strike: Some(true),
        ..Default::default()
    };
    let out = gen(vec![PrintSections::GlobalStyles(styles)]);
    assert!(contains(&out, &[0x1B, 0x47, 0x01]));
}

// ─── Reset ──────────────────────────────────────────────────────────────────

#[test]
fn reset_reinitializes_and_reapplies_code_page() {
    let styles = GlobalStyles {
        reset: Some(true),
        // These must be ignored because reset takes priority.
        bold: Some(true),
        double_strike: Some(true),
        ..Default::default()
    };
    let out = gen(vec![PrintSections::GlobalStyles(styles)]);

    // One ESC @ at document start + one from the reset section.
    assert_eq!(count(&out, &[0x1B, 0x40]), 2, "expected two ESC @ (init + reset)");
    // Code page (ESC t) re-applied after the reset.
    assert_eq!(count(&out, &[0x1B, 0x74]), 2, "expected code page re-applied after reset");
    // Ignored style fields must NOT have emitted their commands.
    assert!(!contains(&out, &[0x1B, 0x45, 0x01]), "bold must be ignored on reset");
    assert!(!contains(&out, &[0x1B, 0x47, 0x01]), "double_strike must be ignored on reset");
}

// ─── Document skeleton ───────────────────────────────────────────────────────

fn text_section(s: &str) -> PrintSections {
    PrintSections::Text(Text {
        text: s.to_string(),
        styles: None,
    })
}

fn styled(styles: GlobalStyles) -> PrintSections {
    PrintSections::GlobalStyles(styles)
}

#[test]
fn document_starts_with_init_and_code_page() {
    let out = gen(vec![text_section("x")]);
    // ESC @ (init) immediately followed by ESC t 0 (code page).
    assert!(out.starts_with(&[0x1B, 0x40, 0x1B, 0x74, 0x00]));
}

#[test]
fn empty_document_is_rejected() {
    assert!(ProcessPrint::new().generate_document(&job(vec![])).is_err());
}

// ─── Text sections ───────────────────────────────────────────────────────────

#[test]
fn title_forces_double_size_and_center() {
    let out = gen(vec![PrintSections::Title(Title {
        text: "Hi".to_string(),
        styles: None,
    })]);
    assert!(contains(&out, &[0x1B, 0x21, 0x30]), "double size (ESC ! 0x30)");
    assert!(contains(&out, &[0x1B, 0x61, 0x01]), "center (ESC a 1)");
    assert!(contains(&out, b"Hi"));
}

#[test]
fn subtitle_forces_double_height_and_bold() {
    let out = gen(vec![PrintSections::Subtitle(Subtitle {
        text: "Sub".to_string(),
        styles: None,
    })]);
    assert!(contains(&out, &[0x1B, 0x21, 0x10]), "double height (ESC ! 0x10)");
    assert!(contains(&out, &[0x1B, 0x45, 0x01]), "bold (ESC E 1)");
    assert!(contains(&out, b"Sub"));
}

#[test]
fn text_is_encoded_verbatim_for_ascii() {
    let out = gen(vec![text_section("HELLO")]);
    assert!(contains(&out, b"HELLO"));
}

#[test]
fn line_repeats_character_across_width() {
    // Mm80 = 48 chars per line.
    let out = gen(vec![PrintSections::Line(Line {
        character: "=".to_string(),
    })]);
    assert!(contains(&out, b"========"));
}

// ─── Global styles (each attribute → its command) ────────────────────────────

#[test]
fn global_styles_emit_expected_commands() {
    let cases: &[(GlobalStyles, &[u8], &str)] = &[
        (GlobalStyles { bold: Some(true), ..Default::default() }, &[0x1B, 0x45, 0x01], "bold"),
        (GlobalStyles { underline: Some(true), ..Default::default() }, &[0x1B, 0x2D, 0x01], "underline"),
        (GlobalStyles { italic: Some(true), ..Default::default() }, &[0x1B, 0x34], "italic"),
        (GlobalStyles { invert: Some(true), ..Default::default() }, &[0x1D, 0x42, 0x01], "invert"),
        (GlobalStyles { rotate: Some(true), ..Default::default() }, &[0x1B, 0x56, 0x01], "rotate"),
        (GlobalStyles { upside_down: Some(true), ..Default::default() }, &[0x1B, 0x7B, 0x01], "upside_down"),
        (GlobalStyles { font: Some("B".into()), ..Default::default() }, &[0x1B, 0x4D, 0x01], "font B"),
        (GlobalStyles { font: Some("C".into()), ..Default::default() }, &[0x1B, 0x4D, 0x02], "font C"),
        (GlobalStyles { align: Some("center".into()), ..Default::default() }, &[0x1B, 0x61, 0x01], "align center"),
        (GlobalStyles { align: Some("right".into()), ..Default::default() }, &[0x1B, 0x61, 0x02], "align right"),
        (GlobalStyles { size: Some("width".into()), ..Default::default() }, &[0x1B, 0x21, 0x20], "double width"),
        (GlobalStyles { size: Some("height".into()), ..Default::default() }, &[0x1B, 0x21, 0x10], "double height"),
        (GlobalStyles { size: Some("double".into()), ..Default::default() }, &[0x1B, 0x21, 0x30], "double size"),
    ];
    for (style, expected, label) in cases {
        let out = gen(vec![styled(style.clone())]);
        assert!(contains(&out, expected), "{} should emit {:02X?}", label, expected);
    }
}

// ─── Control sections ────────────────────────────────────────────────────────

#[test]
fn feed_variants() {
    let lines = gen(vec![PrintSections::Feed(Feed { feed_type: "lines".into(), value: 3 })]);
    assert!(contains(&lines, &[0x1B, 0x64, 3]), "ESC d n");

    let dots = gen(vec![PrintSections::Feed(Feed { feed_type: "dots".into(), value: 50 })]);
    assert!(contains(&dots, &[0x1B, 0x4A, 50]), "ESC J n");

    let lf = gen(vec![PrintSections::Feed(Feed { feed_type: "line_feed".into(), value: 2 })]);
    assert!(contains(&lf, &[0x0A, 0x0A]), "raw LFs");
}

#[test]
fn cut_full_and_partial() {
    let full = gen(vec![PrintSections::Cut(Cut { mode: "full".into(), feed: 3 })]);
    assert!(contains(&full, &[0x1D, 0x56, 66, 3]), "GS V 66 (full)");

    let partial = gen(vec![PrintSections::Cut(Cut { mode: "partial".into(), feed: 0 })]);
    assert!(contains(&partial, &[0x1D, 0x56, 65, 0]), "GS V 65 (partial)");
}

#[test]
fn drawer_pins() {
    // pulse_time 100 -> 50 units (2ms steps).
    let pin2 = gen(vec![PrintSections::Drawer(Drawer { pin: 2, pulse_time: 100 })]);
    assert!(contains(&pin2, &[0x1B, 0x70, 0x00, 50, 50]), "ESC p 0");

    let pin5 = gen(vec![PrintSections::Drawer(Drawer { pin: 5, pulse_time: 100 })]);
    assert!(contains(&pin5, &[0x1B, 0x70, 0x01, 50, 50]), "ESC p 1");
}

// ─── Codes ───────────────────────────────────────────────────────────────────

#[test]
fn qr_uses_gs_paren_k_with_cn_49() {
    let out = gen(vec![PrintSections::Qr(Qr {
        data: "https://example.com".into(),
        size: 6,
        error_correction: "M".into(),
        model: 2,
        align: None,
    })]);
    assert!(contains(&out, &[0x1D, 0x28, 0x6B, 0x04, 0x00, 0x31]), "QR model fn (cn=49)");
}

#[test]
fn barcode_uses_gs_k() {
    let out = gen(vec![PrintSections::Barcode(Barcode {
        data: "123456789012".into(),
        barcode_type: "CODE128".into(),
        width: 2,
        height: 60,
        text_position: "below".into(),
        align: None,
    })]);
    assert!(contains(&out, &[0x1D, 0x6B]), "GS k (barcode)");
    assert!(contains(&out, &[0x1D, 0x68, 60]), "GS h (height)");
    assert!(contains(&out, &[0x1D, 0x77, 2]), "GS w (width)");
}

#[test]
fn gs1_128_uses_gs_k_m_74() {
    let out = gen(vec![PrintSections::Barcode(Barcode {
        data: "12345678".into(),
        barcode_type: "GS1-128".into(),
        width: 2,
        height: 60,
        text_position: "below".into(),
        align: None,
    })]);
    // GS k m n data — m=74 (GS1-128)
    assert!(contains(&out, &[0x1D, 0x6B, 74, 8]), "GS k m=74 (GS1-128)");
}

#[test]
fn gs1_databar_variants_use_gs_k_m_75_to_78() {
    for (ty, m) in [
        ("GS1-DATABAR-OMNI", 75u8),
        ("GS1-DATABAR-TRUNCATED", 76),
        ("GS1-DATABAR-LIMITED", 77),
    ] {
        let out = gen(vec![PrintSections::Barcode(Barcode {
            data: "1234567890123".into(),
            barcode_type: ty.into(),
            width: 2,
            height: 60,
            text_position: "below".into(),
            align: None,
        })]);
        assert!(contains(&out, &[0x1D, 0x6B, m, 13]), "GS k m={m} ({ty})");
    }

    // Expanded acepta datos GS1 con AIs (no numeric-only)
    let out = gen(vec![PrintSections::Barcode(Barcode {
        data: "(01)12345678901231".into(),
        barcode_type: "GS1-DATABAR-EXPANDED".into(),
        width: 2,
        height: 60,
        text_position: "below".into(),
        align: None,
    })]);
    assert!(contains(&out, &[0x1D, 0x6B, 78]), "GS k m=78 (GS1 DataBar Expanded)");
}

#[test]
fn data_matrix_uses_gs_paren_k_with_cn_54() {
    let out = gen(vec![PrintSections::DataMatrix(DataMatrixModel {
        data: "DM data".into(),
        size: 5,
    })]);
    // cn = 54 (0x36) es el valor estándar de DataMatrix (cn=50 es MaxiCode)
    assert!(contains(&out, &[0x1D, 0x28, 0x6B, 0x03, 0x00, 0x36]), "DataMatrix (cn=54)");
}

#[test]
fn pdf417_uses_gs_paren_k_with_cn_48() {
    let out = gen(vec![PrintSections::Pdf417(Pdf417 {
        data: "PDF data".into(),
        columns: 2,
        rows: 5,
        width: 3,
        height: 5,
        error_correction: 2,
    })]);
    assert!(contains(&out, &[0x1D, 0x28, 0x6B, 0x03, 0x00, 0x30]), "PDF417 (cn=48)");
}

#[test]
fn aztec_uses_gs_paren_k_with_cn_53() {
    let out = gen(vec![PrintSections::Aztec(Aztec {
        data: "Aztec data".into(),
        mode: 0,
        layers: 0,
        size: 3,
        error_correction: 23,
        align: None,
    })]);
    // cn = 53 (0x35)
    assert!(contains(&out, &[0x1D, 0x28, 0x6B, 0x03, 0x00, 0x35]), "Aztec (cn=53)");
}

#[test]
fn gs1_databar_2d_uses_gs_paren_k_with_cn_51() {
    let out = gen(vec![PrintSections::Gs1Databar2d(Gs1Databar2d {
        data: "1234567890123".into(),
        databar_type: "STACKED-OMNI".into(),
        width: 2,
        align: None,
    })]);
    // cn = 51 (0x33); store fn 80 con m=73 (Stacked Omnidirectional)
    assert!(contains(&out, &[0x1D, 0x28, 0x6B, 0x03, 0x00, 0x33]), "GS1 DataBar 2D (cn=51)");
    assert!(contains(&out, &[0x33, 0x50, 73]), "store m=73 (Stacked Omni)");
}

#[test]
fn maxicode_uses_gs_paren_k_with_cn_50() {
    let out = gen(vec![PrintSections::MaxiCode(MaxiCode {
        data: "MaxiCode data".into(),
        mode: 4,
        align: None,
    })]);
    // cn = 50 (0x32); modo 4 -> n=0x34
    assert!(contains(&out, &[0x1D, 0x28, 0x6B, 0x03, 0x00, 0x32]), "MaxiCode (cn=50)");
    assert!(contains(&out, &[0x32, 0x43, 0x34]), "modo 4 (n=0x34)");
}

#[test]
fn composite_uses_gs_paren_k_with_cn_52() {
    let out = gen(vec![PrintSections::Composite(Composite {
        data: "Composite data".into(),
        symbol_type: 48,
        width: 2,
        align: None,
    })]);
    // cn = 52 (0x34)
    assert!(contains(&out, &[0x1D, 0x28, 0x6B, 0x03, 0x00, 0x34]), "Composite (cn=52)");
}

// ─── Image & logo ────────────────────────────────────────────────────────────

/// 1x1 white PNG.
const TINY_PNG_BASE64: &str =
    "iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAIAAACQd1PeAAAADElEQVR4nGP4//8/AAX+Av4N70a4AAAAAElFTkSuQmCC";

#[test]
fn image_emits_raster_command() {
    let out = gen(vec![PrintSections::Image(Image {
        data: TINY_PNG_BASE64.into(),
        max_width: 0,
        align: "center".into(),
        dithering: false,
        size: "normal".into(),
    })]);
    assert!(contains(&out, &[0x1D, 0x76, 0x30]), "GS v 0 (raster)");
}

#[test]
fn empty_image_is_rejected() {
    let result = ProcessPrint::new().generate_document(&job(vec![PrintSections::Image(Image {
        data: "".into(),
        max_width: 0,
        align: "left".into(),
        dithering: false,
        size: "normal".into(),
    })]));
    assert!(result.is_err());
}

#[test]
fn logo_emits_fs_p() {
    let out = gen(vec![PrintSections::Logo(Logo {
        key_code: Some(1),
        mode: Some("normal".into()),
        set_logo: None,
    })]);
    assert!(contains(&out, &[0x1C, 0x70, 0x01]), "FS p n (print NV logo)");
}

#[test]
fn logo_defaults_key_code_when_omitted() {
    // Sin key_code ni mode debe imprimir el logo NV con la clave por defecto (1).
    let out = gen(vec![PrintSections::Logo(Logo {
        key_code: None,
        mode: None,
        set_logo: None,
    })]);
    assert!(contains(&out, &[0x1C, 0x70, 0x01]), "FS p n con key_code por defecto");
}

#[test]
fn set_logo_emits_fs_q_store() {
    // `set_logo` tiene prioridad: guarda en memoria NV (FS q) e ignora key_code/mode.
    let out = gen(vec![PrintSections::Logo(Logo {
        key_code: Some(9),
        mode: Some("quadruple".into()),
        set_logo: Some(Image {
            data: TINY_PNG_BASE64.into(),
            max_width: 0,
            align: "center".into(),
            dithering: false,
            size: "normal".into(),
        }),
    })]);
    // FS q n=1 xL xH yL yH (para 1x1 => 1 byte ancho, 1 byte alto)
    assert!(
        contains(&out, &[0x1C, 0x71, 0x01, 0x01, 0x00, 0x01, 0x00]),
        "FS q n xL xH yL yH (store NV logo)"
    );
    // No debe emitir el comando de impresión de logo (FS p).
    assert!(!contains(&out, &[0x1C, 0x70]), "set_logo no debe imprimir (sin FS p)");
}

#[test]
fn set_logo_rejects_empty_image() {
    let result = ProcessPrint::new().generate_document(&job(vec![PrintSections::Logo(Logo {
        key_code: None,
        mode: None,
        set_logo: Some(Image {
            data: "".into(),
            max_width: 0,
            align: "left".into(),
            dithering: false,
            size: "normal".into(),
        }),
    })]));
    assert!(result.is_err());
}

// ─── Table ───────────────────────────────────────────────────────────────────

#[test]
fn table_renders_cell_text() {
    let cell = |s: &str| Text { text: s.to_string(), styles: None };
    let out = gen(vec![PrintSections::Table(Table {
        columns: 2,
        column_widths: None,
        header: None,
        body: vec![vec![cell("ZED"), cell("OK")]],
        truncate: true,
        word_wrap: None,
    })]);
    assert!(contains(&out, b"ZED"));
    assert!(contains(&out, b"OK"));
}

fn build_wrap_table(truncate: bool, word_wrap: Option<bool>) -> Table {
    let cell = |s: &str| Text { text: s.to_string(), styles: None };
    Table {
        columns: 2,
        column_widths: Some(vec![12, 36]),
        header: None,
        body: vec![vec![
            cell("Cafe Americano Extra"),
            cell("Descripcion larga que no cabe en una linea"),
        ]],
        truncate,
        word_wrap,
    }
}

/// Con `word_wrap = true` y sin truncar: el texto que no cabe continúa DEBAJO, dentro
/// de su propia columna (no se desborda a la derecha) y se envuelve por PALABRA (sin
/// partir palabras que caben en el ancho de la columna).
#[test]
fn table_word_wrap_keeps_words_and_wraps_below() {
    let out = gen(vec![PrintSections::Table(build_wrap_table(false, Some(true)))]);
    let text = String::from_utf8_lossy(&out);
    let body: Vec<&str> = text
        .lines()
        .filter(|l| {
            ["Cafe", "Americano", "Extra", "Descripcion", "linea"]
                .iter()
                .any(|w| l.contains(w))
        })
        .collect();

    // La fila ocupa varias líneas: la continuación va debajo, no a la derecha.
    assert!(body.len() >= 2, "la fila debe envolverse en varias lineas: {:?}", body);

    // Word-wrap: las palabras que caben en la columna NO se parten.
    assert!(text.contains("Americano"), "'Americano' no debe partirse");
    assert!(text.contains("Extra"), "'Extra' se conserva y va debajo en col1");
    assert!(text.contains("Descripcion"), "'Descripcion' no debe partirse");
    assert!(text.contains("linea"), "col2 continúa debajo, palabra intacta");

    // No hay desbordamiento a la derecha: en la primera línea, la columna 1 ("Cafe")
    // se rellena hasta 12 y la columna 2 ("Descripcion") arranca justo después.
    let first = text.lines().find(|l| l.contains("Cafe")).unwrap();
    assert!(
        first.contains("Cafe") && first.contains("Descripcion"),
        "col1 y col2 en la misma primera línea, alineadas: {:?}",
        first
    );
    // Entre "Cafe" y "Descripcion" solo debe haber espacios de relleno (no texto).
    let mid = &first[first.find("Cafe").unwrap() + 4..first.find("Descripcion").unwrap()];
    assert!(
        mid.chars().all(|c| c == ' '),
        "col1 se rellena solo con espacios (sin desbordar a la derecha): {:?}",
        first
    );
}

/// Por defecto (`word_wrap` ausente): el ajuste es por CARÁCTER, así que una palabra
/// que no cabe se parte a mitad (comportamiento clásico, más compacto).
#[test]
fn table_default_wrap_is_by_character() {
    let out = gen(vec![PrintSections::Table(build_wrap_table(false, None))]);
    let text = String::from_utf8_lossy(&out);
    // "Cafe Americano Extra" en ancho 12 se llena carácter a carácter:
    // "Cafe America" (12) y luego "no Extra". La palabra "Americano" queda partida.
    assert!(text.contains("Cafe America"), "col1 llena a 12 chars por carácter: {:?}", text);
    assert!(
        !text.contains("Americano"),
        "en modo por carácter 'Americano' se parte (no aparece intacto)"
    );
}

/// Con truncar: el texto que no cabe se corta y NO continúa debajo (independiente de
/// `word_wrap`).
#[test]
fn table_truncate_cuts_and_does_not_wrap_below() {
    let out = gen(vec![PrintSections::Table(build_wrap_table(true, Some(true)))]);
    let text = String::from_utf8_lossy(&out);

    // Solo hay una línea para la fila: nada continúa debajo.
    let row_lines = text.lines().filter(|l| l.contains("Cafe")).count();
    assert_eq!(row_lines, 1, "truncate: la fila debe ocupar una sola linea");
    // El contenido sobrante quedó cortado (no aparece el final de cada celda).
    assert!(!text.contains("Extra"), "col1 truncada: 'Extra' no debe aparecer");
    assert!(!text.contains("una linea"), "col2 truncada: el final no debe aparecer");
}

// ─── Physical test document (TestPrinter) ────────────────────────────────────

/// Every boolean section flag that the configurable dump can toggle.
/// (`include_custom_text` / `include_image` are omitted — they need extra payload.)
const SECTION_FLAGS: &[&str] = &[
    "include_text",
    "include_text_styles",
    "include_alignment",
    "include_columns",
    "include_separators",
    "include_barcode",
    "include_barcode_types",
    "include_qr",
    "include_beep",
    "test_cash_drawer",
    "cut_paper",
    "test_feed",
    "test_all_fonts",
    "test_invert",
    "test_rotate",
    "test_double_strike",
    "test_spacing",
    "test_positioning",
    "test_beep2",
];

fn printer_info() -> Value {
    json!({
        "printer": "test",
        "sections": [],
        "options": { "code_page": 0 },
        "paper_size": "Mm80"
    })
}

/// Builds a request with exactly the listed flags enabled (everything else false).
fn request_with(enabled: &HashSet<String>) -> TestPrintRequest {
    let mut obj = serde_json::Map::new();
    obj.insert("printer_info".to_string(), printer_info());
    for flag in SECTION_FLAGS {
        obj.insert((*flag).to_string(), json!(enabled.contains(*flag)));
    }
    serde_json::from_value(Value::Object(obj)).expect("valid TestPrintRequest")
}

/// Builds a request relying on the serde defaults (the standard test print).
fn request_default() -> TestPrintRequest {
    serde_json::from_value(json!({ "printer_info": printer_info() }))
        .expect("valid TestPrintRequest")
}

fn gen_test(enabled: &HashSet<String>) -> Vec<u8> {
    TestPrinter::new()
        .generate_test_document(&request_with(enabled))
        .expect("test document generation should succeed")
}

fn flag_set(flags: &[&str]) -> HashSet<String> {
    flags.iter().map(|s| s.to_string()).collect()
}

#[test]
fn test_document_spacing_section_uses_new_commands() {
    let out = gen_test(&flag_set(&["test_spacing"]));
    assert!(contains(&out, &[0x1B, 0x33, 20]), "ESC 3 (line spacing)");
    assert!(contains(&out, &[0x1B, 0x32]), "ESC 2 (line spacing reset)");
    assert!(contains(&out, &[0x1B, 0x20, 3]), "ESC SP (char spacing)");
}

#[test]
fn test_document_positioning_section_uses_new_commands() {
    let out = gen_test(&flag_set(&["test_positioning"]));
    assert!(contains(&out, &[0x1B, 0x44, 10, 24, 0x00]), "ESC D (tab stops)");
    assert!(contains(&out, &[0x1B, 0x24, 0xC8, 0x00]), "ESC $ 200 dots");
    assert!(contains(&out, &[0x1D, 0x4C, 60, 0x00]), "GS L (left margin)");
}

#[test]
fn test_document_beep2_uses_generic_buzzer() {
    let out = gen_test(&flag_set(&["test_beep2"]));
    assert!(contains(&out, &[0x1B, 0x42, 2, 3]), "ESC B (generic buzzer)");
}

#[test]
fn test_document_double_strike_line_emits_esc_g() {
    let out = gen_test(&flag_set(&["include_text_styles", "test_double_strike"]));
    assert!(contains(&out, &[0x1B, 0x47, 0x01]), "ESC G (double-strike)");
}

#[test]
fn test_document_logo_stores_and_prints() {
    // test_logo con una imagen: guarda en memoria NV (FS q) y luego imprime (FS p).
    let request: TestPrintRequest = serde_json::from_value(json!({
        "printer_info": printer_info(),
        "test_logo": true,
        "image_base64": TINY_PNG_BASE64,
    }))
    .expect("valid TestPrintRequest");
    let out = TestPrinter::new()
        .generate_test_document(&request)
        .expect("test document generation should succeed");

    // FS q n=1 xL xH yL yH (para 1x1 => 1 byte de ancho y alto)
    assert!(
        contains(&out, &[0x1C, 0x71, 0x01, 0x01, 0x00, 0x01, 0x00]),
        "FS q (store NV logo)"
    );
    // FS p n=1 (print NV logo)
    assert!(contains(&out, &[0x1C, 0x70, 0x01]), "FS p (print NV logo)");
}

#[test]
fn test_document_logo_skipped_without_image() {
    // Sin image_base64 la sección de logo se omite silenciosamente.
    let request: TestPrintRequest = serde_json::from_value(json!({
        "printer_info": printer_info(),
        "test_logo": true,
    }))
    .expect("valid TestPrintRequest");
    let out = TestPrinter::new()
        .generate_test_document(&request)
        .expect("test document generation should succeed");
    assert!(!contains(&out, &[0x1C, 0x71]), "sin imagen no debe emitir FS q");
}

// ─── Configurable dump (input via TEST_SECTIONS) ─────────────────────────────

/// Generates the physical-test document with sections chosen at runtime and dumps
/// the ESC/POS bytes as hex. Controlled by the `TEST_SECTIONS` env var:
///   - unset / `default` → the standard test print (serde defaults)
///   - `all`             → every section enabled
///   - `a,b,c`           → only those flags enabled (see `SECTION_FLAGS`)
///
/// Run with `-- --nocapture` to see the output.
#[test]
fn configurable_test_document() {
    let mode = std::env::var("TEST_SECTIONS").unwrap_or_default();
    let mode = mode.trim();

    let (label, document) = match mode {
        "" | "default" => (
            "default".to_string(),
            TestPrinter::new()
                .generate_test_document(&request_default())
                .expect("test document generation should succeed"),
        ),
        "all" => ("all".to_string(), gen_test(&flag_set(SECTION_FLAGS))),
        list => {
            let requested: HashSet<String> = list
                .split(',')
                .map(|s| s.trim().to_string())
                .filter(|s| !s.is_empty())
                .collect();
            // Fail loudly on typos so the input is not silently ignored.
            for flag in &requested {
                assert!(
                    SECTION_FLAGS.contains(&flag.as_str()),
                    "unknown TEST_SECTIONS flag: '{}'. Valid flags: {:?}",
                    flag,
                    SECTION_FLAGS
                );
            }
            (format!("[{}]", list), gen_test(&requested))
        }
    };

    let hex = document
        .iter()
        .map(|b| format!("{:02X}", b))
        .collect::<Vec<_>>()
        .join(" ");

    eprintln!("TEST_SECTIONS={} -> {} bytes", label, document.len());
    eprintln!("{}", hex);

    assert!(!document.is_empty());
}
