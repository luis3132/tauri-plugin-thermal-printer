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
mod process;
mod commands_esc_pos;
mod drivers;

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
  let mut builder = Builder::new("thermal-printer");
  
  #[cfg(desktop)]
  {
    #[cfg(all(feature = "usb", feature = "serial_port"))]
    {
      builder = builder.invoke_handler(tauri::generate_handler![
        commands::ping,
        commands::list_system_printers,
        commands::list_usb_devices,
        commands::list_serial_ports,
        commands::print
      ]);
    }
    
    #[cfg(all(feature = "usb", not(feature = "serial_port")))]
    {
      builder = builder.invoke_handler(tauri::generate_handler![
        commands::ping,
        commands::list_system_printers,
        commands::list_usb_devices,
        commands::print
      ]);
    }
    
    #[cfg(all(not(feature = "usb"), feature = "serial_port"))]
    {
      builder = builder.invoke_handler(tauri::generate_handler![
        commands::ping,
        commands::list_system_printers,
        commands::list_serial_ports,
        commands::print
      ]);
    }
    
    #[cfg(all(not(feature = "usb"), not(feature = "serial_port")))]
    {
      builder = builder.invoke_handler(tauri::generate_handler![
        commands::ping,
        commands::list_system_printers,
        commands::print
      ]);
    }
  }
  
  #[cfg(mobile)]
  {
    builder = builder.invoke_handler(tauri::generate_handler![commands::ping]);
  }
  
  builder
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
