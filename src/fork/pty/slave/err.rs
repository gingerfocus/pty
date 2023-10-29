use crate::fd::DescriptorError;
use std::error::Error;
use std::fmt;

/// The enum `SlaveError` defines the possible errors from constructor Slave.
#[derive(Clone, Copy, Debug)]
pub enum SlaveError {
    SetSidFail,
    BadDescriptor(DescriptorError),
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
            SlaveError::BadDescriptor(_) => "the descriptor as occured an error",
        }
    }

    /// The function `cause` returns the lower-level cause of this error, if any.
    fn cause(&self) -> Option<&dyn Error> {
        match *self {
            SlaveError::BadDescriptor(ref err) => Some(err),
            _ => None,
        }
    }
}

impl From<DescriptorError> for SlaveError {
    fn from(value: DescriptorError) -> Self {
        SlaveError::BadDescriptor(value)
    }
}
