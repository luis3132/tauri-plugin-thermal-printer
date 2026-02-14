use serde::Deserialize;

use crate::PrinterInfo;
use std::{io::Write, process::{Command}};

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
        let interface_type = ps_printer.InterfaceType
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
        }.to_string();

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
        "Get-Printer | Select-Object Name, InterfaceType, PortName, PrinterStatus | ConvertTo-Json"
    ]);

    let output = command.output()?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(format!("PowerShell error: {}", stderr).into());
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    
    // PowerShell devuelve un objeto sin [] si solo hay una impresora
    let ps_printers: Vec<PowerShellPrinter> = if stdout.trim().starts_with('[') {
        serde_json::from_str(&stdout)?
    } else if stdout.trim().is_empty() {
        Vec::new()
    } else {
        vec![serde_json::from_str(&stdout)?]
    };
    
    let printers: Vec<PrinterInfo> = ps_printers
        .into_iter()
        .map(PrinterInfo::from)
        .collect();
    
    Ok(printers)
}

pub fn print_raw_data_win(printer_name: &str, data: &[u8]) -> std::io::Result<()> {
    use std::fs::OpenOptions;
    
    // En Windows, podemos escribir directamente al dispositivo de la impresora
    // usando su nombre UNC: \\.\<printer_name>
    let printer_path = format!(r"\\.\{}", printer_name);
    
    // Abrir el dispositivo de la impresora en modo escritura binaria
    let mut printer = OpenOptions::new()
        .write(true)
        .create(false)
        .open(&printer_path)?;
    
    // Escribir los bytes crudos directamente
    printer.write_all(data)?;
    
    Ok(())
}