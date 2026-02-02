//! Drivers to communicate with thermal printers
//!
//! This module provides different ways to connect and send data to thermal printers:
//! - Network (TCP/IP)
//! - USB (direct USB connection)
//! - Serial Port (COM/tty ports)
//! - File (direct write to device file)

use std::io::{self, Write};
use std::sync::{Arc, Mutex};

pub mod network;
pub mod file;
#[cfg(feature = "usb")]
pub mod usb;
#[cfg(feature = "serial_port")]
pub mod serial;

/// Result type for driver operations
pub type Result<T> = std::result::Result<T, DriverError>;

/// Driver errors
#[derive(Debug)]
pub enum DriverError {
    Io(String),
    Connection(String),
    Timeout(String),
    NotFound(String),
}

impl std::fmt::Display for DriverError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DriverError::Io(msg) => write!(f, "IO error: {}", msg),
            DriverError::Connection(msg) => write!(f, "Connection error: {}", msg),
            DriverError::Timeout(msg) => write!(f, "Timeout error: {}", msg),
            DriverError::NotFound(msg) => write!(f, "Not found: {}", msg),
        }
    }
}

impl std::error::Error for DriverError {}

impl From<io::Error> for DriverError {
    fn from(err: io::Error) -> Self {
        DriverError::Io(err.to_string())
    }
}

impl<T> From<std::sync::PoisonError<T>> for DriverError {
    fn from(err: std::sync::PoisonError<T>) -> Self {
        DriverError::Io(format!("Mutex poisoned: {}", err))
    }
}

/// Trait for printer drivers
///
/// All drivers must implement this trait to provide a uniform interface
/// for sending data to printers regardless of the connection type.
pub trait Driver: Send + Sync {
    /// Returns the name/description of this driver
    fn name(&self) -> String;

    /// Write data to the printer
    fn write(&self, data: &[u8]) -> Result<()>;

    /// Read data from the printer (if supported)
    fn read(&self, buf: &mut [u8]) -> Result<usize>;

    /// Flush any buffered data
    fn flush(&self) -> Result<()>;
}

/// Console driver for debugging (prints to stdout)
#[derive(Clone)]
pub struct ConsoleDriver {
    show_output: bool,
}

impl ConsoleDriver {
    pub fn new(show_output: bool) -> Self {
        Self { show_output }
    }
}

impl Driver for ConsoleDriver {
    fn name(&self) -> String {
        "console (debug)".to_string()
    }

    fn write(&self, data: &[u8]) -> Result<()> {
        if self.show_output {
            io::stdout().write_all(data)?;
        }
        Ok(())
    }

    fn read(&self, _buf: &mut [u8]) -> Result<usize> {
        Ok(0)
    }

    fn flush(&self) -> Result<()> {
        Ok(())
    }
}
