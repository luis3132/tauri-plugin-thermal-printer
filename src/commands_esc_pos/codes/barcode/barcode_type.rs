/// Tipos de códigos de barras soportados
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BarcodeType {
    UpcA = 65,    // UPC-A
    UpcE = 66,    // UPC-E
    Ean13 = 67,   // EAN13
    Ean8 = 68,    // EAN8
    Code39 = 69,  // CODE39
    Itf = 70,     // ITF (Interleaved 2 of 5)
    Codabar = 71, // CODABAR
    Code93 = 72,  // CODE93
    Code128 = 73, // CODE128
    // NOTA: las variantes GS1 (74-78) requieren soporte de firmware; no todas
    // las impresoras las implementan.
    Gs1128 = 74,             // GS1-128 (EAN-128) — CODE128 con FNC1
    Gs1DatabarOmni = 75,     // GS1 DataBar Omnidirectional
    Gs1DatabarTruncated = 76, // GS1 DataBar Truncated
    Gs1DatabarLimited = 77,  // GS1 DataBar Limited
    Gs1DatabarExpanded = 78, // GS1 DataBar Expanded
}

impl BarcodeType {
    /// Obtiene el valor numérico del tipo de código de barras
    pub fn value(&self) -> u8 {
        *self as u8
    }

    /// Returns true if this barcode type only accepts numeric digits as data
    ///
    /// Nota: GS1-128 y GS1 DataBar Expanded admiten formato GS1 con AIs
    /// (paréntesis / FNC1), por lo que NO se marcan como numeric-only.
    pub fn requires_numeric_data(&self) -> bool {
        matches!(
            self,
            BarcodeType::UpcA
                | BarcodeType::UpcE
                | BarcodeType::Ean13
                | BarcodeType::Ean8
                | BarcodeType::Itf
                | BarcodeType::Gs1DatabarOmni
                | BarcodeType::Gs1DatabarTruncated
                | BarcodeType::Gs1DatabarLimited
        )
    }
}
