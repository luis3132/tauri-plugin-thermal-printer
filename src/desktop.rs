use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use tauri::{plugin::PluginApi, AppHandle, Runtime};
use std::time::Duration;
use std::path::Path;

use crate::models::*;
use crate::process::process_print::ProcessPrint;
use crate::drivers::{Driver, ConsoleDriver};
use crate::drivers::network::NetworkDriver;
use crate::drivers::file::FileDriver;

#[cfg(feature = "usb")]
use crate::drivers::usb::UsbDriver;

#[cfg(feature = "serial_port")]
use crate::drivers::serial::SerialPortDriver;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemPrinterInfo {
    pub name: String,
    pub is_default: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
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

  /// List system printers (using OS print system)
  pub fn list_system_printers(&self) -> crate::Result<Vec<SystemPrinterInfo>> {
    get_available_system_printers()
  }

  /// List available USB devices (requires 'usb' feature)
  #[cfg(feature = "usb")]
  pub fn list_usb_devices(&self) -> crate::Result<Vec<UsbDevice>> {
    UsbDriver::list_devices()
      .map_err(|e| crate::Error::String(format!("Failed to list USB devices: {}", e)))
  }

  /// List available serial ports (requires 'serial_port' feature)
  #[cfg(feature = "serial_port")]
  pub fn list_serial_ports(&self) -> crate::Result<Vec<String>> {
    SerialPortDriver::list_ports()
      .map_err(|e| crate::Error::String(format!("Failed to list serial ports: {}", e)))
  }

  pub fn print(&self, request: PrintJobRequest) -> crate::Result<()> {
    // Generate ESC/POS document
    let mut processor = ProcessPrint::new();
    let document = processor.generate_document(&request)
      .map_err(|e| crate::Error::String(format!("Failed to generate document: {}", e)))?;

    // Create driver based on connection config
    let driver: Box<dyn Driver> = if let Some(connection) = &request.connection {
        create_driver_from_config(connection)?
    } else {
        // Fallback to system printer
        Box::new(SystemPrinterDriver::new(&request.printer))
    };

    // Send to printer using driver
    driver.write(&document)
      .map_err(|e| crate::Error::String(format!("Failed to send to printer: {}", e)))?;
    
    driver.flush()
      .map_err(|e| crate::Error::String(format!("Failed to flush printer: {}", e)))?;

    Ok(())
  }
}

/// Create a driver from connection configuration
fn create_driver_from_config(config: &ConnectionConfig) -> crate::Result<Box<dyn Driver>> {
    match config {
        ConnectionConfig::System { printer_name } => {
            Ok(Box::new(SystemPrinterDriver::new(printer_name)))
        }
        ConnectionConfig::Network { host, port, timeout_secs } => {
            let driver = NetworkDriver::new(host, *port, Some(Duration::from_secs(*timeout_secs)))
                .map_err(|e| crate::Error::String(format!("Failed to create network driver: {}", e)))?;
            Ok(Box::new(driver))
        }
        #[cfg(feature = "usb")]
        ConnectionConfig::Usb { vendor_id, product_id, timeout_secs } => {
            let driver = UsbDriver::new(*vendor_id, *product_id, Some(Duration::from_secs(*timeout_secs)))
                .map_err(|e| crate::Error::String(format!("Failed to create USB driver: {}", e)))?;
            Ok(Box::new(driver))
        }
        #[cfg(feature = "serial_port")]
        ConnectionConfig::Serial { port, baud_rate, timeout_secs } => {
            let driver = SerialPortDriver::new(port, *baud_rate, Some(Duration::from_secs(*timeout_secs)))
                .map_err(|e| crate::Error::String(format!("Failed to create serial driver: {}", e)))?;
            Ok(Box::new(driver))
        }
        ConnectionConfig::File { path } => {
            let driver = FileDriver::new(Path::new(path))
                .map_err(|e| crate::Error::String(format!("Failed to create file driver: {}", e)))?;
            Ok(Box::new(driver))
        }
        ConnectionConfig::Console { show_output } => {
            Ok(Box::new(ConsoleDriver::new(*show_output)))
        }
    }
}

/// System printer driver using OS commands
struct SystemPrinterDriver {
    printer_name: String,
}

impl SystemPrinterDriver {
    fn new(printer_name: &str) -> Self {
        Self {
            printer_name: printer_name.to_string(),
        }
    }
}

impl Driver for SystemPrinterDriver {
    fn name(&self) -> String {
        format!("system ({})", self.printer_name)
    }

    fn write(&self, data: &[u8]) -> Result<(), crate::drivers::DriverError> {
        send_to_system_printer(&self.printer_name, data)
            .map_err(|e| crate::drivers::DriverError::Io(e))
    }

    fn read(&self, _buf: &mut [u8]) -> Result<usize, crate::drivers::DriverError> {
        Ok(0)
    }

    fn flush(&self) -> Result<(), crate::drivers::DriverError> {
        Ok(())
    }
}

// Platform-specific implementations

#[cfg(target_os = "linux")]
fn get_available_system_printers() -> crate::Result<Vec<SystemPrinterInfo>> {
    use printers::get_printers;
    
    let printers_list = get_printers();
    let printer_list = printers_list
        .iter()
        .map(|p| SystemPrinterInfo {
            name: p.name.clone(),
            is_default: false,
            location: None,
        })
        .collect();
    
    Ok(printer_list)
}

#[cfg(target_os = "linux")]
fn send_to_system_printer(printer_name: &str, data: &[u8]) -> Result<(), String> {
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
fn get_available_system_printers() -> crate::Result<Vec<SystemPrinterInfo>> {
    use printers::get_printers;
    
    let printers_list = get_printers();
    let printer_list = printers_list
        .iter()
        .map(|p| SystemPrinterInfo {
            name: p.name.clone(),
            is_default: false,
            location: None,
        })
        .collect();
    
    Ok(printer_list)
}

#[cfg(target_os = "macos")]
fn send_to_system_printer(printer_name: &str, data: &[u8]) -> Result<(), String> {
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
fn get_available_system_printers() -> crate::Result<Vec<SystemPrinterInfo>> {
    Err(crate::Error::String("Windows printer enumeration not yet implemented. Please use printer name directly or specify connection config.".to_string()))
}

#[cfg(target_os = "windows")]
fn send_to_system_printer(printer_name: &str, data: &[u8]) -> Result<(), String> {
    use std::fs::OpenOptions;
    use std::io::Write;
    
    let printer_path = if printer_name.starts_with("\\\\") {
        printer_name.to_string()
    } else {
        format!("\\\\.\\{}", printer_name)
    };
    
    let mut file = OpenOptions::new()
        .write(true)
        .open(&printer_path)
        .map_err(|e| format!("Failed to open printer '{}': {}. Try using the full printer path or connection config.", printer_path, e))?;
    
    file.write_all(data)
        .map_err(|e| format!("Failed to write to printer: {}", e))?;
    
    file.flush()
        .map_err(|e| format!("Failed to flush printer data: {}", e))?;
    
    Ok(())
}

#[cfg(not(any(target_os = "linux", target_os = "macos", target_os = "windows")))]
fn get_available_system_printers() -> crate::Result<Vec<SystemPrinterInfo>> {
    Err(crate::Error::String("Unsupported platform".to_string()))
}

#[cfg(not(any(target_os = "linux", target_os = "macos", target_os = "windows")))]
fn send_to_system_printer(_printer_name: &str, _data: &[u8]) -> Result<(), String> {
    Err("Unsupported platform".to_string())
}
