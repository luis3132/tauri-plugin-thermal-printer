use crate::commands_esc_pos::text::encoder::TextEncoder;
use crate::models::print_sections::{GlobalStyles, Line, Subtitle, Text, Title};

#[derive(Debug, Clone, Copy)]
pub enum TextType {
    // Tamaño de texto
    Normal,
    DoubleHeight,
    DoubleWidth,
    DoubleSize,

    // Estilo de texto
    BoldOn,
    BoldOff,
    UnderlineOn,
    UnderlineOff,
    ItalicOn,
    ItalicOff,

    // Alineación
    AlignLeft,
    AlignCenter,
    AlignRight,

    // Otros...
    InvertOn,
    InvertOff,
    FontA,
    FontB,
    FontC,
    RotateOn,
    RotateOff,
    UpsideDownOn,
    UpsideDownOff,
}

impl TextType {
    /// Retorna el comando ESC/POS como un slice de bytes.
    /// Usamos &'static [u8] porque son constantes grabadas en el binario.
    pub fn command(&self) -> &'static [u8] {
        match self {
            // Tamaño
            Self::Normal => &[0x1B, 0x21, 0x00],
            Self::DoubleHeight => &[0x1B, 0x21, 0x10],
            Self::DoubleWidth => &[0x1B, 0x21, 0x20],
            Self::DoubleSize => &[0x1B, 0x21, 0x30],

            // Estilo
            Self::BoldOn => &[0x1B, 0x45, 0x01],
            Self::BoldOff => &[0x1B, 0x45, 0x00],
            Self::UnderlineOn => &[0x1B, 0x2D, 0x01],
            Self::UnderlineOff => &[0x1B, 0x2D, 0x00],
            Self::ItalicOn => &[0x1B, 0x34],
            Self::ItalicOff => &[0x1B, 0x35],

            // Alineación
            Self::AlignLeft => &[0x1B, 0x61, 0x00],
            Self::AlignCenter => &[0x1B, 0x61, 0x01],
            Self::AlignRight => &[0x1B, 0x61, 0x02],

            // Inversión
            Self::InvertOn => &[0x1D, 0x42, 0x01],
            Self::InvertOff => &[0x1D, 0x42, 0x00],

            // Fuente
            Self::FontA => &[0x1B, 0x4D, 0x00],
            Self::FontB => &[0x1B, 0x4D, 0x01],
            Self::FontC => &[0x1B, 0x4D, 0x02],

            // Rotación y Upside Down
            Self::RotateOn => &[0x1B, 0x56, 0x01],
            Self::RotateOff => &[0x1B, 0x56, 0x00],
            Self::UpsideDownOn => &[0x1B, 0x7B, 0x01],
            Self::UpsideDownOff => &[0x1B, 0x7B, 0x00],
        }
    }
}

/// Genera los comandos ESC/POS necesarios para pasar de un estilo a otro
pub fn get_styles_diff(old: &GlobalStyles, new: &GlobalStyles) -> Vec<u8> {
    let mut output = Vec::new();

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
        match new_font.as_str() {
            "a" => output.extend_from_slice(TextType::FontA.command()),
            "b" => output.extend_from_slice(TextType::FontB.command()),
            "c" => output.extend_from_slice(TextType::FontC.command()),
            _ => {}
        }
    }

    let old_size = get_string(&old.size, "normal");
    let new_size = get_string(&new.size, "normal");
    if old_size != new_size {
        match new_size.as_str() {
            "normal" => output.extend_from_slice(TextType::Normal.command()),
            "width" => output.extend_from_slice(TextType::DoubleWidth.command()),
            "height" => output.extend_from_slice(TextType::DoubleHeight.command()),
            "double" => output.extend_from_slice(TextType::DoubleSize.command()),
            _ => {}
        }
    }

    let old_align = get_string(&old.align, "left");
    let new_align = get_string(&new.align, "left");
    if old_align != new_align {
        match new_align.as_str() {
            "left" => output.extend_from_slice(TextType::AlignLeft.command()),
            "center" => output.extend_from_slice(TextType::AlignCenter.command()),
            "right" => output.extend_from_slice(TextType::AlignRight.command()),
            _ => {}
        }
    }

    output
}

/// Procesa encabezado (centrado, doble tamaño)
pub fn process_title(
    title: &Title,
    current_styles: &GlobalStyles,
    encoder: &TextEncoder,
) -> Result<Vec<u8>, String> {
    let mut output = Vec::new();

    let base_styles = title.styles.as_ref().unwrap_or(current_styles).clone();
    let mut effective_styles = base_styles;
    effective_styles.size = Some("double".to_string());
    effective_styles.align = Some("center".to_string());

    output.extend_from_slice(&get_styles_diff(current_styles, &effective_styles));
    output.extend(encoder.encode_text(&title.text)?);
    output.extend_from_slice(b"\n");
    output.extend_from_slice(&get_styles_diff(&effective_styles, current_styles));

    Ok(output)
}

/// Procesa subtítulo (doble altura, negrita)
pub fn process_subtitle(
    subtitle: &Subtitle,
    current_styles: &GlobalStyles,
    encoder: &TextEncoder,
) -> Result<Vec<u8>, String> {
    let mut output = Vec::new();

    let base_styles = subtitle.styles.as_ref().unwrap_or(current_styles).clone();
    let mut effective_styles = base_styles;
    effective_styles.size = Some("height".to_string());
    effective_styles.bold = Some(true);

    output.extend_from_slice(&get_styles_diff(current_styles, &effective_styles));
    output.extend(encoder.encode_text(&subtitle.text)?);
    output.extend_from_slice(b"\n");
    output.extend_from_slice(&get_styles_diff(&effective_styles, current_styles));

    Ok(output)
}

/// Procesa texto con estilos libres
pub fn process_text(
    text: &Text,
    current_styles: &GlobalStyles,
    encoder: &TextEncoder,
) -> Result<Vec<u8>, String> {
    let mut output = Vec::new();

    let effective_styles = text.styles.as_ref().unwrap_or(current_styles).clone();

    output.extend_from_slice(&get_styles_diff(current_styles, &effective_styles));
    output.extend(encoder.encode_text(&text.text)?);
    output.extend_from_slice(b"\n");
    output.extend_from_slice(&get_styles_diff(&effective_styles, current_styles));

    Ok(output)
}

/// Procesa línea horizontal repetiendo un carácter
pub fn process_line(
    line: &Line,
    current_styles: &GlobalStyles,
    chars_per_line: i32,
) -> Result<Vec<u8>, String> {
    let size = current_styles
        .size
        .as_deref()
        .unwrap_or("normal")
        .to_lowercase();
    let width_multiplier = match size.as_str() {
        "width" | "double" => 0.5,
        _ => 1.0,
    };

    let font = current_styles.font.as_deref().unwrap_or("a").to_lowercase();
    let font_multiplier = match font.as_str() {
        "b" => 1.3,
        "c" => 1.5,
        _ => 1.0,
    };

    let char_count =
        ((chars_per_line as f32 * width_multiplier * font_multiplier) as usize).max(10);

    let character = line.character.chars().next().unwrap_or('-');
    let line_text = character.to_string().repeat(char_count);

    let mut output = Vec::new();
    output.extend_from_slice(line_text.as_bytes());
    output.extend_from_slice(b"\n");
    Ok(output)
}
