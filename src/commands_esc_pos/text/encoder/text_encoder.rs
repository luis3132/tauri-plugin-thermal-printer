use crate::commands_esc_pos::text::code_page::CodePage;
use unicode_width::UnicodeWidthChar;

use super::accent_remover::accent_remover_bytes;
use super::encode::Encode;

#[derive(Debug, Clone)]
pub struct TextEncoder {
    encode: Encode,
    use_gbk: bool,
}

#[derive(Debug, Clone)]
pub(crate) struct EncodedChar {
    pub(crate) bytes: Vec<u8>,
    pub(crate) width: usize,
}

impl TextEncoder {
    pub fn from_code_page(code_page: &CodePage) -> Self {
        Self {
            encode: code_page.encode,
            use_gbk: code_page.use_gbk,
        }
    }

    pub fn encode_text(&self, text: &str) -> Result<Vec<u8>, String> {
        let mut output = Vec::with_capacity(text.len());

        for ch in text.chars() {
            output.extend_from_slice(&self.encode_char(ch)?.bytes);
        }

        Ok(output)
    }

    pub(crate) fn encode_char(&self, ch: char) -> Result<EncodedChar, String> {
        match self.encode.encoding() {
            Some(encoding) => self.encode_with_encoding(ch, encoding),
            None => self.encode_accent_remover(ch),
        }
    }

    fn encode_accent_remover(&self, ch: char) -> Result<EncodedChar, String> {
        if let Some(mapped) = accent_remover_bytes(ch) {
            return Ok(multi_byte_char(mapped));
        }

        if ch.is_ascii() {
            return Ok(single_byte_char(ch as u8));
        }

        self.encode_with_optional_gbk(ch)
    }

    fn encode_with_encoding(
        &self,
        ch: char,
        encoding: &'static encoding_rs::Encoding,
    ) -> Result<EncodedChar, String> {
        let value = ch.to_string();
        let (encoded, _, had_errors) = encoding.encode(&value);

        if had_errors {
            return self.encode_with_optional_gbk(ch);
        }

        Ok(EncodedChar {
            bytes: encoded.into_owned(),
            width: display_width(ch),
        })
    }

    fn encode_with_optional_gbk(&self, ch: char) -> Result<EncodedChar, String> {
        if !self.use_gbk {
            return Ok(passthrough_char(ch));
        }

        encode_with_gbk(ch)
    }
}

fn single_byte_char(byte: u8) -> EncodedChar {
    EncodedChar {
        bytes: vec![byte],
        width: 1,
    }
}

fn multi_byte_char(bytes: &'static [u8]) -> EncodedChar {
    EncodedChar {
        bytes: bytes.to_vec(),
        width: bytes.len(),
    }
}

fn encode_with_gbk(ch: char) -> Result<EncodedChar, String> {
    let value = ch.to_string();
    let (encoded, _, had_errors) = encoding_rs::GBK.encode(&value);

    if had_errors {
        return Ok(passthrough_char(ch));
    }

    Ok(EncodedChar {
        bytes: encoded.into_owned(),
        width: display_width(ch),
    })
}

fn passthrough_char(ch: char) -> EncodedChar {
    EncodedChar {
        bytes: ch.to_string().into_bytes(),
        width: display_width(ch),
    }
}

fn display_width(ch: char) -> usize {
    UnicodeWidthChar::width_cjk(ch)
        .or_else(|| UnicodeWidthChar::width(ch))
        .unwrap_or(1)
        .max(1)
}
