use crate::pty::Pty;

#[derive(Debug)]
pub struct Master {
    pub pty: Pty,
    child: libc::pid_t,
}

impl Master {
    /// Construct a new master process with the given pty and child pid
    pub fn new(pty: Pty, child: libc::pid_t) -> Self {
        Self { pty, child }
    }

    /// Waits until the child terminates.
    pub fn wait(&self) -> Result<libc::pid_t, MasterError> {
        self.wait_with_exit().map(|(pid, _exit)| pid)
    }

    pub fn wait_with_exit(&self) -> Result<(libc::pid_t, Option<i32>), MasterError> {
        loop {
            let mut status = 0;
            match unsafe { libc::waitpid(self.child, &mut status, 0) } {
                0 => continue,
                -1 => return Err(MasterError::WaitpidFail),
                pid => {
                    if libc::WIFEXITED(status) {
                        return Ok((pid, Some(libc::WEXITSTATUS(status))));
                    } else {
                        return Ok((pid, None)); // self.child instead of pid
                    }
                }
            }
        }
    }
}


// use crate::fd::DescriptorError;
use std::error::Error;
use std::fmt;

/// The alias `Result` learns `MasterError` possibility.

/// The enum `MasterError` defines the possible errors from constructor Master.
#[derive(Clone, Copy, Debug)]
pub enum MasterError {
    WaitpidFail,
    // BadDescriptor(DescriptorError),
    PtsnameError,
}

impl fmt::Display for MasterError {
    /// The function `fmt` formats the value using the given formatter.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", ::errno::errno())
    }
}

impl Error for MasterError {
    /// The function `description` returns a short description of the error.
    fn description(&self) -> &str {
        match *self {
            MasterError::WaitpidFail => "`libc::waitpid` returned an error",
            // MasterError::BadDescriptor(_) => "the descriptor as occured an error",
            MasterError::PtsnameError => "the `ptsname` has a error",
        }
    }

    /// The function `cause` returns the lower-level cause of this error, if any.
    fn cause(&self) -> Option<&dyn Error> {
        match *self {
            // MasterError::BadDescriptor(ref err) => Some(err),
            _ => None,
        }
    }
}
