use tauri::{
  plugin::{Builder, TauriPlugin},
  Manager, Runtime,
};

pub use models::*;

#[cfg(desktop)]
mod desktop;
#[cfg(mobile)]
mod mobile;

mod commands;
mod error;
mod models;

pub use error::{Error, Result};

#[cfg(desktop)]
use desktop::ThermalPrinter;
#[cfg(mobile)]
use mobile::ThermalPrinter;

/// Extensions to [`tauri::App`], [`tauri::AppHandle`] and [`tauri::Window`] to access the thermal-printer APIs.
pub trait ThermalPrinterExt<R: Runtime> {
  fn thermal_printer(&self) -> &ThermalPrinter<R>;
}

impl<R: Runtime, T: Manager<R>> crate::ThermalPrinterExt<R> for T {
  fn thermal_printer(&self) -> &ThermalPrinter<R> {
    self.state::<ThermalPrinter<R>>().inner()
  }
}

/// Initializes the plugin.
pub fn init<R: Runtime>() -> TauriPlugin<R> {
  Builder::new("thermal-printer")
    .invoke_handler(tauri::generate_handler![commands::ping])
    .setup(|app, api| {
      #[cfg(mobile)]
      let thermal_printer = mobile::init(app, api)?;
      #[cfg(desktop)]
      let thermal_printer = desktop::init(app, api)?;
      app.manage(thermal_printer);
      Ok(())
    })
    .build()
}
