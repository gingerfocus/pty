
use resu::{Context, Report, ResultExt};

use crate::pty::Pty;
pub use crate::master::{Master, MasterError};
pub use crate::slave::{Slave, SlaveError};
use resu::Result;
use std::fmt;
use std::path::Path;

#[derive(Debug)]
pub enum Fork {
    // Parent child's pid and master's pty.
    Parent(Master),
    // Child pid 0.
    Child(Slave),
}

impl Fork {
    /// The constructor function `new` forks the program
    /// and returns the current pid.
    pub fn new<P>(path: P) -> Result<Self, ForkError>
    where
        P: AsRef<Path>,
    {
        // let path = path.as_ref().to_str().ok_or(ForkError::BadPath)?;
        // let path = CString::new(path).map_err(|_| ForkError::BadPath)?;

        let pty = Pty::new(path).change_context(ForkError::PtyFailure)?;

        let name = pty.ptsname().change_context(ForkError::PtyFailure)?;

        match unsafe { libc::fork() } {
            -1 => Err(Report::new(ForkError::Failure)),
            0 => {
                let c = Slave::new(name).change_context(ForkError::BadSlave)?;
                Ok(Fork::Child(c))
            }
            pid => {
                let master = Master::new(pty, pid);
                Ok(Fork::Parent(master))
            }
        }
    }

    /// The constructor function `from_ptmx` forks the program
    /// and returns the current pid for a default PTMX's path.
    pub fn from_ptmx() -> Result<Self, ForkError> {
        Fork::new(crate::DEFAULT_PTMX)
    }

    /// Returns true if this is the parent process. False otherwise.
    pub fn is_parent(&self) -> bool {
        matches!(self, Fork::Parent(_))
    }

    /// Returns true if this is the child process. False otherwise.
    pub fn is_child(&self) -> bool {
        matches!(self, Fork::Child(_))
    }
}

/// The enum `ForkError` defines the possible errors from constructor Fork.
#[derive(Debug, Clone, Copy)]
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
    BadMaster,
    /// The Slave had a error.
    BadSlave,
    /// A file descriptor erro occured.
    BadDescriptor,
    /// The pty had an error somewhere
    PtyFailure,
}

impl fmt::Display for ForkError {
    /// The function `fmt` formats the value using the given formatter.

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ForkError::BadPath => f.write_str(
                "Your path couldn't be converted into a [`CString`]. this is \
                 likely beacuse it contained unicode or null bytes somewhere \
                 in the middle.",
            ),
            ForkError::Failure => f.write_str(
                "On failure, -1 is returned in the parent,no child process is created, and errno \
                 isset appropriately.",
            ),
            ForkError::SetsidFail => {
                f.write_str("fails if the calling process is alreadya process group leader.")
            }
            ForkError::WaitpidFail => f.write_str("Can't suspending the calling process."),
            ForkError::BadMaster => f.write_str("the master as occured an error"),
            ForkError::BadSlave => f.write_str("the slave as occured an error"),
            ForkError::BadDescriptor => f.write_str("the file descriptor had an error"),
            ForkError::PtyFailure => f.write_str("the pty had an error"),
        }
    }
}

impl Context for ForkError {}
