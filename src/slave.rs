use std::{
    fs,
    os::unix::io::{AsRawFd, RawFd},
    path::Path,
};

#[derive(Debug)]
pub struct Slave {
    pty: fs::File,
}

/// Takes the second argument and pipes all its output to the first argument.
pub fn dup2(f: &fs::File, dst: RawFd) -> Result<i32, ()> {
    match unsafe { libc::dup2(f.as_raw_fd(), dst) } {
        -1 => todo!(), // Err(DescriptorError::Dup2Error)
        fd => Ok(fd),
    }
}

impl Slave {
    /// The constructor function `new` prepares and returns the Slave interface.
    pub fn new(path: impl AsRef<Path>) -> Result<Self, SlaveError> {
        if unsafe { libc::setsid() } == -1 {
            return Err(SlaveError::SetSidFail);
        }
        let f = fs::OpenOptions::new()
            .read(true)
            .write(true)
            .open(path)
            .unwrap();

        dup2(&f, libc::STDIN_FILENO).unwrap();
        dup2(&f, libc::STDOUT_FILENO).unwrap();
        dup2(&f, libc::STDERR_FILENO).unwrap();

        Ok(Slave { pty: f })
    }
}

impl AsRawFd for Slave {
    /// The accessor function `as_raw_fd` returns the fd.
    fn as_raw_fd(&self) -> RawFd {
        self.pty.as_raw_fd()
    }
}

// use crate::fd::DescriptorError;
use std::error::Error;
use std::fmt;

/// The enum `SlaveError` defines the possible errors from constructor Slave.
#[derive(Clone, Copy, Debug)]
pub enum SlaveError {
    SetSidFail,
    // BadDescriptor(DescriptorError),
}

impl fmt::Display for SlaveError {
    /// The function `fmt` formats the value using the given formatter.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", ::errno::errno())
    }
}

impl Error for SlaveError {
    /// The function `description` returns a short description of the error.
    fn description(&self) -> &str {
        match *self {
            Self::SetSidFail => "the setsid function has failed",
            // SlaveError::BadDescriptor(_) => "the descriptor as occured an error",
        }
    }

    /// The function `cause` returns the lower-level cause of this error, if any.
    fn cause(&self) -> Option<&dyn Error> {
        match *self {
            // SlaveError::BadDescriptor(ref err) => Some(err),
            _ => None,
        }
    }
}

// impl From<DescriptorError> for SlaveError {
//     fn from(value: DescriptorError) -> Self {
//         SlaveError::BadDescriptor(value)
//     }
// }
