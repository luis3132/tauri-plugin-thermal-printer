#[derive(Debug, Clone)] // Permite imprimir y copiar el objeto fácilmente
pub struct PrinterOptions {
    pub cut_paper: bool,
    pub beep: bool,
    pub open_cash_drawer: bool,
}

pub impl Default for PrinterOptions {
    fn default() -> Self {
        Self {
            cut_paper: true,
            beep: false,
            open_cash_drawer: false,
        }
    }
    fn assign(&mut self, cut_paper: bool, beep: bool, open_cash_drawer: bool) {
        self.cut_paper = cut_paper;
        self.beep = beep;
        self.open_cash_drawer = open_cash_drawer;
    }
}