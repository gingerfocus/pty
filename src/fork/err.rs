use crate::fd::DescriptorError;
use std::error::Error;
use std::fmt;

use super::pty::{MasterError, PtyError, SlaveError};

/// The enum `ForkError` defines the possible errors from constructor Fork.
#[derive(Clone, Copy, Debug)]
pub enum ForkError {
    /// The path specified could not be used
    BadPath,
    /// Can't creates the child.
    Failure,
    /// Can't set the id group.
    SetsidFail,
    /// Can't suspending the calling process.
    WaitpidFail,
    /// The Master had a error.
    BadMaster(MasterError),
    /// The Slave had a error.
    BadSlave(SlaveError),
    /// A file descriptor erro occured.
    BadDescriptor(DescriptorError),
    /// The pty had an error somewhere
    PtyFailure(PtyError),
}

impl fmt::Display for ForkError {
    /// The function `fmt` formats the value using the given formatter.

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", ::errno::errno())
    }
}

impl Error for ForkError {
    /// The function `description` returns a short description of the error.
    fn description(&self) -> &str {
        match *self {
            ForkError::BadPath => {
                "Your path couldn't be converted into a [`CString`]. this is \
                 likely beacuse it contained unicode or null bytes somewhere \
                 in the middle."
            }
            ForkError::Failure => {
                "On failure, -1 is returned in the parent,no child process is created, and errno \
                 isset appropriately."
            }
            ForkError::SetsidFail => {
                "fails if the calling process is alreadya process group leader."
            }
            ForkError::WaitpidFail => "Can't suspending the calling process.",
            ForkError::BadMaster(_) => "the master as occured an error",
            ForkError::BadSlave(_) => "the slave as occured an error",
            ForkError::BadDescriptor(_) => "the file descriptor had an error",
            ForkError::PtyFailure(_) => "the pty had an error",
        }
    }

    /// The function `cause` returns the lower-level cause of this error, if any.
    fn cause(&self) -> Option<&dyn Error> {
        match *self {
            ForkError::BadMaster(ref err) => Some(err),
            ForkError::BadSlave(ref err) => Some(err),
            ForkError::BadDescriptor(ref err) => Some(err),
            ForkError::PtyFailure(ref err) => Some(err),
            _ => None,
        }
    }
}

impl From<MasterError> for ForkError {
    fn from(value: MasterError) -> Self {
        ForkError::BadMaster(value)
    }
}

impl From<PtyError> for ForkError {
    fn from(value: PtyError) -> Self {
        ForkError::PtyFailure(value)
    }
}
