//!
//! This build script detects if a `nightly` toolchain is used
//! and provides `cfg(nightly)` to detect it
//!

extern crate rustc_version;
use rustc_version::{version_meta, Channel};

fn main() {
    if let Channel::Nightly = version_meta().unwrap().channel {
        println!("cargo:rustc-cfg=nightly")
    }
}
