use tauri::{AppHandle, command, Runtime};

use crate::models::*;
use crate::Result;
use crate::ThermalPrinterExt;

#[command]
pub(crate) async fn print_thermal_printer<R: Runtime>(
    app: AppHandle<R>,
    print_job_request: PrintJobRequest,
) -> Result<()> {
    app.thermal_printer().print_thermal_printer(print_job_request)
}

#[command]
pub(crate) async fn list_thermal_printers<R: Runtime>(
    app: AppHandle<R>,
) -> Result<Vec<PrinterInfo>> {
    app.thermal_printer().list_thermal_printers()
}

#[command]
pub(crate) async fn test_thermal_printer<R: Runtime>(
    app: AppHandle<R>,
    print_job_request: TestPrintRequest,
) -> Result<()> {
    app.thermal_printer().test_thermal_printer(print_job_request)
}