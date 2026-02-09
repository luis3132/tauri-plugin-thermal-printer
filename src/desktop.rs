use serde::de::DeserializeOwned;
use tauri::{plugin::PluginApi, AppHandle, Runtime};

use crate::desktop_printers::unix_base::{get_printers_info, print_raw_data};
use crate::desktop_printers::windows::{get_printers_info_win, print_raw_data_win};
use crate::models::*;
use crate::process::process_print::ProcessPrint;
use crate::process::process_print_test::TestPrinter;
use crate::error::{Error, Result};

pub const OS_NAME: &str = std::env::consts::OS;

pub fn init<R: Runtime, C: DeserializeOwned>(
    app: &AppHandle<R>,
    _api: PluginApi<R, C>,
) -> Result<ThermalPrinter<R>> {
    Ok(ThermalPrinter(app.clone()))
}

/// Access to the thermal-printer APIs.
pub struct ThermalPrinter<R: Runtime>(AppHandle<R>);

impl<R: Runtime> ThermalPrinter<R> {

    pub fn list_thermal_printers(&self) -> Result<Vec<PrinterInfo>> {
        if OS_NAME == "linux" || OS_NAME == "macos" {
            let printer = get_printers_info()?;
            Ok(printer)
        } else if OS_NAME == "windows" {
            let printer = get_printers_info_win()
                .map_err(|err| {
                    let err_msg = format!("Error getting printers info: {}", err);
                    println!("{}", err_msg);
                    Error::Io(std::io::Error::new(std::io::ErrorKind::Other, err_msg))
                })?;
            Ok(printer)
        } else {
            Err(Error::UnsupportedPlatform)
        }
    }

    pub fn print_thermal_printer(&self, print_job_request: PrintJobRequest) -> Result<()> {
        if OS_NAME == "linux" || OS_NAME == "macos" {
            let mut process_print = ProcessPrint::new();
            let data = process_print.generate_document(&print_job_request)
                .map_err(|err| {
                    println!("Error generating document: {}", err);
                    Error::Io(std::io::Error::new(std::io::ErrorKind::InvalidInput, err))
                })?;
            print_raw_data(&print_job_request.printer, &data)
                .map_err(|err| {
                    println!("Error printing raw data: {}", err);
                    err
                })?;
            Ok(())
        } else if OS_NAME == "windows" {
            let mut process_print = ProcessPrint::new();
            let data = process_print.generate_document(&print_job_request)
                .map_err(|err| {
                    println!("Error generating document: {}", err);
                    Error::Io(std::io::Error::new(std::io::ErrorKind::InvalidInput, err))
                })?;
            print_raw_data_win(&print_job_request.printer, &data)
                .map_err(|err| {
                    println!("Error printing raw data: {}", err);
                    Error::Io(err)
                })?;
            Ok(())
        } else {
            Err(Error::UnsupportedPlatform)
        }
    }

    pub fn test_thermal_printer(&self, print_job_request: TestPrintRequest) -> Result<()> {
        if OS_NAME == "linux" || OS_NAME == "macos" {
            let mut process_print = TestPrinter::new();
            let data = process_print.generate_test_document(&print_job_request)
                .map_err(|err| {
                    println!("Error generating document: {}", err);
                    Error::Io(std::io::Error::new(std::io::ErrorKind::InvalidInput, err))
                })?;
            crate::desktop_printers::unix_base::print_raw_data(&print_job_request.printer_info.printer, &data)
                .map_err(|err| {
                    println!("Error printing raw data: {}", err);
                    err
                })?;
            Ok(())
        } else if OS_NAME == "windows" {
            let mut process_print = TestPrinter::new();
            let data = process_print.generate_test_document(&print_job_request)
                .map_err(|err| {
                    println!("Error generating document: {}", err);
                    Error::Io(std::io::Error::new(std::io::ErrorKind::InvalidInput, err))
                })?;
            print_raw_data_win(&print_job_request.printer_info.printer, &data)
                .map_err(|err| {
                    println!("Error printing raw data: {}", err);
                    Error::Io(err)
                })?;
            Ok(())
        } else {
            Err(Error::UnsupportedPlatform)
        }
    }
}
