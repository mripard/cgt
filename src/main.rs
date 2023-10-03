#![warn(missing_debug_implementations)]
#![warn(missing_docs)]
#![warn(rust_2018_idioms)]
#![warn(rust_2021_compatibility)]
#![deny(clippy::all)]
#![deny(clippy::pedantic)]
#![allow(clippy::unnecessary_wraps)]
#![doc = include_str!("../README.md")]

use std::path::PathBuf;

use cgt_core::run_all;

mod tests;

fn main() {
    run_all(&PathBuf::from("/dev/dri/card0"));
}
