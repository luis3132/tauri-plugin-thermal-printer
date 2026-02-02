use tauri::{AppHandle, command, Runtime};

use crate::models::*;
use crate::Result;
use crate::ThermalPrinterExt;

#[cfg(desktop)]
use crate::desktop::SystemPrinterInfo;

#[command]
pub(crate) async fn ping<R: Runtime>(
    app: AppHandle<R>,
    payload: PingRequest,
) -> Result<PingResponse> {
    app.thermal_printer().ping(payload)
}

#[cfg(desktop)]
#[command]
pub(crate) async fn list_system_printers<R: Runtime>(
    app: AppHandle<R>,
) -> Result<Vec<SystemPrinterInfo>> {
    app.thermal_printer().list_system_printers()
}

#[cfg(all(desktop, feature = "usb"))]
#[command]
pub(crate) async fn list_usb_devices<R: Runtime>(
    app: AppHandle<R>,
) -> Result<Vec<UsbDevice>> {
    app.thermal_printer().list_usb_devices()
}

#[cfg(all(desktop, feature = "serial_port"))]
#[command]
pub(crate) async fn list_serial_ports<R: Runtime>(
    app: AppHandle<R>,
) -> Result<Vec<String>> {
    app.thermal_printer().list_serial_ports()
}

#[cfg(desktop)]
#[command]
pub(crate) async fn print<R: Runtime>(
    app: AppHandle<R>,
    request: PrintJobRequest,
) -> Result<()> {
    app.thermal_printer().print(request)
}
