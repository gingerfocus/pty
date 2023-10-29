//! # pty
//!
//! [![Crate][crate-badge]][crate] [![docs-badge][]][docs] [![license-badge][]][license]
//!
//! [crate-badge]: https://img.shields.io/badge/crates.io-v0.3.0-orange.svg?style=flat-square
//! [crate]: https://crates.io/crates/pty
//!
//! [docs-badge]: https://img.shields.io/badge/API-docs-blue.svg?style=flat-square
//! [docs]: https://docs.rs/pty/latest/pty/
//!
//! [license-badge]: https://img.shields.io/badge/license-MIT-blue.svg?style=flat-square
//! [license]: https://github.com/focus172/pty/blob/master/LICENSE.txt
//!
//! The `pty` crate provides `pty::fork()`. That makes a parent process fork with new pseudo-terminal (PTY).
//!
//! This crate depends on followings:
//!
//! * `libc` library
//! * POSIX environment
//!
//! ## Usage
//!
//! Add this to your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! pty = "0.3"
//! ```
//!
//! ### pty::fork()
//!
//! This function returns `pty::Child`. It represents the child process and its PTY.
//!
//! For example, the following code spawns `tty(1)` command by `pty::fork()` and outputs the result of the command.
//!
//! ```rust
//! use std::io::Read;
//! use std::process::Command;
//!
//! let mut master = pty::fork(|child| {
//!     // Child process just execs `tty`
//!     Command::new("tty").status().expect("could not execute tty");
//!     // Recomended way to exit child process but `panic!()` and
//!     // `std::process::exit()` are also fine. Just be a kind soul and call
//!     // `drop(child)` before you leave if you either of those two options.
//!     return 0;
//! });
//!
//! // Read output via PTY master
//! let mut output = String::new();
//! let _ = master.pty.read_to_string(&mut output).unwrap();
//! println!("child tty is: {}", output.trim());
//! ```

#![deny(
    missing_debug_implementations,
    missing_copy_implementations,
    trivial_casts,
    trivial_numeric_casts,
    unused_qualifications
)]
#![feature(never_type)]

mod fd;
pub mod fork;
pub mod prelude;

const DEFAULT_PTMX: &str = "/dev/ptmx";

/// Forks calling the clojure on the child retruning the parent.
pub fn fork<F>(f: F) -> fork::Master
where
    F: FnOnce(fork::Slave) -> i32,
{
    let fork = fork::Fork::from_ptmx().unwrap();
    match fork {
        fork::Fork::Parent(m) => m,
        fork::Fork::Child(c) => {
            let code = f(c);
            std::process::exit(code)
        }
    }
}
