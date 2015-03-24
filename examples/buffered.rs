use std::io::{Read, self};

fn main() {
    let stdin = io::stdin();

    for byte in stdin.lock().bytes() {
        match byte {
            Err(e) => panic!("{}", e),
            Ok(byte) => println!("Got {}", byte),
        }
    }
}
