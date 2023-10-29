extern crate errno;
extern crate libc;
extern crate pty;

use self::pty::prelude::*;

use std::io::prelude::*;
use std::process::{Command, Stdio};
use std::string::String;

#[test]
fn command_read() {
    let fork = Fork::from_ptmx().unwrap();
    match fork {
        Fork::Parent(mut master) => {
            let mut string = String::new();

            master
                .pty
                .read_to_string(&mut string)
                .unwrap_or_else(|e| panic!("{}", e));

            let output = Command::new("tty")
                .stdin(Stdio::inherit())
                .output()
                .unwrap()
                .stdout;
            let output_str = String::from_utf8_lossy(&output);

            let parent_tty = output_str.trim();
            let child_tty = string.trim();

            assert!(child_tty != "");
            assert!(child_tty != parent_tty);

            // only compare if parent is tty
            // travis runs the tests without tty
            if parent_tty != "not a tty" {
                let mut parent_tty_dir: Vec<&str> = parent_tty.split("/").collect();
                let mut child_tty_dir: Vec<&str> = child_tty.split("/").collect();

                parent_tty_dir.pop();
                child_tty_dir.pop();

                assert_eq!(parent_tty_dir, child_tty_dir);
            }
        }
        Fork::Child(_) => {
            Command::new("tty").status().expect("could not execute tty");
        }
    }
}
