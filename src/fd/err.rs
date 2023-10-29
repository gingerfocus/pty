use std::error::Error;
use std::fmt;

/// The enum `DescriptorError` defines the possible errors
/// from constructor Descriptor.
#[derive(Clone, Copy, Debug)]
pub enum DescriptorError {
    /// Can't open.
    CantOpen,
    /// Can't closed.
    CantClose,
    /// the dup2 function failed
    Dup2Error,
    /// the grantpt function failed
    GrantPtError,
    /// the unlockpt function failed
    UnlockPtError,
}

impl fmt::Display for DescriptorError {
    /// The function `fmt` formats the value using the given formatter.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", ::errno::errno())
    }
}

impl Error for DescriptorError {
    /// The function `description` returns a short description of the error.
    fn description(&self) -> &str {
        match *self {
            DescriptorError::CantOpen => "can't open the fd",
            DescriptorError::CantClose => "can't close the fd",
            DescriptorError::Dup2Error => "`libc::dup2` returned an error",
            DescriptorError::GrantPtError => "",
            DescriptorError::UnlockPtError => "",
        }
    }

    /// The function `cause` returns the lower-level cause of this error, if any.
    fn cause(&self) -> Option<&dyn Error> {
        None
    }
}
