#![feature(phase)]

#[phase(plugin)]
extern crate bindgen;

// CUT HERE
#[cfg(target_os = "linux")]
bindgen!("linux.h")
#[cfg(target_os = "macos")]
bindgen!("darwin.h")
