use crate::commands_esc_pos::text::code_page::CodePage;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrinterOptions {
    pub cut_paper: bool,
    pub beep: bool,
    pub open_cash_drawer: bool,
    /// Página de código para la impresora. Define qué idioma/caracteres se usarán.
    /// Por defecto: `CodePage::Page(0)` (CP437, ASCII puro).
    #[serde(default)]
    pub code_page: CodePage,
}

impl Default for PrinterOptions {
    fn default() -> Self {
        Self {
            cut_paper: true,
            beep: false,
            open_cash_drawer: false,
            code_page: CodePage::default(),
        }
    }
}

impl PrinterOptions {
    pub fn assign(&mut self, cut_paper: bool, beep: bool, open_cash_drawer: bool) {
        self.cut_paper = cut_paper;
        self.beep = beep;
        self.open_cash_drawer = open_cash_drawer;
    }
}
