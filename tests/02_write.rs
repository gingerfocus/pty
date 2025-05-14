extern crate libc;
extern crate pty;

use self::pty::prelude::*;

use std::io::prelude::*;
use std::process::Command;

fn read_line(master: &mut Master) -> String {
    let mut buf = [0];
    let mut res = String::new();

    while buf[0] as char != '\n' {
        master.pty.read(&mut buf).expect("cannot read 1 byte");
        res.push(buf[0] as char)
    }
    res
}

#[test]
fn subshell_write() {
    let fork = Fork::from_ptmx().unwrap();

    match fork {
        Fork::Parent(mut master) => {
            let _ = master.pty.write("echo readme!\n".to_string().as_bytes());

            read_line(&mut master); // this is the "echo readme!" we just sent
            read_line(&mut master); // this is the shell and "echo readme!" again
            let s = read_line(&mut master);

            // the are not equal bc of some shell nonsence
            assert_eq!(s.trim(), "readme!");
            let _ = master.pty.write("exit\n".to_string().as_bytes());
        }
        Fork::Child(_) => {
            let _ = Command::new("sh").env_clear().status();
        }
    }
}
