//! USB driver for direct USB connection to printers

use super::{Driver, DriverError, Result};
use rusb::{Context, DeviceHandle, Direction, TransferType};
use std::sync::{Arc, Mutex};
use std::time::Duration;

const DEFAULT_TIMEOUT_SECONDS: u64 = 5;

/// USB driver for direct USB communication
#[derive(Clone)]
pub struct UsbDriver {
    vendor_id: u16,
    product_id: u16,
    output_endpoint: u8,
    input_endpoint: u8,
    device: Arc<Mutex<DeviceHandle<Context>>>,
    timeout: Duration,
}

impl UsbDriver {
    /// Create a new USB driver
    ///
    /// # Arguments
    /// * `vendor_id` - USB Vendor ID
    /// * `product_id` - USB Product ID
    /// * `timeout` - Optional timeout for operations
    ///
    /// # Example
    /// ```no_run
    /// use std::time::Duration;
    /// let driver = UsbDriver::new(0x0525, 0xa700, Some(Duration::from_secs(5)))?;
    /// ```
    pub fn new(vendor_id: u16, product_id: u16, timeout: Option<Duration>) -> Result<Self> {
        let context = Context::new().map_err(|e| DriverError::Io(e.to_string()))?;
        let devices = context.devices().map_err(|e| DriverError::Io(e.to_string()))?;

        for device in devices.iter() {
            let device_descriptor = device
                .device_descriptor()
                .map_err(|e| DriverError::Io(e.to_string()))?;

            if device_descriptor.vendor_id() == vendor_id
                && device_descriptor.product_id() == product_id
            {
                let config_descriptor = device
                    .active_config_descriptor()
                    .map_err(|e| DriverError::Io(e.to_string()))?;

                let (output_endpoint, input_endpoint, interface_number) = config_descriptor
                    .interfaces()
                    .flat_map(|interface| interface.descriptors())
                    .flat_map(|descriptor| {
                        let interface_number = descriptor.interface_number();

                        let mut input_endpoint = None;
                        let mut output_endpoint = None;

                        for endpoint in descriptor.endpoint_descriptors() {
                            if endpoint.transfer_type() == TransferType::Bulk
                                && endpoint.direction() == Direction::In
                            {
                                input_endpoint = Some(endpoint.address());
                            } else if endpoint.transfer_type() == TransferType::Bulk
                                && endpoint.direction() == Direction::Out
                            {
                                output_endpoint = Some(endpoint.address());
                            }
                        }

                        match (output_endpoint, input_endpoint) {
                            (Some(out), Some(inp)) => Some((out, inp, interface_number)),
                            _ => None,
                        }
                    })
                    .next()
                    .ok_or_else(|| {
                        DriverError::NotFound(
                            "No suitable USB endpoints found for device".to_string(),
                        )
                    })?;

                let device_handle = device
                    .open()
                    .map_err(|e| DriverError::Connection(format!("Failed to open USB device: {}", e)))?;

                // Detach kernel driver on Linux if needed
                #[cfg(not(target_os = "windows"))]
                {
                    if let Ok(true) = device_handle.kernel_driver_active(interface_number) {
                        device_handle
                            .detach_kernel_driver(interface_number)
                            .map_err(|e| DriverError::Io(format!("Failed to detach kernel driver: {}", e)))?;
                    }
                }

                // Claim the interface
                device_handle
                    .claim_interface(interface_number)
                    .map_err(|e| DriverError::Connection(format!("Failed to claim USB interface: {}", e)))?;

                return Ok(Self {
                    vendor_id,
                    product_id,
                    output_endpoint,
                    input_endpoint,
                    device: Arc::new(Mutex::new(device_handle)),
                    timeout: timeout.unwrap_or(Duration::from_secs(DEFAULT_TIMEOUT_SECONDS)),
                });
            }
        }

        Err(DriverError::NotFound(format!(
            "USB device not found (VID: {:04x}, PID: {:04x})",
            vendor_id, product_id
        )))
    }

    /// List available USB devices
    pub fn list_devices() -> Result<Vec<(u16, u16, String)>> {
        let context = Context::new().map_err(|e| DriverError::Io(e.to_string()))?;
        let devices = context.devices().map_err(|e| DriverError::Io(e.to_string()))?;

        let mut result = Vec::new();

        for device in devices.iter() {
            if let Ok(descriptor) = device.device_descriptor() {
                let vendor_id = descriptor.vendor_id();
                let product_id = descriptor.product_id();

                let name = if let Ok(handle) = device.open() {
                    handle
                        .read_product_string_ascii(&descriptor)
                        .unwrap_or_else(|_| format!("Unknown (VID: {:04x}, PID: {:04x})", vendor_id, product_id))
                } else {
                    format!("Unknown (VID: {:04x}, PID: {:04x})", vendor_id, product_id)
                };

                result.push((vendor_id, product_id, name));
            }
        }

        Ok(result)
    }
}

impl Driver for UsbDriver {
    fn name(&self) -> String {
        format!(
            "USB (VID: {:04x}, PID: {:04x})",
            self.vendor_id, self.product_id
        )
    }

    fn write(&self, data: &[u8]) -> Result<()> {
        self.device
            .lock()?
            .write_bulk(self.output_endpoint, data, self.timeout)
            .map_err(|e| DriverError::Io(e.to_string()))?;
        Ok(())
    }

    fn read(&self, buf: &mut [u8]) -> Result<usize> {
        self.device
            .lock()?
            .read_bulk(self.input_endpoint, buf, self.timeout)
            .map_err(|e| DriverError::Io(e.to_string()))
    }

    fn flush(&self) -> Result<()> {
        Ok(())
    }
}
