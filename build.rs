//!
//! This build script:
//! - detects if a `nightly` toolchain is used
//! and provides `cfg(nightly)` to detect it.
//!
//! - provides `cfg(std)` to determine if
//! a `#![no_std]` can be used
//!

extern crate rustc_version;
use rustc_version::{version_meta, Channel};

// extern crate vulkano;
//
// fn is_vulkan_supported() -> bool {
//     vulkano::instance::Instance::new(Default::default()).is_ok()
// }

///
/// Determines if a `var` exists in current environment.
///
fn exists(var: &str) -> bool {
    std::env::var(var).is_ok()
}

fn main() {
    if let Channel::Nightly = version_meta().unwrap().channel {
        println!("cargo:rustc-cfg=nightly")
    }

    if exists("CARGO_FEATURE_WINDOW") {
        println!("cargo:rustc-cfg=std")
    }

    // if is_vulkan_supported() {
    //     println!("cargo:rustc-cfg=vulkan")
    // }
}
