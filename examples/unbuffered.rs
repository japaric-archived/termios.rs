#![feature(libc)]
#![feature(old_io)]

extern crate libc;
extern crate termios;

use termios::prelude::*;

use std::old_io::{BytesReader, stdio};

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

    for byte in stdio::stdin().bytes() {
        match byte {
            Err(e) => panic!("{}", e),
            Ok(END_OF_TRANSMISSION) => break,
            Ok(byte) => println!("Got {}", byte),
        }
    }

    // Restore `old_termios`
    old_termios.update(libc::STDIN_FILENO, When::Now).unwrap();
}
