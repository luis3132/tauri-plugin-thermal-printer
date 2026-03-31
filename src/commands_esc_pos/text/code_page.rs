use serde::{Deserialize, Serialize};

/// Página de código (juego de caracteres) que la impresora usará.
/// Seleccionada por idioma/región para mayor claridad.
/// El comando ESC t n (`0x1B 0x74 n`) se envía una vez al iniciar el documento.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum CodePage {
    /// CP437 — ASCII estándar, sin caracteres especiales (por defecto)
    Default,
    /// CP850 — Español, Francés, Italiano, Alemán, Portugués occidental
    Spanish,
    /// CP850 — Alias de Spanish
    French,
    /// CP860 — Portugués (con ã, õ)
    Portuguese,
    /// CP863 — Francés canadiense
    CanadianFrench,
    /// CP865 — Nórdico (sueco, noruego, danés, finlandés — å, ø, æ)
    Nordic,
    /// CP1252 — Windows Latin-1, cobertura amplia Western European (incluye €)
    WindowsLatin,
    /// CP866 — Ruso / Cirílico
    Russian,
    /// CP852 — Europa del Este (polaco, checo, eslovaco, húngaro)
    EasternEurope,
    /// Elimina acentos y caracteres especiales reemplazándolos por su equivalente ASCII.
    /// Úsalo cuando la impresora no soporta ningún code page alternativo.
    /// Ej: á→a, é→e, ñ→n, ü→u, ç→c, ß→ss, ø→o, æ→ae, œ→oe
    AccentRemover,
}

impl Default for CodePage {
    fn default() -> Self {
        CodePage::Default
    }
}

impl CodePage {
    /// Retorna el byte `n` para el comando ESC t n
    fn escpos_n(self) -> u8 {
        match self {
            CodePage::Default => 0,
            CodePage::Spanish | CodePage::French => 2,
            CodePage::Portuguese => 3,
            CodePage::CanadianFrench => 4,
            CodePage::Nordic => 5,
            CodePage::WindowsLatin => 16,
            CodePage::Russian => 17,
            CodePage::EasternEurope => 18,
            CodePage::AccentRemover => 0,
        }
    }

    /// Genera el comando ESC/POS `ESC t n` para seleccionar la página de código
    pub fn escpos_command(self) -> [u8; 3] {
        [0x1B, 0x74, self.escpos_n()]
    }

    /// Convierte un carácter UTF-8 al byte correspondiente en esta página de código.
    /// Los caracteres ASCII (< 128) pasan directamente.
    /// Caracteres no soportados se reemplazan por `?`.
    pub fn encode_char(self, c: char) -> u8 {
        let code = c as u32;
        if code < 128 {
            return code as u8;
        }
        match self {
            CodePage::Default => b'?',
            CodePage::Spanish | CodePage::French => encode_cp850(c),
            CodePage::Portuguese => encode_cp860(c),
            CodePage::CanadianFrench => encode_cp863(c),
            CodePage::Nordic => encode_cp865(c),
            CodePage::WindowsLatin => encode_cp1252(c),
            CodePage::Russian => encode_cp866(c),
            CodePage::EasternEurope => encode_cp852(c),
            CodePage::AccentRemover => strip_accents_str(c)[0],
        }
    }

    /// Convierte un string UTF-8 a bytes en la página de código seleccionada
    pub fn encode_str(self, text: &str) -> Vec<u8> {
        if self == CodePage::AccentRemover {
            // AccentRemover puede expandir un char a múltiples bytes (ß→ss, æ→ae, ...)
            let mut out = Vec::with_capacity(text.len());
            for c in text.chars() {
                if (c as u32) < 128 {
                    out.push(c as u8);
                } else {
                    out.extend_from_slice(strip_accents_str(c));
                }
            }
            out
        } else {
            text.chars().map(|c| self.encode_char(c)).collect()
        }
    }
}

// ─── CP850: Western European ─────────────────────────────────────────────────
fn encode_cp850(c: char) -> u8 {
    match c {
        'Ç' => 0x80, 'ü' => 0x81, 'é' => 0x82, 'â' => 0x83,
        'ä' => 0x84, 'à' => 0x85, 'å' => 0x86, 'ç' => 0x87,
        'ê' => 0x88, 'ë' => 0x89, 'è' => 0x8A, 'ï' => 0x8B,
        'î' => 0x8C, 'ì' => 0x8D, 'Ä' => 0x8E, 'Å' => 0x8F,
        'É' => 0x90, 'æ' => 0x91, 'Æ' => 0x92, 'ô' => 0x93,
        'ö' => 0x94, 'ò' => 0x95, 'û' => 0x96, 'ù' => 0x97,
        'ÿ' => 0x98, 'Ö' => 0x99, 'Ü' => 0x9A, 'ø' => 0x9B,
        '£' => 0x9C, 'Ø' => 0x9D, '×' => 0x9E, 'ƒ' => 0x9F,
        'á' => 0xA0, 'í' => 0xA1, 'ó' => 0xA2, 'ú' => 0xA3,
        'ñ' => 0xA4, 'Ñ' => 0xA5, 'ª' => 0xA6, 'º' => 0xA7,
        '¿' => 0xA8, '®' => 0xA9, '¬' => 0xAA, '½' => 0xAB,
        '¼' => 0xAC, '¡' => 0xAD, '«' => 0xAE, '»' => 0xAF,
        'Á' => 0xB5, 'Â' => 0xB6, 'À' => 0xB7, '©' => 0xB8,
        'È' => 0xD4, 'Ê' => 0xD2, 'Ë' => 0xD3, 'Í' => 0xD6,
        'Î' => 0xD7, 'Ï' => 0xD8, 'Ì' => 0xDE, 'Ó' => 0xE0,
        'ß' => 0xE1, 'Ô' => 0xE2, 'Ò' => 0xE3, 'õ' => 0xE4,
        'Õ' => 0xE5, 'µ' => 0xE6, 'þ' => 0xE7, 'Þ' => 0xE8,
        'Ú' => 0xE9, 'Û' => 0xEA, 'Ù' => 0xEB, 'ý' => 0xEC,
        'Ý' => 0xED, '°' => 0xF8, '·' => 0xFA, '±' => 0xF1,
        'ã' => 0xC6, 'Ã' => 0xC7,
        _ => b'?',
    }
}

