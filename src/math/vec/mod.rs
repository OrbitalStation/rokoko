//!
//! This module provides type `vec`
//!
//! The `vec` type is a type with properties similar to those
//! of `GLSL vec`.
//!
//! # WARNING(On Nightly)
//!
//! To use a `const` fn/trait impl on `nightly` Rust, for example this:
//! ```rust,nightly,compile_fail
//! use rokoko::prelude::*;
//!
//! const VEC: ivec2 = ivec2::from([1, 2]);
//! ```
//! You need to enable corresponding feature, or else it won't compile!!!
//!
//! ```rust,nightly
//! // `from` is fn from `From` trait, so `const_trait_impl` here
//! #![feature(const_trait_impl)]
//!
//! use rokoko::prelude::*;
//!
//! const VEC: ivec2 = ivec2::from([1, 2]);
//! ```
//!
//! # Examples
//!
//! ```rust
//! use rokoko::prelude::*;
//!
//! let a = vec::<i32, 2>::from([1, 2]);
//!
//! // Aliases are also supported
//! let b = ivec2::from([3, 4]);
//!
//! // Operators are overloaded
//! assert_eq!(a + b, ivec2::from([4, 6]));
//!
//! // Possible from single value
//! assert_eq!(a - b, ivec2::single(-2));
//!
//! ```
//!

mod ops;

pub mod new;

pub mod alias;
pub use self::alias::*;

use crate::*;

///
/// The main type of the crate.
///
/// See module documentation for more information.
///
/// Not camel-case `Vec` to show it is among the basic types
///
#[allow(non_camel_case_types)]
pub struct vec <T, const N: usize> ([T; N]);

///
/// `vec` is Clone if `T` is Clone
///
impl <T: Clone, const N: usize> Clone for vec <T, N> {
    #[inline]
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

///
/// `vec` is Copy if `T` is Copy
///
impl <T: Copy, const N: usize> Copy for vec <T, N> {}

///
/// `vec` is Eq if `T` is Eq
///
impl <T: Eq, const N: usize> Eq for vec <T, N> where Self: PartialEq {}

///
/// `vec` implements Default if `T` is Default and Clone.
///
/// # Constness
///
/// Const when `T` is `~const Default` and `nightly` feature is enabled
///
#[nightly(const(T: Default))]
impl <T: Default + Copy, const N: usize> Default for vec <T, N> {
    #[inline]
    fn default() -> Self {
        Self([T::default(); N])
    }
}

impl <T, const N: usize> vec <T, N> {
    ///
    /// Creates a new vec from an array.
    ///
    /// # Examples
    ///
    /// ```
    /// use rokoko::prelude::*;
    ///
    /// let vec = vec::from_array([1, 2, 3]);
    ///
    /// assert_eq!(vec.into_array(), [1, 2, 3]);
    /// ```
    ///
    #[inline]
    pub const fn from_array(array: [T; N]) -> Self {
        Self(array)
    }

    ///
    /// Consumes `vec` and returns an array.
    ///
    /// # Constness
    ///
    /// Const when `nightly` feature is enabled
    ///
    /// # Examples
    ///
    /// ```
    /// use rokoko::prelude::*;
    ///
    /// let vec = vec::from_array([1, 2, 3]);
    ///
    /// assert_eq!(vec.into_array(), [1, 2, 3]);
    /// ```
    ///
    #[nightly(const)]
    #[inline]
    pub fn into_array(self) -> [T; N] {
        unsafe { core::ptr::read(&core::mem::ManuallyDrop::new(self).0) }
    }

    ///
    /// Returns a reference to an inner array.
    ///
    /// # Examples
    ///
    /// ```
    /// use rokoko::prelude::*;
    ///
    /// let vec = vec::from_array([1, 2, 3]);
    ///
    /// assert_eq!(vec.as_array(), &[1, 2, 3]);
    /// ```
    ///
    #[inline]
    pub const fn as_array(&self) -> &[T; N] {
        &self.0
    }

    ///
    /// Returns a mutable reference to an inner array.
    ///
    /// # Constness
    ///
    /// Const when `nightly` feature is enabled
    ///
    /// # Examples
    ///
    /// ```
    /// use rokoko::prelude::*;
    ///
    /// let mut vec = vec::from_array([1, 2, 3]);
    ///
    /// let array = vec.as_array_mut();
    ///
    /// array[1] = 12;
    ///
    /// assert_eq!(vec.as_array(), &[1, 12, 3]);
    /// ```
    ///
    #[inline]
    #[nightly(const)]
    pub fn as_array_mut(&mut self) -> &mut [T; N] {
        &mut self.0
    }
}

impl <T: Copy, const N: usize> vec <T, N> {
    ///
    /// Creates a new vec filled with `value`s.
    ///
    /// # Examples
    ///
    /// ```
    /// use rokoko::prelude::*;
    ///
    /// let vec = dvec3::single(4.0);
    ///
    /// assert_eq!(vec.into_array(), [4.0, 4.0, 4.0]);
    /// ```
    ///
    #[inline]
    #[nightly(const)]
    pub fn single(value: T) -> Self {
        Self([value; N])
    }
}

impl <T, const N: usize> vec <T, N> {
    ///
    /// Returns an uninitialized vec.
    ///
    /// # Safety
    ///
    /// Caller must guarantee that it will fill vec fully
    /// before using it.
    ///
    /// # Examples
    ///
    /// ```
    /// use rokoko::prelude::*;
    ///
    /// let vec = unsafe {
    ///     let mut uninit = bvec3::uninit();
    ///     uninit[0] = true;
    ///     uninit[1] = false;
    ///     uninit[2] = false;
    ///     uninit
    /// };
    ///
    /// assert_eq!(vec, vec::from_array([true, false, false]));
    /// ```
    ///
    #[inline]
    pub const unsafe fn uninit() -> Self {
        core::mem::MaybeUninit::uninit().assume_init()
    }
}
