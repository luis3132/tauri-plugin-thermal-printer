use serde::de::DeserializeOwned;
use tauri::{
    plugin::{PluginApi, PluginHandle},
    AppHandle, Runtime,
};

use crate::error::{Error, Result};
use crate::models::*;
use crate::process::process_print::ProcessPrint;
use crate::process::process_print_test::TestPrinter;

pub const OS_NAME: &str = std::env::consts::OS;

#[derive(Debug, serde::Deserialize)]
pub struct PrintersResponse {
    pub printers: Vec<PrinterInfo>,
}

#[derive(Debug, serde::Serialize)]
struct PrintRawRequest {
    identifier: String,
    data: Vec<u8>,
}

#[cfg(target_os = "ios")]
tauri::ios_plugin_binding!(init_plugin_thermal_printer);

// initializes the Kotlin or Swift plugin classes
pub fn init<R: Runtime, C: DeserializeOwned>(
    _app: &AppHandle<R>,
    api: PluginApi<R, C>,
) -> Result<ThermalPrinter<R>> {
    #[cfg(target_os = "android")]
    let handle =
        api.register_android_plugin("com.luis3132.thermal_printer", "Thermal_Printer_Plugin")?;
    #[cfg(target_os = "ios")]
    let handle = api.register_ios_plugin(init_plugin_thermal_printer)?;
    Ok(ThermalPrinter(handle))
}

/// Access to the thermal-printer APIs.
pub struct ThermalPrinter<R: Runtime>(PluginHandle<R>);

impl<R: Runtime> ThermalPrinter<R> {
    pub fn list_thermal_printers(&self) -> Result<Vec<PrinterInfo>> {
        if OS_NAME == "android" {
            println!("Listing thermal printers");
            let response: PrintersResponse =
                self.0.run_mobile_plugin("list_thermal_printers", ())?;
            Ok(response.printers)
        } else {
            Err(Error::UnsupportedPlatform)
        }
    }

    pub fn print_thermal_printer(&self, print_job_request: PrintJobRequest) -> Result<()> {
        if OS_NAME == "android" {
            let identifier = print_job_request.printer.clone();
            let data = ProcessPrint::new()
                .generate_document(&print_job_request)
                .map_err(|e| Error::Io(std::io::Error::new(std::io::ErrorKind::InvalidInput, e)))?;
            let _: () = self
                .0
                .run_mobile_plugin("print_raw_data", PrintRawRequest { identifier, data })?;
            Ok(())
        } else {
            Err(Error::UnsupportedPlatform)
        }
    }

    pub fn test_thermal_printer(&self, print_job_request: TestPrintRequest) -> Result<()> {
        if OS_NAME == "android" {
            let identifier = print_job_request.printer_info.printer.clone();
            let data = TestPrinter::new()
                .generate_test_document(&print_job_request)
                .map_err(|e| Error::Io(std::io::Error::new(std::io::ErrorKind::InvalidInput, e)))?;
            let _: () = self
                .0
                .run_mobile_plugin("print_raw_data", PrintRawRequest { identifier, data })?;
            Ok(())
        } else {
            Err(Error::UnsupportedPlatform)
        }
    }
}
