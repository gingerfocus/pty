extern crate libc;
extern crate pty;

use self::pty::prelude::*;

use std::io::Read;
use std::process::{Command, Stdio};

fn main() {
    match Fork::from_ptmx().unwrap() {
        Fork::Parent(master) => master_main(master),
        Fork::Child(_) => {
            let _ = Command::new("tty").status();
        }
    }
}

fn master_main(mut master: Master) {
    let mut string = String::new();

    master.pty.read_to_string(&mut string).unwrap();

    let output = Command::new("tty")
        .stdin(Stdio::inherit())
        .output()
        .unwrap()
        .stdout;
    let output_str = String::from_utf8_lossy(&output);

    let parent_tty = output_str.trim();
    let child_tty = string.trim();

    println!(
        "child_tty(\"{}\")[{}] != \"{}\" => {}",
        child_tty,
        child_tty.len(),
        "",
        child_tty != ""
    );
    assert!(child_tty != "");
    assert!(child_tty != parent_tty);

    let mut parent_tty_dir: Vec<&str> = parent_tty.split("/").collect();
    let mut child_tty_dir: Vec<&str> = child_tty.split("/").collect();

    parent_tty_dir.pop();
    child_tty_dir.pop();

    assert_eq!(parent_tty_dir, child_tty_dir);
}
