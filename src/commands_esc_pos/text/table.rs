use crate::commands_esc_pos::text::encoder::TextEncoder;
use crate::commands_esc_pos::text::table_render::{render_row, RenderedLine};
use crate::models::print_sections::{Table, Text};

pub fn process_section(
    table: &Table,
    chars_per_line: i32,
    encoder: &TextEncoder,
) -> Result<Vec<u8>, String> {
    validate_table(table, chars_per_line)?;
    process_table(table, chars_per_line, table.truncate, encoder)
}

pub fn process_table(
    table: &Table,
    max_width: i32,
    truncate: bool,
    encoder: &TextEncoder,
) -> Result<Vec<u8>, String> {
    if table.columns == 0 {
        return Ok(Vec::new());
    }

    let column_widths = resolve_column_widths(table, max_width);
    let column_groups = build_column_groups(&column_widths, max_width, table.columns as usize);
    let mut output = Vec::new();

    if let Some(header) = &table.header {
        if !header.is_empty() {
            write_row_groups(
                &mut output,
                header,
                &column_widths,
                &column_groups,
                truncate,
                encoder,
            )?;
        }
    }

    for row in &table.body {
        write_row_groups(
            &mut output,
            row,
            &column_widths,
            &column_groups,
            truncate,
            encoder,
        )?;
    }

    Ok(output)
}

fn validate_table(table: &Table, chars_per_line: i32) -> Result<(), String> {
    validate_column_widths(table, chars_per_line)?;
    validate_header(table)?;
    validate_rows(table)
}

fn validate_column_widths(table: &Table, chars_per_line: i32) -> Result<(), String> {
    if let Some(widths) = &table.column_widths {
        let total: i32 = widths.iter().map(|&width| i32::from(width)).sum();
        if total != chars_per_line {
            return Err(format!(
                "column_widths sum ({}) must equal paper chars_per_line ({})",
                total, chars_per_line
            ));
        }
    }

    Ok(())
}

fn validate_header(table: &Table) -> Result<(), String> {
    if let Some(header) = &table.header {
        let num_columns = table.columns as usize;
        if !header.is_empty() && header.len() != num_columns {
            return Err(format!(
                "Table header has {} cells but {} columns declared",
                header.len(),
                num_columns
            ));
        }
    }

    Ok(())
}

fn validate_rows(table: &Table) -> Result<(), String> {
    let num_columns = table.columns as usize;

    for (row_idx, row) in table.body.iter().enumerate() {
        if row.len() != num_columns {
            return Err(format!(
                "Table row {} has {} cells but {} columns declared",
                row_idx,
                row.len(),
                num_columns
            ));
        }
    }

    Ok(())
}

fn resolve_column_widths(table: &Table, max_width: i32) -> Vec<i32> {
    let num_columns = table.columns as usize;

    if let Some(widths) = &table.column_widths {
        if widths.len() == num_columns {
            return widths.iter().map(|&width| i32::from(width)).collect();
        }
    }

    let equal_width = max_width / num_columns as i32;
    vec![equal_width; num_columns]
}

fn build_column_groups(
    column_widths: &[i32],
    max_width: i32,
    num_columns: usize,
) -> Vec<Vec<usize>> {
    let total_width: i32 = column_widths.iter().sum();
    if total_width <= max_width {
        return vec![(0..num_columns).collect()];
    }

    split_columns_into_groups(column_widths, max_width)
}

fn split_columns_into_groups(column_widths: &[i32], max_width: i32) -> Vec<Vec<usize>> {
    let mut groups = Vec::new();
    let mut current_group = Vec::new();
    let mut current_width = 0;

    for (idx, &width) in column_widths.iter().enumerate() {
        if current_width + width <= max_width {
            current_group.push(idx);
            current_width += width;
            continue;
        }

        if !current_group.is_empty() {
            groups.push(current_group);
        }

        current_group = vec![idx];
        current_width = width;
    }

    if !current_group.is_empty() {
        groups.push(current_group);
    }

    groups
}

fn write_row_groups(
    output: &mut Vec<u8>,
    row: &[Text],
    column_widths: &[i32],
    column_groups: &[Vec<usize>],
    truncate: bool,
    encoder: &TextEncoder,
) -> Result<(), String> {
    for group in column_groups {
        let widths = group_widths(group, column_widths);
        let cells = group_cells(group, row);
        write_rendered_lines(output, render_row(&cells, &widths, truncate, encoder)?);
    }

    Ok(())
}

fn group_widths(group: &[usize], column_widths: &[i32]) -> Vec<i32> {
    group
        .iter()
        .filter_map(|&idx| column_widths.get(idx).copied())
        .collect()
}

fn group_cells(group: &[usize], row: &[Text]) -> Vec<Text> {
    group
        .iter()
        .filter_map(|&idx| row.get(idx))
        .cloned()
        .collect()
}

fn write_rendered_lines(output: &mut Vec<u8>, lines: Vec<RenderedLine>) {
    for line in lines {
        output.extend_from_slice(&line.bytes);
        output.push(b'\n');
    }
}
