//! Serial port driver for RS-232/COM port printers

use super::{Driver, DriverError, Result};
use serialport::SerialPort;
use std::io::{Read, Write};
use std::sync::{Arc, Mutex};
use std::time::Duration;

const DEFAULT_TIMEOUT_SECONDS: u64 = 5;

/// Serial port driver
#[derive(Clone)]
pub struct SerialPortDriver {
    path: String,
    port: Arc<Mutex<Box<dyn SerialPort>>>,
}

impl SerialPortDriver {
    /// Create a new serial port driver
    ///
    /// # Arguments
    /// * `path` - Serial port path (e.g., "/dev/ttyUSB0" on Linux, "COM1" on Windows)
    /// * `baud_rate` - Baud rate (e.g., 9600, 115200)
    /// * `timeout` - Optional timeout for operations
    ///
    /// # Example
    /// ```no_run
    /// use std::time::Duration;
    /// let driver = SerialPortDriver::new("/dev/ttyUSB0", 115200, Some(Duration::from_secs(5)))?;
    /// ```
    pub fn new(path: &str, baud_rate: u32, timeout: Option<Duration>) -> Result<Self> {
        let mut builder = serialport::new(path, baud_rate);

        if let Some(timeout) = timeout {
            builder = builder.timeout(timeout);
        } else {
            builder = builder.timeout(Duration::from_secs(DEFAULT_TIMEOUT_SECONDS));
        }

        let port = builder
            .open()
            .map_err(|e| DriverError::Connection(format!("Failed to open serial port: {}", e)))?;

        Ok(Self {
            path: path.to_string(),
            port: Arc::new(Mutex::new(port)),
        })
    }

    /// List available serial ports
    pub fn list_ports() -> Result<Vec<String>> {
        let ports = serialport::available_ports()
            .map_err(|e| DriverError::Io(format!("Failed to list serial ports: {}", e)))?;

        Ok(ports.iter().map(|p| p.port_name.clone()).collect())
    }
}

impl Driver for SerialPortDriver {
    fn name(&self) -> String {
        format!("serial port ({})", self.path)
    }

    fn write(&self, data: &[u8]) -> Result<()> {
        self.port.lock()?.write_all(data)?;
        Ok(())
    }

    fn read(&self, buf: &mut [u8]) -> Result<usize> {
        Ok(self.port.lock()?.read(buf)?)
    }

    fn flush(&self) -> Result<()> {
        Ok(self.port.lock()?.flush()?)
    }
}
