#![feature(io)]

use std::old_io::{BytesReader, stdio};

fn main() {
    for byte in stdio::stdin().bytes() {
        match byte {
            Err(e) => panic!("{}", e),
            Ok(byte) => println!("Got {}", byte),
        }
    }
}
