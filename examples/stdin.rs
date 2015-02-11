#![feature(libc)]

extern crate libc;
extern crate termios;

use termios::prelude::*;

fn main() {
    println!("{:?}", Termios::fetch(libc::STDIN_FILENO).unwrap());
}
