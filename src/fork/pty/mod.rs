mod master;
mod slave;

pub use self::master::{Master, MasterError};
pub use self::slave::{Slave, SlaveError};

pub use self::err::PtyError;
use crate::fd::Fd;
use std::io;
use std::os::unix::io::{AsRawFd, RawFd};

mod err;

#[derive(Debug, Clone)]
pub struct Pty {
    fd: Fd,
}

impl Pty {
    pub fn new(path: *const ::libc::c_char) -> Result<Self, PtyError> {
        let fd = Fd::open(path, libc::O_RDWR, None)?;
        fd.grantpt()?;
        fd.unlockpt()?;

        Ok(Self { fd })
    }

    /// Returns a pointer to a static buffer, which will be overwritten on
    /// subsequent calls.
    pub unsafe fn ptsname(&self) -> Result<*const libc::c_char, MasterError> {
        match libc::ptsname(self.as_raw_fd()) {
            c if c.is_null() => Err(MasterError::PtsnameError),
            c => Ok(c),
        }
    }
}

impl AsRawFd for Pty {
    /// The accessor function `as_raw_fd` returns the fd.
    fn as_raw_fd(&self) -> RawFd {
        self.fd.as_raw_fd()
    }
}

impl io::Read for Pty {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        unsafe {
            match libc::read(
                self.as_raw_fd(),
                buf.as_mut_ptr() as *mut libc::c_void,
                buf.len(),
            ) {
                -1 => Ok(0),
                len => Ok(len as usize),
            }
        }
    }
}

impl io::Write for Pty {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        match unsafe {
            libc::write(
                self.as_raw_fd(),
                buf.as_ptr() as *const libc::c_void,
                buf.len(),
            )
        } {
            -1 => Err(io::Error::last_os_error()),
            ret => Ok(ret as usize),
        }
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}
