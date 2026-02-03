use crate::models::print_sections::Table;
use crate::commands_esc_pos::text::text_type::TextType;

pub fn process_table(table: &Table, max_width: i32, truncate: bool) -> Result<Vec<u8>, String> {
    if table.columns == 0 {
        return Ok(Vec::new());
    }

    let num_columns = table.columns as usize;

    // Calcular anchos de columnas
    let column_widths: Vec<i32> = if table.column_widths.len() == num_columns {
        table.column_widths.iter().map(|&w| w as i32).collect()
    } else {
        let equal_width = max_width / num_columns as i32;
        vec![equal_width; num_columns]
    };

    // Verificar si la suma de anchos excede max_width
    let total_width: i32 = column_widths.iter().sum();
    
    let mut output = Vec::new();

    if total_width > max_width {
        // Dividir columnas en grupos que quepan en max_width
        let column_groups = split_columns_into_groups(&column_widths, max_width);
        
        // Procesar header
        if !table.header.is_empty() {
            for group in &column_groups {
                let group_cells: Vec<_> = group.iter()
                    .filter_map(|&idx| table.header.get(idx))
                    .cloned()
                    .collect();
                let group_widths: Vec<i32> = group.iter()
                    .filter_map(|&idx| column_widths.get(idx).copied())
                    .collect();
                let row_lines = process_row(&group_cells, &group_widths, truncate);
                for line in row_lines {
                    output.extend(line.as_bytes());
                    output.extend(b"\n");
                }
            }
        }

        // Procesar body
        for row in &table.body {
            for group in &column_groups {
                let group_cells: Vec<_> = group.iter()
                    .filter_map(|&idx| row.get(idx))
                    .cloned()
                    .collect();
                let group_widths: Vec<i32> = group.iter()
                    .filter_map(|&idx| column_widths.get(idx).copied())
                    .collect();
                let row_lines = process_row(&group_cells, &group_widths, truncate);
                for line in row_lines {
                    output.extend(line.as_bytes());
                    output.extend(b"\n");
                }
            }
        }
    } else {
        // Procesar normalmente si cabe en max_width
        // Procesar header
        if !table.header.is_empty() {
            let header_lines = process_row(&table.header, &column_widths, truncate);
            for line in header_lines {
                output.extend(line.as_bytes());
                output.extend(b"\n");
            }
        }

        // Procesar body
        for row in &table.body {
            let row_lines = process_row(row, &column_widths, truncate);
            for line in row_lines {
                output.extend(line.as_bytes());
                output.extend(b"\n");
            }
        }
    }

    Ok(output)
}

fn split_columns_into_groups(column_widths: &[i32], max_width: i32) -> Vec<Vec<usize>> {
    let mut groups = Vec::new();
    let mut current_group = Vec::new();
    let mut current_width = 0;

    for (idx, &width) in column_widths.iter().enumerate() {
        if current_width + width <= max_width {
            current_group.push(idx);
            current_width += width;
        } else {
            if !current_group.is_empty() {
                groups.push(current_group);
            }
            current_group = vec![idx];
            current_width = width;
        }
    }

    if !current_group.is_empty() {
        groups.push(current_group);
    }

    groups
}

fn process_row(row: &[crate::models::print_sections::Text], column_widths: &[i32], truncate: bool) -> Vec<String> {
    // Para cada celda, obtener las líneas
    let mut cell_lines: Vec<Vec<String>> = Vec::new();
    for (i, cell) in row.iter().enumerate() {
        let width = if i < column_widths.len() { column_widths[i] } else { 10 }; // default
        let text = remove_accents(&cell.text);
        let lines = if truncate {
            vec![truncate_text(&text, width as usize)]
        } else {
            wrap_text(&text, width as usize)
        };
        cell_lines.push(lines);
    }

    // Número máximo de líneas
    let max_lines = cell_lines.iter().map(|l| l.len()).max().unwrap_or(1);

    // Generar líneas de output
    let mut result = Vec::new();
    for line_idx in 0..max_lines {
        let mut line = String::new();
        for (i, cell) in cell_lines.iter().enumerate() {
            let part = if line_idx < cell.len() { &cell[line_idx] } else { "" };
            let padded = format!("{:<width$}", part, width = column_widths.get(i).unwrap_or(&10) as usize);
            line.push_str(&padded);
        }
        result.push(line.trim_end().to_string());
    }

    result
}

fn truncate_text(text: &str, width: usize) -> String {
    if text.len() <= width {
        text.to_string()
    } else {
        text.chars().take(width).collect()
    }
}

fn wrap_text(text: &str, width: usize) -> Vec<String> {
    let mut lines = Vec::new();
    let mut current = String::new();
    for word in text.split_whitespace() {
        if current.len() + word.len() + 1 > width {
            if !current.is_empty() {
                lines.push(current);
                current = word.to_string();
            } else {
                // Palabra muy larga, truncar
                lines.push(word.chars().take(width).collect());
            }
        } else {
            if !current.is_empty() {
                current.push(' ');
            }
            current.push_str(word);
        }
    }
    if !current.is_empty() {
        lines.push(current);
    }
    lines
}

fn remove_accents(text: &str) -> String {
    text.chars()
        .map(|c| match c {
            'á' | 'à' | 'â' | 'ä' | 'ã' => 'a',
            'é' | 'è' | 'ê' | 'ë' => 'e',
            'í' | 'ì' | 'î' | 'ï' => 'i',
            'ó' | 'ò' | 'ô' | 'ö' | 'õ' => 'o',
            'ú' | 'ù' | 'û' | 'ü' => 'u',
            'Á' | 'À' | 'Â' | 'Ä' | 'Ã' => 'A',
            'É' | 'È' | 'Ê' | 'Ë' => 'E',
            'Í' | 'Ì' | 'Î' | 'Ï' => 'I',
            'Ó' | 'Ò' | 'Ô' | 'Ö' | 'Õ' => 'O',
            'Ú' | 'Ù' | 'Û' | 'Ü' => 'U',
            'ñ' => 'n',
            'Ñ' => 'N',
            _ => c,
        })
        .collect()
}
