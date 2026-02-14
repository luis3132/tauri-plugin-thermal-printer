use serde::Deserialize;
use std::ffi::OsStr;
use std::os::windows::ffi::OsStrExt;
use std::ptr;
use winapi::shared::minwindef::{DWORD, LPVOID};
use winapi::um::winspool::DOC_INFO_1W;
use winapi::um::winspool::{
    ClosePrinter, EndDocPrinter, EndPagePrinter, OpenPrinterW, StartDocPrinterW, StartPagePrinter,
    WritePrinter, EnumPrintersW, PRINTER_INFO_2W, PRINTER_ENUM_LOCAL,
};
use winapi::um::winnt::LPWSTR;

use crate::PrinterInfo;

// Struct intermedia para deserializar el JSON de PowerShell
#[allow(non_snake_case)]
#[derive(Debug, Deserialize)]
struct PowerShellPrinter {
    Name: String,
    PortName: String,
    PrinterStatus: u32,
    InterfaceType: Option<String>,
}

impl From<PowerShellPrinter> for PrinterInfo {
    fn from(ps_printer: PowerShellPrinter) -> Self {
        // Usar InterfaceType si está disponible, sino parsear del PortName
        let interface_type = ps_printer
            .InterfaceType
            .unwrap_or_else(|| get_interface_type(&ps_printer.PortName));

        // Mapear el status numérico a string
        let status = match ps_printer.PrinterStatus {
            0 => "Other",
            1 => "Unknown",
            2 => "Idle",
            3 => "Printing",
            4 => "Warmup",
            5 => "Stopped",
            6 => "Offline",
            _ => "Unknown",
        }
        .to_string();

        PrinterInfo {
            name: ps_printer.Name,
            interface_type,
            identifier: ps_printer.PortName,
            status,
        }
    }
}

fn get_interface_type(port_name: &str) -> String {
    if port_name.starts_with("IP_") || port_name.contains(':') || port_name.starts_with("WSD") {
        "Network".to_string()
    } else if port_name.starts_with("USB") {
        "USB".to_string()
    } else if port_name.starts_with("LPT") {
        "Parallel".to_string()
    } else if port_name.starts_with("COM") {
        "Serial".to_string()
    } else if port_name.eq_ignore_ascii_case("FILE:") {
        "File".to_string()
    } else {
        "Other".to_string()
    }
}

pub fn get_printers_info_win() -> Result<Vec<PrinterInfo>, Box<dyn std::error::Error>> {
    let mut printers = Vec::new();

    unsafe {
        let mut needed: DWORD = 0;
        let mut returned: DWORD = 0;

        // Primera llamada para obtener el tamaño necesario
        EnumPrintersW(
            PRINTER_ENUM_LOCAL,
            ptr::null_mut(),
            2, // PRINTER_INFO_2
            ptr::null_mut(),
            0,
            &mut needed,
            &mut returned,
        );

        if needed == 0 {
            return Ok(printers);
        }

        let mut buffer: Vec<u8> = vec![0; needed as usize];

        let result = EnumPrintersW(
            PRINTER_ENUM_LOCAL,
            ptr::null_mut(),
            2,
            buffer.as_mut_ptr(),
            needed,
            &mut needed,
            &mut returned,
        );

        if result == 0 {
            return Err(std::io::Error::last_os_error().into());
        }

        let printer_info_array = buffer.as_ptr() as *const PRINTER_INFO_2W;

        for i in 0..returned {
            let printer_info = *printer_info_array.offset(i as isize);

            let name = wide_to_string(printer_info.pPrinterName);
            let port_name = wide_to_string(printer_info.pPortName);

            let interface_type = get_interface_type(&port_name);

            let status = match printer_info.Status {
                0 => "Idle",
                winapi::um::winspool::PRINTER_STATUS_BUSY => "Printing",
                winapi::um::winspool::PRINTER_STATUS_DOOR_OPEN => "Offline",
                winapi::um::winspool::PRINTER_STATUS_ERROR => "Error",
                winapi::um::winspool::PRINTER_STATUS_INITIALIZING => "Initializing",
                winapi::um::winspool::PRINTER_STATUS_IO_ACTIVE => "IO Active",
                winapi::um::winspool::PRINTER_STATUS_MANUAL_FEED => "Manual Feed",
                winapi::um::winspool::PRINTER_STATUS_NO_TONER => "No Toner",
                winapi::um::winspool::PRINTER_STATUS_NOT_AVAILABLE => "Not Available",
                winapi::um::winspool::PRINTER_STATUS_OFFLINE => "Offline",
                winapi::um::winspool::PRINTER_STATUS_OUT_OF_MEMORY => "Out of Memory",
                winapi::um::winspool::PRINTER_STATUS_OUTPUT_BIN_FULL => "Output Bin Full",
                winapi::um::winspool::PRINTER_STATUS_PAGE_PUNT => "Page Punt",
                winapi::um::winspool::PRINTER_STATUS_PAPER_JAM => "Paper Jam",
                winapi::um::winspool::PRINTER_STATUS_PAPER_OUT => "Paper Out",
                winapi::um::winspool::PRINTER_STATUS_PAPER_PROBLEM => "Paper Problem",
                winapi::um::winspool::PRINTER_STATUS_PAUSED => "Paused",
                winapi::um::winspool::PRINTER_STATUS_PENDING_DELETION => "Pending Deletion",
                winapi::um::winspool::PRINTER_STATUS_PRINTING => "Printing",
                winapi::um::winspool::PRINTER_STATUS_PROCESSING => "Processing",
                winapi::um::winspool::PRINTER_STATUS_SERVER_UNKNOWN => "Server Unknown",
                winapi::um::winspool::PRINTER_STATUS_TONER_LOW => "Toner Low",
                winapi::um::winspool::PRINTER_STATUS_USER_INTERVENTION => "User Intervention",
                winapi::um::winspool::PRINTER_STATUS_WAITING => "Waiting",
                winapi::um::winspool::PRINTER_STATUS_WARMING_UP => "Warming Up",
                _ => "Unknown",
            }.to_string();

            printers.push(PrinterInfo {
                name,
                interface_type,
                identifier: port_name,
                status,
            });
        }
    }

    println!("Found {} printers", printers.len());
    println!("Printers info: {:#?}", printers);

    Ok(printers)
}

