//!
//! This module provides convenient GLSL-like aliases for `vec`.
//!
//! # Examples
//!
//! ```rust
//! use rokoko::prelude::*;
//!
//! let v = ivec3::from([1, 2, 3]);
//! assert_eq!(v.into_array(), [1, 2, 3]);
//!
//! let v2 = fvec2::from([0.59, 0.664]);
//! assert_eq!(v2.into_array(), [0.59, 0.664]);
//! ```
//!

#![allow(non_camel_case_types)]

use super::*;

pub type bvec <const N: usize> = vec <bool, N>;
pub type bvec4 = bvec <4>;
pub type bvec3 = bvec <3>;
pub type bvec2 = bvec <2>;
pub type bvec1 = bvec <1>;

pub type ivec <const N: usize> = vec <i32, N>;
pub type ivec4 = ivec <4>;
pub type ivec3 = ivec <3>;
pub type ivec2 = ivec <2>;
pub type ivec1 = ivec <1>;

pub type uvec <const N: usize> = vec <u32, N>;
pub type uvec4 = uvec <4>;
pub type uvec3 = uvec <3>;
pub type uvec2 = uvec <2>;
pub type uvec1 = uvec <1>;

pub type fvec <const N: usize> = vec <f32, N>;
pub type fvec4 = fvec <4>;
pub type fvec3 = fvec <3>;
pub type fvec2 = fvec <2>;
pub type fvec1 = fvec <1>;

pub type dvec <const N: usize> = vec <f64, N>;
pub type dvec4 = dvec <4>;
pub type dvec3 = dvec <3>;
pub type dvec2 = dvec <2>;
pub type dvec1 = dvec <1>;

pub type vec4 = fvec4;
pub type vec3 = fvec3;
pub type vec2 = fvec2;
pub type vec1 = fvec1;
