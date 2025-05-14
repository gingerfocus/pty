use crate::master::MasterError;

use std::ffi::CStr;
use std::os::unix::io::AsRawFd;
use std::path::Path;
use std::{fs, io};

#[derive(Debug)]
pub struct Pty {
    fd: fs::File,
}

/// Change UID and GID of slave pty associated with master pty whose
/// fd is provided, to the real UID and real GID of the calling thread.
pub fn grantpt(f: &fs::File) -> Result<(), ()> {
    match unsafe { libc::grantpt(f.as_raw_fd()) } {
        0 => Ok(()),
        _ => todo!(), // Err(DescriptorError::GrantPtError),
    }
}

/// Unlock the pty associated with this fd.
pub fn unlockpt(f: &fs::File) -> Result<(), ()> {
    match unsafe { libc::unlockpt(f.as_raw_fd()) } {
        0 => Ok(()),
        _ => todo!(), // Err(DescriptorError::UnlockPtError),
    }
}

impl Pty {
    pub fn new(path: impl AsRef<Path>) -> Result<Self, PtyError> {
        let f = fs::OpenOptions::new()
            .read(true)
            .write(true)
            .open(path)
            .unwrap();

        grantpt(&f).unwrap();
        unlockpt(&f).unwrap();

        Ok(Self { fd: f })
    }

    /// Returns a pointer to a static buffer, which will be overwritten on
    /// subsequent calls.
    pub fn ptsname(&self) -> Result<String, MasterError> {
        match unsafe { libc::ptsname(self.fd.as_raw_fd()) } {
            c if c.is_null() => Err(MasterError::PtsnameError),
            c => {
                let s = unsafe { CStr::from_ptr(c) };
                let s = s.to_str().unwrap().to_owned();
                Ok(s)
            }
        }
    }
}

impl io::Read for Pty {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.fd.read(buf)
    }
}

impl io::Write for Pty {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.fd.write(buf)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.fd.flush()
    }
}

// use crate::fd::DescriptorError;
use std::error::Error;
use std::fmt;

/// The alias `Result` learns `MasterError` possibility.

/// The enum `MasterError` defines the possible errors from constructor Master.
#[derive(Clone, Copy, Debug)]
pub enum PtyError {
    WaitpidFail,
    // BadDescriptor(DescriptorError),
    GrantptError,
    UnlockptError,
    PtsnameError,
}

impl fmt::Display for PtyError {
    /// The function `fmt` formats the value using the given formatter.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", ::errno::errno())
    }
}

impl Error for PtyError {
    /// The function `description` returns a short description of the error.
    fn description(&self) -> &str {
        match *self {
            PtyError::WaitpidFail => "`libc::waitpid` returned an error",
            // PtyError::BadDescriptor(_) => "the descriptor as occured an error",
            PtyError::GrantptError => "the `grantpt` has a error, errnois set appropriately.",
            PtyError::UnlockptError => "the `grantpt` has a error, errnois set appropriately.",
            PtyError::PtsnameError => "the `ptsname` has a error",
        }
    }

    /// The function `cause` returns the lower-level cause of this error, if any.
    fn cause(&self) -> Option<&dyn Error> {
        match *self {
            // PtyError::BadDescriptor(ref err) => Some(err),
            _ => None,
        }
    }
}

// impl From<DescriptorError> for PtyError {
//     fn from(value: DescriptorError) -> Self {
//         PtyError::BadDescriptor(value)
//     }
// }
