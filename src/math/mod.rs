//!
//! This module provides different mathematical types and functions
//!

use crate::*;

cfg_if::cfg_if! {
    if #[cfg(feature = "math")] {
        pub mod vec;
    } else {
        // Stub.
        pub type vec <T, const N: usize> = [T; N];
    }
}