// ─── CP860: Portuguese ───────────────────────────────────────────────────────
fn encode_cp860(c: char) -> u8 {
    match c {
        'Ç' => 0x80, 'ü' => 0x81, 'é' => 0x82, 'â' => 0x83,
        'ã' => 0x84, 'à' => 0x85, 'Á' => 0x86, 'ç' => 0x87,
        'ê' => 0x88, 'Ê' => 0x89, 'è' => 0x8A, 'Í' => 0x8B,
        'Ô' => 0x8C, 'ì' => 0x8D, 'Ã' => 0x8E, 'Â' => 0x8F,
        'É' => 0x90, 'À' => 0x91, 'È' => 0x92, 'ô' => 0x93,
        'õ' => 0x94, 'ò' => 0x95, 'Ú' => 0x96, 'ù' => 0x97,
        'Ì' => 0x98, 'Õ' => 0x99, 'Ü' => 0x9A, '¢' => 0x9B,
        '£' => 0x9C, 'Ù' => 0x9D, '₧' => 0x9E, 'Ó' => 0x9F,
        'á' => 0xA0, 'í' => 0xA1, 'ó' => 0xA2, 'ú' => 0xA3,
        'ñ' => 0xA4, 'Ñ' => 0xA5, 'ª' => 0xA6, 'º' => 0xA7,
        '¿' => 0xA8, 'Ò' => 0xA9, '¬' => 0xAA, '½' => 0xAB,
        '¼' => 0xAC, '¡' => 0xAD, '«' => 0xAE, '»' => 0xAF,
        _ => b'?',
    }
}

// ─── CP863: Canadian French ──────────────────────────────────────────────────
fn encode_cp863(c: char) -> u8 {
    match c {
        'Ç' => 0x80, 'ü' => 0x81, 'é' => 0x82, 'â' => 0x83,
        'Â' => 0x84, 'à' => 0x85, '¶' => 0x86, 'ç' => 0x87,
        'ê' => 0x88, 'ë' => 0x89, 'è' => 0x8A, 'ï' => 0x8B,
        'î' => 0x8C, '‗' => 0x8D, 'À' => 0x8E, '§' => 0x8F,
        'É' => 0x90, 'È' => 0x91, 'Ê' => 0x92, 'ô' => 0x93,
        'Ë' => 0x94, 'Ï' => 0x95, 'û' => 0x96, 'ù' => 0x97,
        '¤' => 0x98, 'Ô' => 0x99, 'Ü' => 0x9A, '¢' => 0x9B,
        '£' => 0x9C, 'Ù' => 0x9D, 'Û' => 0x9E, 'ƒ' => 0x9F,
        '¦' => 0xA0, '´' => 0xA1, 'ó' => 0xA2, 'ú' => 0xA3,
        '¨' => 0xA4, '¸' => 0xA5, '³' => 0xA6, '¯' => 0xA7,
        'Î' => 0xA8, '⌐' => 0xA9, '¬' => 0xAA, '½' => 0xAB,
        '¼' => 0xAC, '¾' => 0xAD, '«' => 0xAE, '»' => 0xAF,
        _ => b'?',
    }
}

// ─── CP865: Nordic ───────────────────────────────────────────────────────────
fn encode_cp865(c: char) -> u8 {
    match c {
        'Ç' => 0x80, 'ü' => 0x81, 'é' => 0x82, 'â' => 0x83,
        'ä' => 0x84, 'à' => 0x85, 'å' => 0x86, 'ç' => 0x87,
        'ê' => 0x88, 'ë' => 0x89, 'è' => 0x8A, 'ï' => 0x8B,
        'î' => 0x8C, 'ì' => 0x8D, 'Ä' => 0x8E, 'Å' => 0x8F,
        'É' => 0x90, 'æ' => 0x91, 'Æ' => 0x92, 'ô' => 0x93,
        'ö' => 0x94, 'ò' => 0x95, 'û' => 0x96, 'ù' => 0x97,
        'ÿ' => 0x98, 'Ö' => 0x99, 'Ü' => 0x9A, 'ø' => 0x9B,
        '£' => 0x9C, 'Ø' => 0x9D, '¤' => 0x9E, 'á' => 0xA0,
        'í' => 0xA1, 'ó' => 0xA2, 'ú' => 0xA3, 'ñ' => 0xA4,
        'Ñ' => 0xA5, 'ª' => 0xA6, 'º' => 0xA7, '¿' => 0xA8,
        '¬' => 0xAA, '½' => 0xAB, '¼' => 0xAC, '¡' => 0xAD,
        '«' => 0xAE,
        _ => b'?',
    }
}

