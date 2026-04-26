use crate::commands_esc_pos::text::encoder::{EncodedChar, TextEncoder};
use crate::models::print_sections::Text;

#[derive(Debug, Clone, Default)]
pub struct RenderedLine {
    pub bytes: Vec<u8>,
    width: usize,
}

pub fn render_row(
    row: &[Text],
    column_widths: &[i32],
    truncate: bool,
    encoder: &TextEncoder,
) -> Result<Vec<RenderedLine>, String> {
    let rendered_cells = render_cells(row, column_widths, truncate, encoder)?;
    let max_lines = rendered_cells.iter().map(Vec::len).max().unwrap_or(1);
    let mut output = Vec::new();

    for line_idx in 0..max_lines {
        output.push(render_output_line(&rendered_cells, column_widths, line_idx));
    }

    Ok(output)
}

fn render_cells(
    row: &[Text],
    column_widths: &[i32],
    truncate: bool,
    encoder: &TextEncoder,
) -> Result<Vec<Vec<RenderedLine>>, String> {
    let mut rendered = Vec::new();

    for (index, cell) in row.iter().enumerate() {
        let width = column_widths.get(index).copied().unwrap_or(10).max(0) as usize;
        rendered.push(render_cell(&cell.text, width, truncate, encoder)?);
    }

    Ok(rendered)
}

fn render_output_line(
    rendered_cells: &[Vec<RenderedLine>],
    column_widths: &[i32],
    line_idx: usize,
) -> RenderedLine {
    let mut output = RenderedLine::default();

    for (index, cell) in rendered_cells.iter().enumerate() {
        let segment = cell.get(line_idx).cloned().unwrap_or_default();
        output.bytes.extend_from_slice(&segment.bytes);

        if index + 1 < rendered_cells.len() {
            push_padding(
                &mut output.bytes,
                column_widths[index] as usize,
                segment.width,
            );
        }
    }

    output.width = output.bytes.len();
    output
}

fn render_cell(
    text: &str,
    width: usize,
    truncate: bool,
    encoder: &TextEncoder,
) -> Result<Vec<RenderedLine>, String> {
    if truncate {
        return Ok(vec![truncate_text(text, width, encoder)?]);
    }

    wrap_text(text, width, encoder)
}

fn truncate_text(text: &str, width: usize, encoder: &TextEncoder) -> Result<RenderedLine, String> {
    if width == 0 {
        return Ok(RenderedLine::default());
    }

    let mut output = RenderedLine::default();

    for ch in text.chars() {
        let encoded = encoder.encode_char(ch)?;
        if output.width + encoded.width > width {
            break;
        }

        push_encoded_char(&mut output, encoded);
    }

    Ok(output)
}

fn wrap_text(text: &str, width: usize, encoder: &TextEncoder) -> Result<Vec<RenderedLine>, String> {
    if width == 0 {
        return Ok(vec![RenderedLine::default()]);
    }

    let mut lines = Vec::new();
    let mut current = RenderedLine::default();

    for ch in text.chars() {
        push_wrapped_char(&mut lines, &mut current, ch, width, encoder)?;
    }

    if !current.bytes.is_empty() {
        lines.push(current);
    }

    if lines.is_empty() {
        lines.push(RenderedLine::default());
    }

    Ok(lines)
}

fn push_wrapped_char(
    lines: &mut Vec<RenderedLine>,
    current: &mut RenderedLine,
    ch: char,
    width: usize,
    encoder: &TextEncoder,
) -> Result<(), String> {
    let encoded = encoder.encode_char(ch)?;

    if current.width > 0 && current.width + encoded.width > width {
        finish_wrapped_line(lines, current);
        if ch.is_whitespace() {
            return Ok(());
        }
    }

    if current.width == 0 && ch.is_whitespace() {
        return Ok(());
    }

    push_encoded_char(current, encoded);

    if current.width >= width {
        finish_wrapped_line(lines, current);
    }

    Ok(())
}

fn finish_wrapped_line(lines: &mut Vec<RenderedLine>, current: &mut RenderedLine) {
    lines.push(std::mem::take(current));
}

fn push_encoded_char(line: &mut RenderedLine, encoded: EncodedChar) {
    line.width += encoded.width;
    line.bytes.extend_from_slice(&encoded.bytes);
}

fn push_padding(bytes: &mut Vec<u8>, width: usize, text_width: usize) {
    bytes.extend(std::iter::repeat(b' ').take(width.saturating_sub(text_width)));
}
