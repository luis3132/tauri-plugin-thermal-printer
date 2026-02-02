use tauri::{AppHandle, command, Runtime};

use crate::models::*;
use crate::Result;
use crate::ThermalPrinterExt;

#[cfg(desktop)]
use crate::desktop::PrinterInfo;

#[command]
pub(crate) async fn ping<R: Runtime>(
    app: AppHandle<R>,
    payload: PingRequest,
) -> Result<PingResponse> {
    app.thermal_printer().ping(payload)
}

#[cfg(desktop)]
#[command]
pub(crate) async fn list_printers<R: Runtime>(
    app: AppHandle<R>,
) -> Result<Vec<PrinterInfo>> {
    app.thermal_printer().list_printers()
}

#[cfg(desktop)]
#[command]
pub(crate) async fn print<R: Runtime>(
    app: AppHandle<R>,
    request: PrintJobRequest,
) -> Result<()> {
    app.thermal_printer().print(request)
}
