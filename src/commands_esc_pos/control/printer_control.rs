use crate::models::print_sections::{
    Beep, CharSpacing, Cut, Drawer, Feed, LeftMargin, LineSpacing, Position, PrintAreaWidth,
    TabStops,
};

/// ESC/POS GS V mode byte for partial cut (also used as default)
pub const CUT_MODE_PARTIAL: u8 = 65;
/// ESC/POS GS V mode byte for full cut
pub const CUT_MODE_FULL: u8 = 66;

/// Comandos de control de la impresora térmica
pub struct PrinterControl;

impl PrinterControl {
    /// Inicializa la impresora (resetea a configuración por defecto)
    /// ESC @
    pub fn initialize() -> Vec<u8> {
        vec![0x1B, 0x40]
    }

    /// Múltiples saltos de línea
    /// # Arguments
    /// * `lines` - Número de líneas a saltar
    pub fn line_feed_multiple(lines: usize) -> Vec<u8> {
        vec![0x0A; lines]
    }

    /// Salto de línea simple
    /// LF
    pub fn line_feed() -> Vec<u8> {
        vec![0x0A]
    }

    /// Retorno de carro
    /// CR
    // pub fn carriage_return() -> Vec<u8> {
    //     vec![0x0D]
    // }

    /// Avance de papel (feed)
    /// ESC d n
    /// # Arguments
    /// * `lines` - Número de líneas a avanzar (0-255)
    pub fn feed_paper(lines: u8) -> Vec<u8> {
        vec![0x1B, 0x64, lines]
    }

    /// Avance de papel en unidades de puntos
    /// ESC J n
    /// # Arguments
    /// * `dots` - Número de puntos a avanzar (0-255)
    pub fn feed_paper_dots(dots: u8) -> Vec<u8> {
        vec![0x1B, 0x4A, dots]
    }

    /// Corte de papel con avance
    /// GS V m n
    /// # Arguments
    /// * `mode` - 0 = completo, 1 = parcial, 65 = parcial alt, 66 = parcial alt2
    /// * `feed_lines` - Líneas a avanzar antes de cortar (0-255)
    pub fn cut_paper_with_feed(mode: u8, feed_lines: u8) -> Vec<u8> {
        vec![0x1D, 0x56, mode, feed_lines]
    }

    /// Activa el cajón de dinero (cash drawer) - Puerto 1
    /// ESC p m t1 t2
    /// # Arguments
    /// * `pulse_time` - Tiempo del pulso en milisegundos (0-500ms, en pasos de 2ms)
    pub fn open_cash_drawer_pin2(pulse_time: u16) -> Vec<u8> {
        let mut t = pulse_time / 2; // Convertir a unidades de 2ms
        if t > 255 {
            t = 255;
        }
        vec![0x1B, 0x70, 0x00, t as u8, t as u8]
    }

    /// Activa el cajón de dinero (cash drawer) - Puerto 2
    /// ESC p m t1 t2
    /// # Arguments
    /// * `pulse_time` - Tiempo del pulso en milisegundos
    pub fn open_cash_drawer_pin5(pulse_time: u16) -> Vec<u8> {
        let mut t = pulse_time / 2;
        if t > 255 {
            t = 255;
        }
        vec![0x1B, 0x70, 0x01, t as u8, t as u8]
    }

    /// Procesa sección Feed
    pub fn process_feed(feed: &Feed) -> Result<Vec<u8>, String> {
        match feed.feed_type.as_str() {
            "lines" => Ok(Self::feed_paper(feed.value)),
            "dots" => Ok(Self::feed_paper_dots(feed.value)),
            "line_feed" => Ok(Self::line_feed_multiple(feed.value as usize)),
            _ => Err("Unknown feed type".to_string()),
        }
    }

    /// Procesa sección Cut
    pub fn process_cut(cut: &Cut) -> Result<Vec<u8>, String> {
        let mode = match cut.mode.as_str() {
            "full" | "partial_alt2" => CUT_MODE_FULL,
            _ => CUT_MODE_PARTIAL,
        };
        Ok(Self::cut_paper_with_feed(mode, cut.feed))
    }

