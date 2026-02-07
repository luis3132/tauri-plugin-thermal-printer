use crate::PrinterInfo;
use std::{io::Write, process::{Command, Stdio}};

#[cfg(windows)]
use std::os::windows::process::CommandExt;

// Constante para CREATE_NO_WINDOW
#[cfg(windows)]
const CREATE_NO_WINDOW: u32 = 0x08000000;

pub fn get_printers_info_win() -> Result<Vec<PrinterInfo>, Box<dyn std::error::Error>> {
    let mut command = Command::new("powershell");
    command
        .args(&[
            "-NoProfile",
            "-WindowStyle", "Hidden",
            "-Command",
            r#"
            $printers = @(Get-Printer | ForEach-Object {
                $portName = $_.PortName
                $port = Get-PrinterPort -Name $portName -ErrorAction SilentlyContinue
                
                $interfaceType = if ($portName -match '^USB') { 
                    'USB' 
                } elseif ($portName -match '^LPT') { 
                    'PARALLEL' 
                } elseif ($portName -match '^COM') { 
                    'SERIAL' 
                } elseif ($portName -match 'IP_') { 
                    'NETWORK' 
                } else { 
                    'OTHER' 
                }
                
                $identifier = if ($interfaceType -eq 'USB') {
                    "usb://$($_.DriverName)/$($_.Name)"
                } elseif ($interfaceType -eq 'NETWORK') {
                    "network://$($port.PrinterHostAddress)"
                } else {
                    "$($interfaceType.ToLower())://$portName"
                }
                
                $status = switch ($_.PrinterStatus) {
                    0 { 'IDLE' }
                    1 { 'PAUSED' }
                    2 { 'ERROR' }
                    3 { 'PENDING_DELETION' }
                    4 { 'PAPER_JAM' }
                    5 { 'PAPER_OUT' }
                    6 { 'MANUAL_FEED' }
                    7 { 'PAPER_PROBLEM' }
                    8 { 'OFFLINE' }
                    9 { 'IO_ACTIVE' }
                    10 { 'BUSY' }
                    11 { 'PRINTING' }
                    default { 'UNKNOWN' }
                }
                
                [PSCustomObject]@{
                    name = $_.Name
                    interface_type = $interfaceType
                    identifier = $identifier
                    status = $status
                }
            })
            if ($printers.Count -eq 0) {
                Write-Output "[]"
            } else {
                $printers | ConvertTo-Json -AsArray
            }
            "#
        ]);
    
    #[cfg(windows)]
    command.creation_flags(CREATE_NO_WINDOW);
    
    let output = command.output()?;
    
    // Check if command failed
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(format!("PowerShell command failed: {}", stderr).into());
    }
    
    let stdout = String::from_utf8_lossy(&output.stdout).trim().to_string();
    
    // Handle empty output
    if stdout.is_empty() {
        return Ok(Vec::new());
    }
    
    // Parse JSON
    let printers: Vec<PrinterInfo> = serde_json::from_str(&stdout)
        .map_err(|e| format!("Failed to parse JSON: {}. Output was: '{}'", e, stdout))?;
    
    Ok(printers)
}

pub fn print_raw_data_win(printer_name: &str, data: &[u8]) -> std::io::Result<()> {
    let mut command = Command::new("powershell");
    command
        .args([
            "-NoProfile",
            "-WindowStyle", "Hidden",
            "-Command",
            &format!("$input | Out-Printer -Name '{}'", printer_name)
        ])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());
    
    #[cfg(windows)]
    command.creation_flags(CREATE_NO_WINDOW);
    
    let mut child = command.spawn()?;
    
    if let Some(mut stdin) = child.stdin.take() {
        stdin.write_all(data)?;
    }
    
    let output = child.wait_with_output()?;
    
    if output.status.success() {
        Ok(())
    } else {
        let error_msg = String::from_utf8_lossy(&output.stderr);
        Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("Print failed: {}", error_msg),
        ))
    }
}