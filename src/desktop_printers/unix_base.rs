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
                .map(|(interface_type, _identifier)| PrinterInfo {
                    name: name.clone(),
                    interface_type: interface_type.clone(),
                    // SUR LINUX: L'identifier DOIT être le nom pour la commande 'lp -d'
                    identifier: name.clone(),
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
        .filter_map(parse_printer_status)
        .collect();

    Ok(statuses)
}

fn get_printer_devices() -> Result<HashMap<String, (String, String)>> {
    let stdout = lpstat(&["-v"])?;

    let devices = stdout
        .lines()
        .filter_map(parse_device_line)
        .collect();

    Ok(devices)
}

fn parse_printer_status(line: &str) -> Option<(String, String)> {
    let (name, status_part) = if line.contains(" is ") {
        let parts: Vec<&str> = line.split(" is ").collect();
        let name = parts[0].split_whitespace().last()?;
        (name, parts[1])
    } else if line.contains(" est ") {
        let parts: Vec<&str> = line.split(" est ").collect();
        let name = parts[0].split_whitespace().last()?;
        (name, parts[1])
    } else {
        return None;
    };

    let status = match () {
        _ if status_part.contains("idle") || status_part.contains("libre") => "idle",
        _ if status_part.contains("printing") || status_part.contains("occupé") => "printing",
        _ if status_part.contains("disabled") || status_part.contains("désactivé") => "disabled",
        _ => "unknown",
    };

    Some((name.trim().to_string(), status.to_string()))
}

fn parse_device_line(line: &str) -> Option<(String, (String, String))> {
    let separator = if line.contains(" : ") { " : " } else { ": " };
    let (prefix_and_name, uri) = line.split_once(separator)?;
    let name = prefix_and_name.split_whitespace().last()?;
    let (interface_type, identifier) = uri.split_once(':').unwrap_or(("unknown", uri));

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
