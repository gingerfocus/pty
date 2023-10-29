mod err;

use libc;

use super::Pty;

pub use self::err::MasterError;

#[derive(Debug, Clone)]
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
