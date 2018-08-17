use std::{fs, io};
//use std::os::unix::io::AsRawFd;

use super::libc;


/// Is this stream a TTY?
pub fn is_tty() -> bool {
    //unsafe { libc::isatty(stream.as_raw_fd()) == 1 }
    true
}

/// Get the TTY device.
///
/// This allows for getting stdio representing _only_ the TTY, and not other streams.
pub fn get_tty() -> io::Result<fs::File> {
    fs::OpenOptions::new().read(true).write(true).open("/dev/tty")
}
