//!
//! This module provides convenient GLSL-like aliases for `vec`.
//!
//! Example: TODO!
//!

use super::*;

rokoko_macro::impl_aliases_for_vec!(4;
    b = bool,
    i = i32,
    u = u32,
    f = f32,
    d = f64
);
