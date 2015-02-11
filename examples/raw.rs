#![feature(libc)]

extern crate libc;
extern crate termios;

use termios::prelude::*;

fn main() {
    let mut termios = Termios::fetch(libc::STDIN_FILENO).unwrap();
    println!("Cooked:\n{:?}", termios);
    termios.make_raw();
    println!("\nRaw:\n{:?}", termios);
}
