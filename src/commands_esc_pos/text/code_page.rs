use serde::{Deserialize, Serialize};

/// Página de código (juego de caracteres) que la impresora usará.
/// El comando ESC t n (`0x1B 0x74 n`) se envía una vez al iniciar el documento.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum CodePage {
    /// Envía ESC t n con el número de página indicado.
    /// El usuario es responsable de elegir el número correcto para su impresora
    /// y de asegurarse que el texto ya esté codificado correctamente.
    /// Para texto UTF-8 con acentos, usa `AccentRemover` en su lugar.
    Page(u8),
    /// Elimina acentos y caracteres especiales reemplazándolos por su equivalente ASCII.
    /// Úsalo cuando la impresora no soporta ningún code page alternativo.
    /// Ej: á→a, é→e, ñ→n, ü→u, ç→c, ß→ss, ø→o, æ→ae, œ→oe
    AccentRemover,
}

impl Default for CodePage {
    fn default() -> Self {
        CodePage::Page(0)
    }
}

impl CodePage {
    /// Genera el comando ESC/POS `ESC t n` para seleccionar la página de código.
    /// Para `AccentRemover` envía n=0 (CP437 estándar) ya que el texto saldrá en ASCII.
    pub fn escpos_command(self) -> [u8; 3] {
        let n = match self {
            CodePage::Page(n) => n,
            CodePage::AccentRemover => 0,
        };
        [0x1B, 0x74, n]
    }

    /// Convierte un string UTF-8 a bytes según la página de código seleccionada.
    /// - `Page(n)`: el texto se envía directamente como bytes.
    /// - `AccentRemover`: convierte acentos y diacríticos a su equivalente ASCII.
    pub fn encode_str(self, text: &str) -> Vec<u8> {
        match self {
            CodePage::Page(_) => text.as_bytes().to_vec(),
            CodePage::AccentRemover => {
                let mut out = Vec::with_capacity(text.len());
                for c in text.chars() {
                    if (c as u32) < 128 {
                        out.push(c as u8);
                    } else {
                        out.extend_from_slice(strip_accents_str(c));
                    }
                }
                out
            }
        }
    }
}

// ─── Accent Remover ──────────────────────────────────────────────────────────
/// Retorna la representación ASCII del char como slice estático.
fn strip_accents_str(c: char) -> &'static [u8] {
    match c {
        // ── Minúsculas con diacrítico ────────────────────────────────────────
        'á'|'à'|'â'|'ä'|'ã'|'å'|'ā'|'ă'|'ą' => b"a",
        'é'|'è'|'ê'|'ë'|'ē'|'ě'|'ę' => b"e",
        'í'|'ì'|'î'|'ï'|'ī'|'ĭ' => b"i",
        'ó'|'ò'|'ô'|'ö'|'õ'|'ő'|'ō'|'ø' => b"o",
        'ú'|'ù'|'û'|'ü'|'ű'|'ū'|'ů' => b"u",
        'ý'|'ÿ' => b"y",
        'ñ'|'ń'|'ň' => b"n",
        'ç'|'ć'|'č' => b"c",
        'ð'|'ď' => b"d",
        'ł'|'ľ'|'ĺ' => b"l",
        'ř'|'ŗ' => b"r",
        'š'|'ś'|'ş' => b"s",
        'ť'|'ţ' => b"t",
        'ž'|'ź'|'ż' => b"z",
        'ğ' => b"g",
        'ı' => b"i",
        // multi-char
        'ß' => b"ss",
        'æ' => b"ae",
        'œ' => b"oe",
        'þ' => b"th",
        // ── Mayúsculas con diacrítico ────────────────────────────────────────
        'Á'|'À'|'Â'|'Ä'|'Ã'|'Å'|'Ā'|'Ă'|'Ą' => b"A",
        'É'|'È'|'Ê'|'Ë'|'Ē'|'Ě'|'Ę' => b"E",
        'Í'|'Ì'|'Î'|'Ï'|'Ī' => b"I",
        'Ó'|'Ò'|'Ô'|'Ö'|'Õ'|'Ő'|'Ō'|'Ø' => b"O",
        'Ú'|'Ù'|'Û'|'Ü'|'Ű'|'Ū'|'Ů' => b"U",
        'Ý' => b"Y",
        'Ñ'|'Ń'|'Ň' => b"N",
        'Ç'|'Ć'|'Č' => b"C",
        'Ð'|'Ď' => b"D",
        'Ł'|'Ľ'|'Ĺ' => b"L",
        'Ř'|'Ŗ' => b"R",
        'Š'|'Ś'|'Ş' => b"S",
        'Ť'|'Ţ' => b"T",
        'Ž'|'Ź'|'Ż' => b"Z",
        'Ğ' => b"G",
        'Æ' => b"AE",
        'Œ' => b"OE",
        'Þ' => b"TH",
        // ── Puntuación y símbolos especiales ────────────────────────────────
        '¿' => b"?",
        '¡' => b"!",
        '«' | '»' => b"\"",
        '\u{2018}' | '\u{2019}' => b"'",   // ' '
        '\u{201C}' | '\u{201D}' => b"\"",  // " "
        '–' | '—' => b"-",
        '…' => b"...",
        '•' => b"*",
        '·' => b".",
        '°' => b"o",
        '±' => b"+/-",
        '×' => b"x",
        '÷' => b"/",
        '½' => b"1/2",
        '¼' => b"1/4",
        '¾' => b"3/4",
        '€' => b"EUR",
        '£' => b"GBP",
        '¥' => b"JPY",
        '¢' => b"c",
        '©' => b"(C)",
        '®' => b"(R)",
        '™' => b"(TM)",
        '§' => b"S",
        '¶' => b"P",
        '†' => b"+",
        '‡' => b"++",
        '‰' => b"0/00",
        '√' => b"sqrt",
        '∞' => b"inf",
        '≈' => b"~",
        '≠' => b"!=",
        '≤' => b"<=",
        '≥' => b">=",
        '←' => b"<-",
        '→' => b"->",
        '↑' => b"^",
        '↓' => b"v",
        'µ' => b"u",
        'ª' => b"a",
        'º' => b"o",
        '¬' => b"!",
        _ => b"?",
    }
}
