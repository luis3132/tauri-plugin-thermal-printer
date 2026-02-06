use super::data_matrix_size::DataMatrixSize;

/// Constructor de comandos para códigos DataMatrix
/// 
/// NOTA: DataMatrix no es soportado por todas las impresoras térmicas.
/// Funciona principalmente en modelos Epson TM-T88VI y superiores.
#[derive(Debug, Clone)]
pub struct DataMatrix {
    data: String,
    size: DataMatrixSize,
}

impl DataMatrix {
    /// Crea un nuevo código DataMatrix con valores por defecto
    ///
    /// # Arguments
    /// * `data` - Datos a codificar en el DataMatrix
    pub fn new(data: String) -> Self {
        Self {
            data,
            size: DataMatrixSize::Size6,
        }
    }

    /// Establece el tamaño del módulo del código DataMatrix
    ///
    /// # Arguments
    /// * `size` - Tamaño del módulo (1-16)
    pub fn set_size(mut self, size: DataMatrixSize) -> Self {
        self.size = size;
        self
    }

    /// Genera el comando ESC/POS para DataMatrix
    /// 
    /// NOTA: DataMatrix no es soportado por todas las impresoras térmicas.
    /// Funciona principalmente en modelos Epson TM-T88VI y superiores.
    pub fn get_command(&self) -> Vec<u8> {
        let mut output = Vec::new();
        let data_bytes = self.data.as_bytes();
        let data_length = (data_bytes.len() + 3) as u16;
        let p_l = (data_length & 0xFF) as u8;
        let p_h = ((data_length >> 8) & 0xFF) as u8;

        // DataMatrix usa cn = 50 (0x32)

        // Función 67 (0x43) - Establecer tamaño del módulo
        output.extend_from_slice(&[
            0x1D, // GS
            0x28, // (
            0x6B, // k
            0x03, // pL
            0x00, // pH
            0x32, // cn = 50 (0x32 = DataMatrix)
            0x43, // fn = 67 (0x43 = tamaño de módulo)
            self.size.value(), // n (1-16)
        ]);

        // Función 80 (0x50) - Almacenar datos en el buffer
        output.extend_from_slice(&[
            0x1D, // GS
            0x28, // (
            0x6B, // k
            p_l,  // pL (parte baja de longitud)
            p_h,  // pH (parte alta de longitud)
            0x32, // cn = 50
            0x50, // fn = 80 (0x50 = almacenar datos)
            0x30, // m = 48 (tipo de almacenamiento)
        ]);
        output.extend_from_slice(data_bytes); // datos

        // Función 81 (0x51) - Imprimir el símbolo DataMatrix
        output.extend_from_slice(&[
            0x1D, // GS
            0x28, // (
            0x6B, // k
            0x03, // pL
            0x00, // pH
            0x32, // cn = 50
            0x51, // fn = 81 (0x51 = imprimir)
            0x30, // m = 48
        ]);

        output
    }

    // /// Verifica si la impresora soporta DataMatrix
    // /// Retorna un comando de consulta (no todas las impresoras responden)
    // pub fn check_support() -> Vec<u8> {
    //     vec![
    //         0x1D, // GS
    //         0x28, // (
    //         0x6B, // k
    //         0x02, // pL
    //         0x00, // pH
    //         0x32, // cn = 50
    //         0x40, // fn = 64 (consultar)
    //     ]
    // }
}