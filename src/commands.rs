use tauri::{AppHandle, command, Runtime};

use crate::models::*;
use crate::Result;
use crate::ThermalPrinterExt;

#[command]
pub(crate) async fn ping<R: Runtime>(
    app: AppHandle<R>,
    payload: PingRequest,
) -> Result<PingResponse> {
    app.thermal_printer().ping(payload)
}