    /// Procesa sección Beep (Epson `ESC ( A`)
    pub fn process_beep(beep: &Beep) -> Result<Vec<u8>, String> {
        let times = if beep.times == 0 { 1 } else { beep.times };
        let duration = if beep.duration == 0 {
            100
        } else {
            beep.duration
        };
        Ok(Self::beep_custom(times, duration))
    }

    /// Procesa sección Beep2 (buzzer genérico `ESC B n t`)
    ///
    /// Alternativa a [`Self::process_beep`] para impresoras genéricas/clones que
    /// ignoran el `ESC ( A` de Epson.
    pub fn process_beep2(beep: &Beep) -> Result<Vec<u8>, String> {
        let times = if beep.times == 0 { 1 } else { beep.times };
        let duration = if beep.duration == 0 { 3 } else { beep.duration };
        Ok(Self::beep_generic(times, duration))
    }

    /// Buzzer genérico `ESC B n t`
    /// # Arguments
    /// * `count` - Número de beeps (1-9)
    /// * `duration` - Duración por beep en unidades de ~100ms (1-9)
    pub fn beep_generic(count: u8, duration: u8) -> Vec<u8> {
        let count = count.clamp(1, 9);
        let duration = duration.clamp(1, 9);
        vec![0x1B, 0x42, count, duration]
    }

    /// Interlineado vertical
    /// `ESC 3 n` (n puntos) o `ESC 2` (default ~1/6") cuando `value` es `None`.
    pub fn set_line_spacing(value: Option<u8>) -> Vec<u8> {
        match value {
            Some(n) => vec![0x1B, 0x33, n],
            None => vec![0x1B, 0x32],
        }
    }

    /// Espaciado a la derecha de cada carácter
    /// `ESC SP n`
    pub fn set_char_spacing(n: u8) -> Vec<u8> {
        vec![0x1B, 0x20, n]
    }

    /// Posición horizontal absoluta para el siguiente dato
    /// `ESC $ nL nH`
    pub fn set_absolute_position(pos: u16) -> Vec<u8> {
        vec![0x1B, 0x24, (pos & 0xFF) as u8, ((pos >> 8) & 0xFF) as u8]
    }

    /// Define posiciones de tabulación horizontal
    /// `ESC D n1 n2 ... NUL`
    pub fn set_tab_stops(positions: &[u8]) -> Vec<u8> {
        let mut out = vec![0x1B, 0x44];
        for &p in positions.iter().take(32) {
            if p == 0 {
                continue; // 0 terminaría la lista antes de tiempo
            }
            out.push(p);
        }
        out.push(0x00); // NUL: fin de la lista
        out
    }

    /// Margen izquierdo (modo estándar)
    /// `GS L nL nH`
    pub fn set_left_margin(margin: u16) -> Vec<u8> {
        vec![0x1D, 0x4C, (margin & 0xFF) as u8, ((margin >> 8) & 0xFF) as u8]
    }

    /// Ancho del área de impresión (modo estándar)
    /// `GS W nL nH`
    pub fn set_print_area_width(width: u16) -> Vec<u8> {
        vec![0x1D, 0x57, (width & 0xFF) as u8, ((width >> 8) & 0xFF) as u8]
    }

    /// Procesa sección LineSpacing
    pub fn process_line_spacing(ls: &LineSpacing) -> Result<Vec<u8>, String> {
        Ok(Self::set_line_spacing(ls.value))
    }

    /// Procesa sección CharSpacing
    pub fn process_char_spacing(cs: &CharSpacing) -> Result<Vec<u8>, String> {
        Ok(Self::set_char_spacing(cs.value))
    }

    /// Procesa sección Position
    pub fn process_position(p: &Position) -> Result<Vec<u8>, String> {
        Ok(Self::set_absolute_position(p.value))
    }

    /// Procesa sección TabStops
    pub fn process_tab_stops(t: &TabStops) -> Result<Vec<u8>, String> {
        if t.positions.is_empty() {
            return Err("Tab stops cannot be empty".to_string());
        }
        Ok(Self::set_tab_stops(&t.positions))
    }

    /// Procesa sección LeftMargin
    pub fn process_left_margin(m: &LeftMargin) -> Result<Vec<u8>, String> {
        Ok(Self::set_left_margin(m.value))
    }

