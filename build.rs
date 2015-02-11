#![feature(env)]
#![feature(io)]
#![feature(path)]

extern crate bindgen;

use bindgen::{Bindings, BindgenOptions};
use std::default::Default;
use std::env;
use std::old_io::fs::File;

#[cfg(target_os = "freebsd")]
const HEADER_FILE: &'static str = "freebsd.h";
#[cfg(target_os = "linux")]
const HEADER_FILE: &'static str = "linux.h";
#[cfg(target_os = "macos")]
const HEADER_FILE: &'static str = "macos.h";

fn main() {
    let src_dir = Path::new(env::var_string("CARGO_MANIFEST_DIR").unwrap()).join("src");
    let include_file = src_dir.join(HEADER_FILE);
    let mut options: BindgenOptions = Default::default();
    options.clang_args.push(include_file.as_str().unwrap().to_string());
    let bindings = Bindings::generate(&options, None, None).unwrap().to_string();
    File::create(&src_dir.join("raw/ffi.rs")).unwrap().write_all(bindings.as_bytes()).unwrap();
}
