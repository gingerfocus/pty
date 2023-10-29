mod err;
mod pty;

pub use self::err::ForkError;
use self::pty::Pty;
pub use self::pty::{Master, MasterError};
pub use self::pty::{Slave, SlaveError};
use libc;
use std::ffi::CString;
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
        let path = path.as_ref().to_str().ok_or(ForkError::BadPath)?;
        let path = CString::new(path).map_err(|_| ForkError::BadPath)?;

        let pty = Pty::new(path.as_ptr())?;

        let name = unsafe { pty.ptsname() }?;

        match unsafe { libc::fork() } {
            -1 => Err(ForkError::Failure),
            0 => match Slave::new(name) {
                Ok(c) => Ok(Fork::Child(c)),
                Err(e) => Err(ForkError::BadSlave(e)),
            },
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
