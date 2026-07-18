/// Subtipos de GS1 DataBar bidimensional (parámetro `m` de la función 80).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Gs1Databar2dType {
    Stacked = 72,          // GS1 DataBar Stacked
    StackedOmni = 73,      // GS1 DataBar Stacked Omnidirectional
    ExpandedStacked = 76,  // GS1 DataBar Expanded Stacked
}

impl Gs1Databar2dType {
    /// Valor numérico del subtipo (m).
    pub fn value(&self) -> u8 {
        *self as u8
    }
}
