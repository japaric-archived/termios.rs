// examples/buffered.rs
use std::io::stdio;

fn main() {
    for byte in stdio::stdin().bytes() {
        match byte {
            Err(e) => fail!("{}", e),
            Ok(byte) => println!("Got {}", byte),
        }
    }
}