// ─── CP1252: Windows Latin-1 ─────────────────────────────────────────────────
// Para chars en rango 0xA0–0xFF la codificación coincide con Unicode, así que
// podemos hacer un cast directo si cae en ese rango.
fn encode_cp1252(c: char) -> u8 {
    let code = c as u32;
    // Rango 0xA0-0xFF: coincide con ISO-8859-1 / CP1252 byte a byte
    if (0xA0..=0xFF).contains(&code) {
        return code as u8;
    }
    // Zona especial 0x80-0x9F de CP1252
    match c {
        '€' => 0x80, 'ƒ' => 0x83, '„' => 0x84, '…' => 0x85,
        '†' => 0x86, '‡' => 0x87, 'ˆ' => 0x88, '‰' => 0x89,
        'Š' => 0x8A, '‹' => 0x8B, 'Œ' => 0x8C, 'Ž' => 0x8E,
        '\u{2018}' => 0x91, '\u{2019}' => 0x92, '\u{201C}' => 0x93, '\u{201D}' => 0x94,
        '•' => 0x95, '–' => 0x96, '—' => 0x97, '˜' => 0x98,
        '™' => 0x99, 'š' => 0x9A, '›' => 0x9B, 'œ' => 0x9C,
        'ž' => 0x9E, 'Ÿ' => 0x9F,
        _ => b'?',
    }
}

// ─── CP866: Russian / Cyrillic ───────────────────────────────────────────────
fn encode_cp866(c: char) -> u8 {
    let code = c as u32;
    match code {
        // Cirílico mayúsculas А-П → 0x80-0x8F
        0x0410..=0x041F => (code - 0x0410 + 0x80) as u8,
        // Cirílico mayúsculas Р-Я → 0x90-0x9F
        0x0420..=0x042F => (code - 0x0420 + 0x90) as u8,
        // Cirílico minúsculas а-п → 0xA0-0xAF
        0x0430..=0x043F => (code - 0x0430 + 0xA0) as u8,
        // Cirílico minúsculas р-я → 0xE0-0xEF
        0x0440..=0x044F => (code - 0x0440 + 0xE0) as u8,
        // Ё / ё
        0x0401 => 0xF2,
        0x0451 => 0xF0,
        _ => b'?',
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

// ─── CP852: Eastern Europe ───────────────────────────────────────────────────
fn encode_cp852(c: char) -> u8 {
    match c {
        // Polaco
        'ą' => 0xA5, 'Ą' => 0xA4,
        'ć' => 0x86, 'Ć' => 0x8F,
        'ę' => 0xA9, 'Ę' => 0xA8,
        'ł' => 0x88, 'Ł' => 0x9D,
        'ń' => 0xE4, 'Ń' => 0xE3,
        'ś' => 0x98, 'Ś' => 0x97,
        'ź' => 0xAB, 'Ź' => 0x8D,
        'ż' => 0xBE, 'Ż' => 0xBD,
        // Checo / Eslovaco
        'č' => 0x9F, 'Č' => 0xAC,
        'š' => 0xE6, 'Š' => 0xE7,
        'ž' => 0xA7, 'Ž' => 0xA6,
        'ř' => 0xFC, 'Ř' => 0xFD,
        'ů' => 0x85, 'Ů' => 0xDE,
        'ď' => 0xD0, 'Ď' => 0xD2,
        'ě' => 0xD8, 'Ě' => 0xB7,
        'í' => 0xA1, 'Í' => 0xD6,
        'ť' => 0x9B, 'Ť' => 0x9C,
        'ý' => 0xEC, 'Ý' => 0xED,
        // Húngaro adicional
        'ő' => 0x8B, 'Ő' => 0x8A,
        'ű' => 0x8C, 'Ű' => 0x8E,
        // Compartidos con CP850
        'Ç' => 0x80, 'ü' => 0x81, 'é' => 0x82, 'â' => 0x83,
        'ä' => 0x84, 'à' => 0x85, 'ç' => 0x87, 'ê' => 0x88,
        'ë' => 0x89, 'è' => 0x8A, 'î' => 0x8C, 'Ä' => 0x8E,
        'É' => 0x90, 'ô' => 0x93, 'ö' => 0x94, 'ú' => 0xA3,
        'ó' => 0xA2, 'Ó' => 0xE0, 'Ö' => 0x99, 'Ü' => 0x9A,
        'á' => 0xA0, 'Á' => 0xB5,
        _ => b'?',
    }
}
