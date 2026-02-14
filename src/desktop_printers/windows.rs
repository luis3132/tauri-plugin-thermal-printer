use serde::Deserialize;
use std::ffi::OsStr;
use std::os::windows::ffi::OsStrExt;
use std::ptr;
use winapi::shared::minwindef::{DWORD, LPVOID};
use winapi::um::winspool::DOC_INFO_1W;
use winapi::um::winspool::{
    ClosePrinter, EndDocPrinter, EndPagePrinter, OpenPrinterW, StartDocPrinterW, StartPagePrinter,
    WritePrinter,
};

use crate::PrinterInfo;
use std::process::Command;

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
    let mut command = Command::new("powershell");
    command.args(&[
        "-NoProfile",
        "-WindowStyle",
        "Hidden",
        "-Command",
        "Get-Printer | Select-Object Name, InterfaceType, PortName, PrinterStatus | ConvertTo-Json",
    ]);

    println!("Querying installed printers via PowerShell");
    println!("Running command: powershell {:?}", command);

    let output = command.output()?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        eprintln!("PowerShell returned non-zero exit: {}", stderr);
        return Err(format!("PowerShell error: {}", stderr).into());
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    println!("PowerShell stdout: {}", stdout.trim());

    // PowerShell devuelve un objeto sin [] si solo hay una impresora
    let ps_printers: Vec<PowerShellPrinter> = if stdout.trim().starts_with('[') {
        serde_json::from_str(&stdout)?
    } else if stdout.trim().is_empty() {
        Vec::new()
    } else {
        vec![serde_json::from_str(&stdout)?]
    };

    let printers: Vec<PrinterInfo> = ps_printers.into_iter().map(PrinterInfo::from).collect();

    println!("Found {} printers", printers.len());
    println!("Printers info: {:#?}", printers);

    Ok(printers)
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
