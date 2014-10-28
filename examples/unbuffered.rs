// examples/unbuffered.rs
extern crate libc;
extern crate termios;

use std::io::stdio;
use termios::{TCSANOW, Clear, Termios};

// a.k.a. "Ctrl + D"
const END_OF_TRANSMISSION: u8 = 4;

fn main() {
    let old_termios = Termios::fetch(libc::STDIN_FILENO).unwrap();
    let mut new_termios = old_termios;

    // Disable line buffering
    new_termios.lflag.clear(termios::local::ICANON);

    // Disable echo
    new_termios.lflag.clear(termios::local::ECHO);

    new_termios.update(libc::STDIN_FILENO, TCSANOW).unwrap();

    for byte in stdio::stdin().bytes() {
        match byte {
            Err(e) => fail!("{}", e),
            Ok(END_OF_TRANSMISSION) => break,
            Ok(byte) => println!("Got {}", byte),
        }
    }

    // Restore `old_termios`
    old_termios.update(libc::STDIN_FILENO, TCSANOW).unwrap();
}
