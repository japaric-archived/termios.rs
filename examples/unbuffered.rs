#![feature(io)]
#![feature(libc)]

extern crate libc;
extern crate termios;

use std::io::{ReadExt, self};

use termios::prelude::*;

// a.k.a. "Ctrl + D"
const END_OF_TRANSMISSION: u8 = 4;

fn main() {
    let old_termios = Termios::fetch(libc::STDIN_FILENO).unwrap();
    let mut new_termios = old_termios;

    // Disable line buffering
    new_termios.clear(local::Flag::ICANON);

    // Disable echo
    new_termios.clear(local::Flag::ECHO);

    new_termios.update(libc::STDIN_FILENO, When::Now).unwrap();

    let stdin = io::stdin();
    for byte in stdin.lock().bytes() {
        match byte {
            Err(e) => panic!("{}", e),
            Ok(END_OF_TRANSMISSION) => break,
            Ok(byte) => println!("Got {}", byte),
        }
    }

    // Restore `old_termios`
    old_termios.update(libc::STDIN_FILENO, When::Now).unwrap();
}
