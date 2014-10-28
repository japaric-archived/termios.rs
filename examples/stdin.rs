// examples/stdin.rs
extern crate libc;
extern crate termios;

use termios::Termios;

fn main() {
    println!("{}", Termios::fetch(libc::STDIN_FILENO).unwrap());
}
