//!
//! This module provides different mathematical types and functions.
//!
//! # no_std
//!
//! This module is `#![no_std]`-friendly, i.e. it does not require `std`.
//!

use crate::*;

cfg_if::cfg_if! {
    if #[cfg(feature = "math")] {
        pub mod vec;
    } else {
        /// Stub.
        pub type vec <T, const N: usize> = [T; N];
    }
}
