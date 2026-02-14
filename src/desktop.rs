use serde::de::DeserializeOwned;
use tauri::{plugin::PluginApi, AppHandle, Runtime};

use crate::models::*;
use crate::process::process_print::ProcessPrint;
use crate::process::process_print_test::TestPrinter;
use crate::error::{Error, Result};

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
        #[cfg(target_os = "windows")]
        {
            let printer = crate::desktop_printers::windows::get_printers_info_win()
                .map_err(|err| {
                    let err_msg = format!("Error getting printers info: {}", err);
                    println!("{}", err_msg);
                    Error::Io(std::io::Error::new(std::io::ErrorKind::Other, err_msg))
                })?;
            Ok(printer)
        }
        #[cfg(not(target_os = "windows"))]
        {
            let printer = crate::desktop_printers::unix_base::get_printers_info()?;
            Ok(printer)
        }
    }

    pub fn print_thermal_printer(&self, print_job_request: PrintJobRequest) -> Result<()> {
        let mut process_print = ProcessPrint::new();
        let data = process_print.generate_document(&print_job_request)
            .map_err(|err| {
                println!("Error generating document: {}", err);
                Error::Io(std::io::Error::new(std::io::ErrorKind::InvalidInput, err))
            })?;
        #[cfg(target_os = "windows")]
        {
            crate::desktop_printers::windows::print_raw_data_win(&print_job_request.printer, &data)
                .map_err(|err| {
                    println!("Error printing raw data: {}", err);
                    Error::Io(err)
                })?;
        }
        #[cfg(not(target_os = "windows"))]
        {
            crate::desktop_printers::unix_base::print_raw_data(&print_job_request.printer, &data)
                .map_err(|err| {
                    println!("Error printing raw data: {}", err);
                    err
                })?;
        }
        Ok(())
    }

    pub fn test_thermal_printer(&self, print_job_request: TestPrintRequest) -> Result<()> {
        let mut process_print = TestPrinter::new();
        let data = process_print.generate_test_document(&print_job_request)
            .map_err(|err| {
                println!("Error generating document: {}", err);
                Error::Io(std::io::Error::new(std::io::ErrorKind::InvalidInput, err))
            })?;
        #[cfg(target_os = "windows")]
        {
            crate::desktop_printers::windows::print_raw_data_win(&print_job_request.printer_info.printer, &data)
                .map_err(|err| {
                    println!("Error printing raw data: {}", err);
                    Error::Io(err)
                })?;
        }
        #[cfg(not(target_os = "windows"))]
        {
            crate::desktop_printers::unix_base::print_raw_data(&print_job_request.printer_info.printer, &data)
                .map_err(|err| {
                    println!("Error printing raw data: {}", err);
                    err
                })?;
        }
        Ok(())
    }
}
