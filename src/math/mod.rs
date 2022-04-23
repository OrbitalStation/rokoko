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
        pub mod vec {
            ///
            /// Aliases could be used even without `math` feature, so they do.
            ///
            #[path = "../vec/alias.rs"]
            pub mod alias;
            pub use self::alias::*;

            /// Stub.
            #[allow(non_camel_case_types)]
            pub type vec <T, const N: usize> = [T; N];
        }
    }
}
