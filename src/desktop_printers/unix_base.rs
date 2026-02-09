use std::process::Command;
use std::collections::HashMap;
use std::process::Stdio;
use std::io::Write;
use crate::models::print_job_request::PrinterInfo;
use crate::error::Result;

pub fn get_printers_info() -> Result<Vec<PrinterInfo>> {
    let output = Command::new("lpstat").arg("-t").output()?;
    let stdout = String::from_utf8(output.stdout).map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;

    let mut printers = Vec::new();
    let mut devices = HashMap::new();

    for line in stdout.lines() {
        if line.starts_with("device for ") {
            // device for POS-80: parallel:/dev/usb/lp0
            let parts: Vec<&str> = line.split(": ").collect();
            if parts.len() == 2 {
                let name = parts[0].strip_prefix("device for ").unwrap();
                let device = parts[1];
                let device_parts: Vec<&str> = device.split(':').collect();
                if device_parts.len() == 2 {
                    let interface_type = device_parts[0].to_string();
                    let identifier = device_parts[1].to_string();
                    devices.insert(name.to_string(), (interface_type, identifier));
                }
            }
        } else if line.starts_with("printer ") && line.contains(" is ") {
            // printer POS-80 is idle.  enabled since Tue 09 Dec 2025 07:33:02 -05
            let parts: Vec<&str> = line.split(" is ").collect();
            if parts.len() == 2 {
                let name = parts[0].strip_prefix("printer ").unwrap();
                let status_part = parts[1];
                let status = if status_part.contains("idle") {
                    "idle"
                } else if status_part.contains("printing") {
                    "printing"
                } else {
                    "unknown"
                };
                if let Some((interface_type, identifier)) = devices.get(name) {
                    printers.push(PrinterInfo {
                        name: name.to_string(),
                        interface_type: interface_type.clone(),
                        identifier: identifier.clone(),
                        status: status.to_string(),
                    });
                }
            }
        }
    }

    Ok(printers)
}

pub fn print_raw_data(printer_name: &str, data: &[u8]) -> std::io::Result<()> {
    let mut child = Command::new("lp")
        .arg("-d")
        .arg(printer_name)
        .arg("-o")
        .arg("raw")
        .stdin(Stdio::piped())
        .spawn()?;

    let mut stdin = child.stdin.take().expect("Failed to open stdin");
    stdin.write_all(data)?;
    drop(stdin);

    let status = child.wait()?;
    if status.success() {
        Ok(())
    } else {
        Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("lp command failed with status: {}", status),
        ))
    }
}
