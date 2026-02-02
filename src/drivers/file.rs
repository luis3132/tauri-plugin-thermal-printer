//! File driver for device files (Linux/Unix) or direct printer access

use super::{Driver, Result};
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use std::path::Path;
use std::sync::{Arc, Mutex};

/// File driver for writing to device files or printer ports
#[derive(Clone)]
pub struct FileDriver {
    path: String,
    file: Arc<Mutex<File>>,
}

impl FileDriver {
    /// Create a new file driver
    ///
    /// # Arguments
    /// * `path` - Path to the device file (e.g., /dev/usb/lp0, /dev/ttyUSB0)
    ///
    /// # Example
    /// ```no_run
    /// use std::path::Path;
    /// let driver = FileDriver::new(Path::new("/dev/usb/lp0"))?;
    /// ```
    pub fn new(path: &Path) -> Result<Self> {
        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(false)
            .open(path)?;

        Ok(Self {
            path: path.to_string_lossy().to_string(),
            file: Arc::new(Mutex::new(file)),
        })
    }
}

impl Driver for FileDriver {
    fn name(&self) -> String {
        format!("file ({})", self.path)
    }

    fn write(&self, data: &[u8]) -> Result<()> {
        self.file.lock()?.write_all(data)?;
        Ok(())
    }

    fn read(&self, buf: &mut [u8]) -> Result<usize> {
        Ok(self.file.lock()?.read(buf)?)
    }

    fn flush(&self) -> Result<()> {
        Ok(self.file.lock()?.flush()?)
    }
}
