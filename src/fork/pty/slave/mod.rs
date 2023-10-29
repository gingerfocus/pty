mod err;

use crate::fd::Fd;

pub use self::err::SlaveError;
use std::os::unix::io::{AsRawFd, RawFd};

#[derive(Debug)]
pub struct Slave {
    pty: Fd,
}

impl Slave {
    /// The constructor function `new` prepares and returns the Slave interface.
    pub fn new(path: *const ::libc::c_char) -> Result<Self, SlaveError> {
        if unsafe { libc::setsid() } == -1 {
            return Err(SlaveError::SetSidFail);
        }

        let fd = Fd::open(path, libc::O_RDWR, None)?;

        fd.dup2(libc::STDIN_FILENO)?;
        fd.dup2(libc::STDOUT_FILENO)?;
        fd.dup2(libc::STDERR_FILENO)?;

        Ok(Slave { pty: fd })
    }
}

impl AsRawFd for Slave {
    /// The accessor function `as_raw_fd` returns the fd.
    fn as_raw_fd(&self) -> RawFd {
        self.pty.as_raw_fd()
    }
}
