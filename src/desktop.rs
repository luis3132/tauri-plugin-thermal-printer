use serde::de::DeserializeOwned;
use tauri::{plugin::PluginApi, AppHandle, Runtime};

use crate::desktop_printers::unix_base::get_printers_info;
use crate::models::*;
use crate::process::process_print::ProcessPrint;
use crate::process::process_print_test::TestPrinter;

pub const OS_NAME: &str = std::env::consts::OS;

pub fn init<R: Runtime, C: DeserializeOwned>(
    app: &AppHandle<R>,
    _api: PluginApi<R, C>,
) -> crate::Result<ThermalPrinter<R>> {
    Ok(ThermalPrinter(app.clone()))
}

/// Access to the thermal-printer APIs.
pub struct ThermalPrinter<R: Runtime>(AppHandle<R>);

impl<R: Runtime> ThermalPrinter<R> {

    pub fn list_thermal_printers(&self) -> crate::Result<Vec<PrinterInfo>> {
        if OS_NAME == "linux" || OS_NAME == "macos" {
            let printer = get_printers_info()?;
            Ok(printer)
        } else {
            Err(crate::Error::UnsupportedPlatform)
        }
    }

    pub fn print_thermal_printer(&self, print_job_request: PrintJobRequest) -> crate::Result<()> {
        if OS_NAME == "linux" || OS_NAME == "macos" {
            let mut process_print = ProcessPrint::new();
            let data = process_print.generate_document(&print_job_request)
                .map_err(|err| {
                    println!("Error generating document: {}", err);
                    crate::Error::Io(std::io::Error::new(std::io::ErrorKind::InvalidInput, err))
                })?;
            crate::desktop_printers::unix_base::print_raw_data(&print_job_request.printer, &data)
                .map_err(|err| {
                    println!("Error printing raw data: {}", err);
                    err
                })?;
            Ok(())
        } else {
            Err(crate::Error::UnsupportedPlatform)
        }
    }

    pub fn test_thermal_printer(&self, print_job_request: TestPrintRequest) -> crate::Result<()> {
        if OS_NAME == "linux" || OS_NAME == "macos" {
            let mut process_print = TestPrinter::new();
            let data = process_print.generate_test_document(&print_job_request)
                .map_err(|err| {
                    println!("Error generating document: {}", err);
                    crate::Error::Io(std::io::Error::new(std::io::ErrorKind::InvalidInput, err))
                })?;
            crate::desktop_printers::unix_base::print_raw_data(&print_job_request.printer_info.printer, &data)
                .map_err(|err| {
                    println!("Error printing raw data: {}", err);
                    err
                })?;
            Ok(())
        } else {
            Err(crate::Error::UnsupportedPlatform)
        }
    }
}
