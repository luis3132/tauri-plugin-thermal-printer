/// Comandos de control de la impresora térmica
pub struct PrinterControl;

impl PrinterControl {
    /// Inicializa la impresora (resetea a configuración por defecto)
    /// ESC @
    pub fn initialize() -> Vec<u8> {
        vec![0x1B, 0x40]
    }

    /// Salto de línea simple
    /// LF
    pub fn line_feed() -> Vec<u8> {
        vec![0x0A]
    }

    /// Múltiples saltos de línea
    /// # Arguments
    /// * `lines` - Número de líneas a saltar
    pub fn line_feed_multiple(lines: usize) -> Vec<u8> {
        vec![0x0A; lines]
    }

    /// Retorno de carro
    /// CR
    pub fn carriage_return() -> Vec<u8> {
        vec![0x0D]
    }

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

    /// Corte de papel completo
    /// GS V 0 (o GS V 48)
    pub fn cut_paper_full() -> Vec<u8> {
        vec![0x1D, 0x56, 0x00]
    }

    /// Corte de papel parcial (versión estándar)
    /// GS V 1 (o GS V 49)
    pub fn cut_paper_partial() -> Vec<u8> {
        vec![0x1D, 0x56, 0x01]
    }

    /// Corte de papel parcial alternativo (más compatible)
    /// GS V 66 n
    pub fn cut_paper_partial_alt() -> Vec<u8> {
        vec![0x1D, 0x56, 0x42, 0x00]
    }

    /// Corte de papel con avance
    /// GS V m n
    /// # Arguments
    /// * `mode` - 0 = completo, 1 = parcial, 65 = parcial alt, 66 = parcial alt2
    /// * `feed_lines` - Líneas a avanzar antes de cortar (0-255)
    pub fn cut_paper_with_feed(mode: u8, feed_lines: u8) -> Vec<u8> {
        vec![0x1D, 0x56, mode, feed_lines]
    }

    /// Corte de papel con avance (versión simplificada)
    /// ESC i - Corte parcial (más compatible)
    pub fn cut_paper_simple() -> Vec<u8> {
        vec![0x1B, 0x69]
    }

    /// Corte de papel completo (versión simplificada)
    /// ESC m - Corte completo
    pub fn cut_paper_full_simple() -> Vec<u8> {
        vec![0x1B, 0x6D]
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
            0x05, 0x00,       // pL pH (longitud de parámetros = 5)
            0x61,             // n (buzzer type = 97 = 0x61)
            count,            // m (número de veces)
            duration,         // t (duración en centésimas de segundo)
        ]
    }

    /// Beep simple con valores por defecto
    pub fn beep() -> Vec<u8> {
        Self::beep_custom(2, 3)
    }

    /// Beep de error (más largo)
    pub fn beep_error() -> Vec<u8> {
        Self::beep_custom(3, 5)
    }

    /// Beep de éxito (corto)
    pub fn beep_success() -> Vec<u8> {
        Self::beep_custom(1, 2)
    }

    /// Establece el interlineado por defecto
    /// ESC 2
    pub fn set_default_line_spacing() -> Vec<u8> {
        vec![0x1B, 0x32]
    }

    /// Establece el interlineado
    /// ESC 3 n
    /// # Arguments
    /// * `spacing` - Espaciado en puntos (0-255)
    pub fn set_line_spacing(spacing: u8) -> Vec<u8> {
        vec![0x1B, 0x33, spacing]
    }

    /// Establece el espaciado de caracteres
    /// ESC SP n
    /// # Arguments
    /// * `spacing` - Espaciado en puntos (0-255)
    pub fn set_character_spacing(spacing: u8) -> Vec<u8> {
        vec![0x1B, 0x20, spacing]
    }

    /// Habilita/deshabilita el modo de impresión automática
    /// ESC c 5 n
    /// # Arguments
    /// * `enable` - true para habilitar, false para deshabilitar
    pub fn set_print_mode(enable: bool) -> Vec<u8> {
        vec![0x1B, 0x63, 0x35, if enable { 1 } else { 0 }]
    }

    /// Obtiene el estado de la impresora
    /// DLE EOT n
    /// # Arguments
    /// * `status_type` - Tipo de estado a consultar (1-4)
    pub fn get_printer_status(status_type: u8) -> Vec<u8> {
        vec![0x10, 0x04, status_type]
    }

    /// Comando para imprimir y avanzar papel
    /// ESC d n
    pub fn print_and_feed(lines: u8) -> Vec<u8> {
        Self::feed_paper(lines)
    }

    /// Comando para imprimir y cortar papel
    pub fn print_and_cut() -> Vec<u8> {
        let mut output = Vec::new();
        output.extend_from_slice(&Self::feed_paper(5)); // Avanzar papel antes de cortar
        output.extend_from_slice(&Self::cut_paper_partial());
        output
    }

    /// Secuencia completa de finalización de impresión
    /// Avanza papel, hace beep y corta
    pub fn finish_printing() -> Vec<u8> {
        let mut output = Vec::new();
        output.extend_from_slice(&Self::feed_paper(3));
        output.extend_from_slice(&Self::beep());
        output.extend_from_slice(&Self::feed_paper(2));
        output.extend_from_slice(&Self::cut_paper_partial());
        output
    }

    /// Modo de página (permite posicionamiento absoluto)
    /// ESC L
    pub fn enable_page_mode() -> Vec<u8> {
        vec![0x1B, 0x4C]
    }

    /// Modo estándar (desactiva modo página)
    /// ESC S
    pub fn enable_standard_mode() -> Vec<u8> {
        vec![0x1B, 0x53]
    }

    /// Establece el área de impresión en modo página
    /// ESC W xL xH yL yH dxL dxH dyL dyH
    pub fn set_page_mode_area(x: u16, y: u16, width: u16, height: u16) -> Vec<u8> {
        vec![
            0x1B,
            0x57,
            (x & 0xFF) as u8,
            ((x >> 8) & 0xFF) as u8,
            (y & 0xFF) as u8,
            ((y >> 8) & 0xFF) as u8,
            (width & 0xFF) as u8,
            ((width >> 8) & 0xFF) as u8,
            (height & 0xFF) as u8,
            ((height >> 8) & 0xFF) as u8,
        ]
    }

    /// Imprime el contenido del buffer en modo página
    /// ESC FF
    pub fn print_page_mode() -> Vec<u8> {
        vec![0x1B, 0x0C]
    }
}