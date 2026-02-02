use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use tauri::{plugin::PluginApi, AppHandle, Runtime};

use crate::models::*;
use crate::process::process_print::ProcessPrint;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrinterInfo {
    pub name: String,
    pub is_default: bool,
}

pub fn init<R: Runtime, C: DeserializeOwned>(
  app: &AppHandle<R>,
  _api: PluginApi<R, C>,
) -> crate::Result<ThermalPrinter<R>> {
  Ok(ThermalPrinter(app.clone()))
}

/// Access to the thermal-printer APIs.
pub struct ThermalPrinter<R: Runtime>(AppHandle<R>);

impl<R: Runtime> ThermalPrinter<R> {
  pub fn ping(&self, payload: PingRequest) -> crate::Result<PingResponse> {
    Ok(PingResponse {
      value: payload.value,
    })
  }

  pub fn list_printers(&self) -> crate::Result<Vec<PrinterInfo>> {
    get_available_printers()
  }

  pub fn print(&self, request: PrintJobRequest) -> crate::Result<()> {
    // Generar documento ESC/POS
    let mut processor = ProcessPrint::new();
    let document = processor.generate_document(&request)
      .map_err(|e| crate::Error::String(format!("Failed to generate document: {}", e)))?;

    // Enviar a la impresora
    send_to_printer(&request.printer, &document)
      .map_err(|e| crate::Error::String(format!("Failed to send to printer: {}", e)))?;

    Ok(())
  }
}

// Implementaciones específicas por plataforma

#[cfg(target_os = "linux")]
fn get_available_printers() -> crate::Result<Vec<PrinterInfo>> {
    use printers::get_printers;
    
    let printers_list = get_printers();
    let printer_list = printers_list
        .iter()
        .map(|p| PrinterInfo {
            name: p.name.clone(),
            is_default: false, // printers crate no indica cuál es default en Linux
        })
        .collect();
    
    Ok(printer_list)
}

#[cfg(target_os = "linux")]
fn send_to_printer(printer_name: &str, data: &[u8]) -> Result<(), String> {
    // En Linux, enviamos directamente al dispositivo usando lp command
    use std::process::Command;
    use std::io::Write;
    
    let mut child = Command::new("lp")
        .arg("-d")
        .arg(printer_name)
        .arg("-")
        .stdin(std::process::Stdio::piped())
        .spawn()
        .map_err(|e| format!("Failed to spawn lp command: {}", e))?;
    
    if let Some(mut stdin) = child.stdin.take() {
        stdin.write_all(data)
            .map_err(|e| format!("Failed to write to lp stdin: {}", e))?;
    }
    
    let status = child.wait()
        .map_err(|e| format!("Failed to wait for lp command: {}", e))?;
    
    if !status.success() {
        return Err(format!("lp command failed with status: {}", status));
    }
    
    Ok(())
}

#[cfg(target_os = "macos")]
fn get_available_printers() -> crate::Result<Vec<PrinterInfo>> {
    use printers::get_printers;
    
    let printers_list = get_printers();
    let printer_list = printers_list
        .iter()
        .map(|p| PrinterInfo {
            name: p.name.clone(),
            is_default: false,
        })
        .collect();
    
    Ok(printer_list)
}

#[cfg(target_os = "macos")]
fn send_to_printer(printer_name: &str, data: &[u8]) -> Result<(), String> {
    // En macOS, también usamos lp command
    use std::process::Command;
    use std::io::Write;
    
    let mut child = Command::new("lp")
        .arg("-d")
        .arg(printer_name)
        .arg("-")
        .stdin(std::process::Stdio::piped())
        .spawn()
        .map_err(|e| format!("Failed to spawn lp command: {}", e))?;
    
    if let Some(mut stdin) = child.stdin.take() {
        stdin.write_all(data)
            .map_err(|e| format!("Failed to write to lp stdin: {}", e))?;
    }
    
    let status = child.wait()
        .map_err(|e| format!("Failed to wait for lp command: {}", e))?;
    
    if !status.success() {
        return Err(format!("lp command failed with status: {}", status));
    }
    
    Ok(())
}

#[cfg(target_os = "windows")]
fn get_available_printers() -> crate::Result<Vec<PrinterInfo>> {
    // Para Windows, necesitaremos usar WinAPI o una alternativa simple
    // Por ahora, retornamos un error indicando que debe implementarse
    Err(crate::Error::String("Windows printer enumeration not yet implemented. Please use printer name directly.".to_string()))
}

#[cfg(target_os = "windows")]
fn send_to_printer(printer_name: &str, data: &[u8]) -> Result<(), String> {
    // En Windows, escribimos directamente al puerto de impresora
    use std::fs::OpenOptions;
    use std::io::Write;
    
    // Intentar abrir como dispositivo de red (\\\\computername\\printername)
    // o como puerto local
    let printer_path = if printer_name.starts_with("\\\\") {
        printer_name.to_string()
    } else {
        format!("\\\\.\\{}", printer_name)
    };
    
    let mut file = OpenOptions::new()
        .write(true)
        .open(&printer_path)
        .map_err(|e| format!("Failed to open printer '{}': {}. Try using the full printer path.", printer_path, e))?;
    
    file.write_all(data)
        .map_err(|e| format!("Failed to write to printer: {}", e))?;
    
    file.flush()
        .map_err(|e| format!("Failed to flush printer data: {}", e))?;
    
    Ok(())
}

#[cfg(not(any(target_os = "linux", target_os = "macos", target_os = "windows")))]
fn get_available_printers() -> crate::Result<Vec<PrinterInfo>> {
    Err(crate::Error::String("Unsupported platform".to_string()))
}

#[cfg(not(any(target_os = "linux", target_os = "macos", target_os = "windows")))]
fn send_to_printer(_printer_name: &str, _data: &[u8]) -> Result<(), String> {
    Err("Unsupported platform".to_string())
}