fn wide_to_string(wide: LPWSTR) -> String {
    if wide.is_null() {
        return String::new();
    }
    unsafe {
        let len = (0..).take_while(|&i| *wide.offset(i) != 0).count();
        let slice = std::slice::from_raw_parts(wide, len);
        String::from_utf16_lossy(slice)
    }
}

pub fn print_raw_data_win(printer_name: &str, data: &[u8]) -> std::io::Result<()> {
    println!(
        "Sending raw data to printer '{}' ({} bytes)",
        printer_name,
        data.len()
    );

    // Convertir el nombre de la impresora a UTF-16
    let printer_name_wide: Vec<u16> = OsStr::new(printer_name)
        .encode_wide()
        .chain(std::iter::once(0))
        .collect();

    // Handle de la impresora
    let mut h_printer: LPVOID = ptr::null_mut();

    unsafe {
        // Abrir la impresora
        let result = OpenPrinterW(
            printer_name_wide.as_ptr() as *mut _,
            &mut h_printer,
            ptr::null_mut(),
        );

        if result == 0 {
            eprintln!("Error opening printer '{}'", printer_name);
            return Err(std::io::Error::last_os_error());
        }

        println!("Opened printer '{}'", printer_name);

        // Preparar información del documento
        let doc_name: Vec<u16> = OsStr::new("Raw Print Job")
            .encode_wide()
            .chain(std::iter::once(0))
            .collect();

        let data_type: Vec<u16> = OsStr::new("RAW")
            .encode_wide()
            .chain(std::iter::once(0))
            .collect();

        let mut doc_info = DOC_INFO_1W {
            pDocName: doc_name.as_ptr() as *mut _,
            pOutputFile: ptr::null_mut(),
            pDatatype: data_type.as_ptr() as *mut _,
        };

        // Iniciar el documento
        let job_id = StartDocPrinterW(h_printer, 1, &mut doc_info as *mut _ as *mut _);
        if job_id == 0 {
            eprintln!("Error starting document on printer '{}'", printer_name);
            ClosePrinter(h_printer);
            return Err(std::io::Error::last_os_error());
        }

        println!("Started print job {}", job_id);

        // Iniciar página
        let page_result = StartPagePrinter(h_printer);
        if page_result == 0 {
            eprintln!("Error starting page on printer '{}'", printer_name);
            EndDocPrinter(h_printer);
            ClosePrinter(h_printer);
            return Err(std::io::Error::last_os_error());
        }

        // Escribir los datos
        let mut bytes_written: DWORD = 0;
        let write_result = WritePrinter(
            h_printer,
            data.as_ptr() as LPVOID,
            data.len() as DWORD,
            &mut bytes_written,
        );

        if write_result == 0 {
            eprintln!("Error writing to printer '{}'", printer_name);
            EndPagePrinter(h_printer);
            EndDocPrinter(h_printer);
            ClosePrinter(h_printer);
            return Err(std::io::Error::last_os_error());
        }

        println!(
            "Wrote {} bytes to printer '{}'",
            bytes_written, printer_name
        );

        // Finalizar página
        EndPagePrinter(h_printer);

        // Finalizar documento
        EndDocPrinter(h_printer);

        // Cerrar la impresora
        ClosePrinter(h_printer);

        println!("Successfully completed print job on '{}'", printer_name);
        Ok(())
    }
}
