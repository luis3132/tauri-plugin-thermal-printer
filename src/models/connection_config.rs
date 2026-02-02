use serde::{Deserialize, Serialize};

/// Connection type for printer communication
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum ConnectionConfig {
    /// Use system's default printing (lp command on Linux/macOS)
    System {
        printer_name: String,
    },
    /// Network connection via TCP/IP
    Network {
        host: String,
        port: u16,
        #[serde(default = "default_timeout")]
        timeout_secs: u64,
    },
    /// USB connection (requires vendor_id and product_id)
    #[cfg(feature = "usb")]
    Usb {
        vendor_id: u16,
        product_id: u16,
        #[serde(default = "default_timeout")]
        timeout_secs: u64,
    },
    /// Serial port connection
    #[cfg(feature = "serial_port")]
    Serial {
        port: String,
        baud_rate: u32,
        #[serde(default = "default_timeout")]
        timeout_secs: u64,
    },
    /// Direct file/device access
    File {
        path: String,
    },
    /// Console output for debugging
    Console {
        show_output: bool,
    },
}

fn default_timeout() -> u64 {
    5
}

impl Default for ConnectionConfig {
    fn default() -> Self {
        ConnectionConfig::Console {
            show_output: true,
        }
    }
}

/// Printer information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrinterDevice {
    pub name: String,
    pub is_default: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub driver: Option<String>,
}

/// USB device information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UsbDevice {
    pub vendor_id: u16,
    pub product_id: u16,
    pub name: String,
}
