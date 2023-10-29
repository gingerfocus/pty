# pty
The `pty` crate provides `pty::fork()`. That makes a parent process fork with new pseudo-terminal (PTY).

This crate depends on followings:
* `libc` library
* POSIX environment

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
pty = "0.3"
```

### pty::fork()

This function returns `pty::Child`. It represents the child process and its PTY.

For example, the following code spawns `tty(1)` command by `pty::fork()` and outputs the result of the command.

```rust
use std::io::Read;
use std::process::Command;

fn main() {
    let mut master = pty::fork(|child| {
        // Child process just execs `tty`
        Command::new("tty").status().expect("could not execute tty");
        // Recommended way to exit child process but `panic!()` and
        // `std::process::exit()` are also fine. Just be a kind soul and call
        // `drop(child)` before you leave if you either of those two options.
        return 0;
    });

    // Read output via PTY master
    let mut output = String::new();
    let _ = master.pty.read_to_string(&mut output).unwrap();
    println!("child tty is: {}", output.trim());
}
```

When run this, we get new PTY in the child process.

```
$ tty
/dev/pts/5
$ cargo run
    Running `target/debug/example`
child tty is: /dev/pts/8
```

## Documentation

API documentation for latest version: http://hibariya.github.io/pty-rs/pty/index.html

## License

Copyright (c) 2015 Hika Hibariya

Distributed under the [MIT License](LICENSE.txt).
