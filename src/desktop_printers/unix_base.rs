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
        .filter_map(|line| {
            let words: Vec<&str> = line.split_whitespace().collect();
            if words.len() < 3 { return None; }
            
            let name = words[1].to_string();
            let status_word = words[2].to_lowercase();
            
            let status = match () {
                _ if status_word.contains("idle") || status_word.contains("libre") || status_word.contains("attente") => "idle",
                _ if status_word.contains("print") || status_word.contains("occup") || status_word.contains("activ") => "printing",
                _ if status_word.contains("disab") || status_word.contains("stopp") || status_word.contains("désact") => "disabled",
                _ => "unknown",
            };
            
            Some((name, status.to_string()))
        })
        .collect();

    Ok(statuses)
}

fn get_printer_devices() -> Result<HashMap<String, (String, String)>> {
    let stdout = lpstat(&["-v"])?;

    let devices = stdout
        .lines()
        .filter_map(|line| {
            let separator = if line.contains(" : ") { " : " } else { ":" };
            let parts: Vec<&str> = line.split(separator).collect();
            if parts.len() < 2 { return None; }
            
            let prefix_part = parts[0].trim();
            let uri_part = parts[1].trim();
            
            let name = prefix_part.split_whitespace().last()?.to_string();
            
            let interface_type = uri_part.split_once("://")
                .map(|(proto, _)| proto)
                .unwrap_or("unknown")
                .to_string();

            Some((name, (interface_type, uri_part.to_string())))
        })
        .collect();

    Ok(devices)
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