    /// Procesa sección PrintAreaWidth
    pub fn process_print_area_width(w: &PrintAreaWidth) -> Result<Vec<u8>, String> {
        Ok(Self::set_print_area_width(w.value))
    }

    /// Procesa sección Drawer
    pub fn process_drawer(drawer: &Drawer) -> Result<Vec<u8>, String> {
        if drawer.pin == 2 {
            Ok(Self::open_cash_drawer_pin2(drawer.pulse_time))
        } else {
            Ok(Self::open_cash_drawer_pin5(drawer.pulse_time))
        }
    }

    /// Emite un beep/sonido en la impresora
    /// ESC ( A pL pH n m t
    /// # Arguments
    /// * `count` - Número de veces que suena (1-9)
    /// * `duration` - Duración en centésimas de segundo (1-9)
    pub fn beep_custom(count: u8, duration: u8) -> Vec<u8> {
        let count = count.clamp(1, 9);
        let duration = duration.clamp(1, 9);

        vec![
            0x1B, 0x28, 0x41, // ESC ( A
            0x05, 0x00,     // pL pH (longitud de parámetros = 5)
            0x61,     // n (buzzer type = 97 = 0x61)
            count,    // m (número de veces)
            duration, // t (duración en centésimas de segundo)
        ]
    }

    // /// Establece el interlineado
    // /// ESC 3 n
    // /// # Arguments
    // /// * `spacing` - Espaciado en puntos (0-255)
    // pub fn set_line_spacing(spacing: u8) -> Vec<u8> {
    //     vec![0x1B, 0x33, spacing]
    // }

    // /// Establece el espaciado de caracteres
    // /// ESC SP n
    // /// # Arguments
    // /// * `spacing` - Espaciado en puntos (0-255)
    // pub fn set_character_spacing(spacing: u8) -> Vec<u8> {
    //     vec![0x1B, 0x20, spacing]
    // }

    // /// Habilita/deshabilita el modo de impresión automática
    // /// ESC c 5 n
    // /// # Arguments
    // /// * `enable` - true para habilitar, false para deshabilitar
    // pub fn set_print_mode(enable: bool) -> Vec<u8> {
    //     vec![0x1B, 0x63, 0x35, if enable { 1 } else { 0 }]
    // }

    // /// Obtiene el estado de la impresora
    // /// DLE EOT n
    // /// # Arguments
    // /// * `status_type` - Tipo de estado a consultar (1-4)
    // pub fn get_printer_status(status_type: u8) -> Vec<u8> {
    //     vec![0x10, 0x04, status_type]
    // }

    // /// Comando para imprimir y avanzar papel
    // /// ESC d n
    // pub fn print_and_feed(lines: u8) -> Vec<u8> {
    //     Self::feed_paper(lines)
    // }

    // /// Modo de página (permite posicionamiento absoluto)
    // /// ESC L
    // pub fn enable_page_mode() -> Vec<u8> {
    //     vec![0x1B, 0x4C]
    // }

    // /// Modo estándar (desactiva modo página)
    // /// ESC S
    // pub fn enable_standard_mode() -> Vec<u8> {
    //     vec![0x1B, 0x53]
    // }

    // /// Establece el área de impresión en modo página
    // /// ESC W xL xH yL yH dxL dxH dyL dyH
    // pub fn set_page_mode_area(x: u16, y: u16, width: u16, height: u16) -> Vec<u8> {
    //     vec![
    //         0x1B,
    //         0x57,
    //         (x & 0xFF) as u8,
    //         ((x >> 8) & 0xFF) as u8,
    //         (y & 0xFF) as u8,
    //         ((y >> 8) & 0xFF) as u8,
    //         (width & 0xFF) as u8,
    //         ((width >> 8) & 0xFF) as u8,
    //         (height & 0xFF) as u8,
    //         ((height >> 8) & 0xFF) as u8,
    //     ]
    // }

    // /// Imprime el contenido del buffer en modo página
    // /// ESC FF
    // pub fn print_page_mode() -> Vec<u8> {
    //     vec![0x1B, 0x0C]
    // }
}
