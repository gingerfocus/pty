mod err;

pub use self::err::DescriptorError;
use std::os::{fd::AsRawFd, unix::io::RawFd};

#[derive(Debug, Clone)]
pub struct Fd {
    inner: RawFd,
}

impl Fd {
    /// The constructor function `open` opens the path
    /// and returns the fd.
    pub fn open(
        path: *const libc::c_char,
        flag: libc::c_int,
        mode: Option<libc::c_int>,
    ) -> Result<Self, DescriptorError> {
        match unsafe { libc::open(path, flag, mode.unwrap_or(0)) } {
            -1 => Err(DescriptorError::CantOpen),
            fd => Ok(Fd { inner: fd }),
        }
    }

    pub fn dup2(&self, dst: RawFd) -> Result<libc::c_int, DescriptorError> {
        match unsafe { libc::dup2(self.as_raw_fd(), dst) } {
            -1 => Err(DescriptorError::Dup2Error),
            d => Ok(d),
        }
    }

    /// Change UID and GID of slave pty associated with master pty whose
    /// fd is provided, to the real UID and real GID of the calling thread.
    pub fn grantpt(&self) -> Result<libc::c_int, DescriptorError> {
        match unsafe { libc::grantpt(self.as_raw_fd()) } {
            -1 => Err(DescriptorError::GrantPtError),
            c => Ok(c),
        }
    }

    /// Unlock the pty associated with this fd.
    pub fn unlockpt(&self) -> Result<libc::c_int, DescriptorError> {
        match unsafe { libc::unlockpt(self.as_raw_fd()) } {
            -1 => Err(DescriptorError::UnlockPtError),
            c => Ok(c),
        }
    }

    /// The function `close` leaves the fd.
    fn close(&self) -> Result<(), DescriptorError> {
        match unsafe { libc::close(self.inner) } {
            -1 => Err(DescriptorError::CantClose),
            _ => Ok(()),
        }
    }
}

impl AsRawFd for Fd {
    fn as_raw_fd(&self) -> RawFd {
        self.inner
    }
}

impl Drop for Fd {
    /// The destructor function `drop` call the method `close`
    /// and panic if a error is occurred.
    fn drop(&mut self) {
        if self.close().is_err() {
            unimplemented!();
        }
    }
}
