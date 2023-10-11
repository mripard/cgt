#![warn(missing_debug_implementations)]
#![warn(missing_docs)]
#![warn(rust_2018_idioms)]
#![warn(rust_2021_compatibility)]
#![deny(clippy::all)]
#![deny(clippy::pedantic)]
#![allow(clippy::unnecessary_wraps)]
#![doc = include_str!("../README.md")]

use cgt_core::{run_all, DeviceSpecifier, RunResult};

mod tests;

fn main() -> RunResult {
    run_all(DeviceSpecifier::ModuleName(String::from("vkms")))
}
