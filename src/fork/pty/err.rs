use crate::fd::DescriptorError;
use std::error::Error;
use std::fmt;

/// The alias `Result` learns `MasterError` possibility.

/// The enum `MasterError` defines the possible errors from constructor Master.
#[derive(Clone, Copy, Debug)]
pub enum PtyError {
    WaitpidFail,
    BadDescriptor(DescriptorError),
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
            PtyError::BadDescriptor(_) => "the descriptor as occured an error",
            PtyError::GrantptError => "the `grantpt` has a error, errnois set appropriately.",
            PtyError::UnlockptError => "the `grantpt` has a error, errnois set appropriately.",
            PtyError::PtsnameError => "the `ptsname` has a error",
        }
    }

    /// The function `cause` returns the lower-level cause of this error, if any.
    fn cause(&self) -> Option<&dyn Error> {
        match *self {
            PtyError::BadDescriptor(ref err) => Some(err),
            _ => None,
        }
    }
}

impl From<DescriptorError> for PtyError {
    fn from(value: DescriptorError) -> Self {
        PtyError::BadDescriptor(value)
    }
}
