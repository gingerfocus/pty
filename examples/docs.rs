use pty::fork::*;
use std::io::Read;
use std::process::Command;

fn main() {
    match Fork::from_ptmx().unwrap() {
        Fork::Parent(mut master) => {
            // Read output via PTY master
            let mut output = String::new();

            match master.pty.read_to_string(&mut output) {
                Ok(_nread) => println!("child tty is: {}", output.trim()),
                Err(e) => panic!("read error: {}", e),
            }
        }
        Fork::Child(_) => {
            // Child process just exec `tty`
            Command::new("tty").status().expect("could not execute tty");
        }
    }
}
