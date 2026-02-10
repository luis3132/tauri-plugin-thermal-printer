use serde::de::DeserializeOwned;
use tauri::{
    plugin::{PluginApi, PluginHandle},
    AppHandle, Runtime,
};

use crate::error::{Error, Result};
use crate::models::*;

pub const OS_NAME: &str = std::env::consts::OS;

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
    pub fn ping(&self, payload: PingRequest) -> Result<PingResponse> {
        self.0
            .run_mobile_plugin("ping", payload)
            .map_err(Into::into)
    }
    pub fn list_thermal_printers(&self) -> Result<Vec<PrinterInfo>> {
        if OS_NAME == "android" {
            println!("Listing thermal printers");
            Ok(self.0.run_mobile_plugin("list_thermal_printers", ())?)
        } else {
            Err(Error::UnsupportedPlatform)
        }
    }

    pub fn print_thermal_printer(&self, _print_job_request: PrintJobRequest) -> Result<()> {
        if OS_NAME == "android" {
            Ok(())
        } else {
            Err(Error::UnsupportedPlatform)
        }
    }

    pub fn test_thermal_printer(&self, _print_job_request: TestPrintRequest) -> Result<()> {
        if OS_NAME == "android" {
            Ok(())
        } else {
            Err(Error::UnsupportedPlatform)
        }
    }
}
