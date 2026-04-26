use crate::error::Result;
use crate::models::print_job_request::PrinterInfo;
use std::collections::HashMap;
use std::io::Write;
use std::process::Command;
use std::process::Stdio;

fn lpstat(args: &[&str]) -> std::io::Result<String> {
    let output = Command::new("lpstat")
        .args(args)
        .env("LANG", "C")
        .env("LC_ALL", "C")
        .output()?;

    String::from_utf8(output.stdout)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))
}

pub fn get_printers_info() -> Result<Vec<PrinterInfo>> {
    let statuses = get_printer_statuses()?;
    let devices = get_printer_devices()?;

    let printers = statuses
        .into_iter()
        .filter_map(|(name, status)| {
            devices
                .get(&name)
                .map(|(interface_type, identifier)| PrinterInfo {
                    name,
                    interface_type: interface_type.clone(),
                    identifier: identifier.clone(),
                    status,
                })
        })
        .collect();

    Ok(printers)
}

fn get_printer_statuses() -> Result<HashMap<String, String>> {
    let stdout = lpstat(&["-p"])?;

    let statuses = stdout
        .lines()
        .filter(|line| line.starts_with("printer "))
        .filter_map(parse_printer_status)
        .collect();

    Ok(statuses)
}

fn get_printer_devices() -> Result<HashMap<String, (String, String)>> {
    let stdout = lpstat(&["-v"])?;

    let devices = stdout
        .lines()
        .filter(|line| line.starts_with("device for "))
        .filter_map(parse_device_line)
        .collect();

    Ok(devices)
}

fn parse_printer_status(line: &str) -> Option<(String, String)> {
    // "printer <name> is idle/printing/disabled ..."
    let rest = line.strip_prefix("printer ")?;
    let (name, status_part) = rest.split_once(" is ")?;

    let status = match () {
        _ if status_part.contains("idle") => "idle",
        _ if status_part.contains("printing") => "printing",
        _ if status_part.contains("disabled") => "disabled",
        _ => "unknown",
    };

    Some((name.trim().to_string(), status.to_string()))
}

fn parse_device_line(line: &str) -> Option<(String, (String, String))> {
    // "device for <name>: <interface>:<identifier>"
    let rest = line.strip_prefix("device for ")?;
    let (name, device) = rest.split_once(": ")?;
    let (interface_type, identifier) = device.split_once(':')?;

    Some((
        name.trim().to_string(),
        (
            interface_type.trim().to_string(),
            identifier.trim().to_string(),
        ),
    ))
}

pub fn print_raw_data(printer_name: &str, data: &[u8]) -> std::io::Result<()> {
    let mut child = Command::new("lp")
        .args(["-d", printer_name, "-o", "raw"])
        .stdin(Stdio::piped())
        .spawn()?;

    child
        .stdin
        .take()
        .ok_or_else(|| std::io::Error::new(std::io::ErrorKind::BrokenPipe, "Failed to open stdin"))?
        .write_all(data)?;

    let status = child.wait()?;

    status.success().then_some(()).ok_or_else(|| {
        std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("lp command failed with status: {}", status),
        )
    })
}
