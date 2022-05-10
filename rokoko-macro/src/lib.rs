//!
//! This crate provides macros for usage in `rokoko` crate, both in
//! lib- and user-spaces.
//!

use proc_macro::*;

pub(crate) mod tools;
pub(crate) mod wb_statics;

include!("lib/mod.rs");
