//! Network driver for TCP/IP connected printers

use super::{Driver, DriverError, Result};
use std::io::{Read, Write};
use std::net::{IpAddr, SocketAddr, TcpStream};
use std::sync::{Arc, Mutex};
use std::time::Duration;

const DEFAULT_TIMEOUT_SECONDS: u64 = 5;

/// Network driver for TCP/IP printers
#[derive(Clone)]
pub struct NetworkDriver {
    host: String,
    port: u16,
    stream: Arc<Mutex<TcpStream>>,
    timeout: Duration,
}

impl NetworkDriver {
    /// Create a new network driver
    ///
    /// # Arguments
    /// * `host` - IP address or hostname of the printer
    /// * `port` - TCP port (usually 9100 for thermal printers)
    /// * `timeout` - Optional timeout for operations
    ///
    /// # Example
    /// ```no_run
    /// use std::time::Duration;
    /// let driver = NetworkDriver::new("192.168.1.100", 9100, Some(Duration::from_secs(5)))?;
    /// ```
    pub fn new(host: &str, port: u16, timeout: Option<Duration>) -> Result<Self> {
        let stream = match timeout {
            Some(timeout) => {
                let addr = SocketAddr::new(
                    host.parse::<IpAddr>()
                        .map_err(|e| DriverError::Connection(format!("Invalid IP address: {}", e)))?,
                    port,
                );
                TcpStream::connect_timeout(&addr, timeout)?
            }
            None => TcpStream::connect((host, port))?,
        };

        let timeout = timeout.unwrap_or(Duration::from_secs(DEFAULT_TIMEOUT_SECONDS));

        Ok(Self {
            host: host.to_string(),
            port,
            stream: Arc::new(Mutex::new(stream)),
            timeout,
        })
    }
}

impl Driver for NetworkDriver {
    fn name(&self) -> String {
        format!("network ({}:{})", self.host, self.port)
    }

    fn write(&self, data: &[u8]) -> Result<()> {
        let mut stream = self.stream.lock()?;
        stream.set_write_timeout(Some(self.timeout))?;
        stream.write_all(data)?;
        Ok(())
    }

    fn read(&self, buf: &mut [u8]) -> Result<usize> {
        let mut stream = self.stream.lock()?;
        stream.set_read_timeout(Some(self.timeout))?;
        Ok(stream.read(buf)?)
    }

    fn flush(&self) -> Result<()> {
        Ok(self.stream.lock()?.flush()?)
    }
}
