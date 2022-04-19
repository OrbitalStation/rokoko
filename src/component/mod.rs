//!
//! This module provides structs `Empty`, `With` and trait `Component`
//! for convenient turning on/off fields of structs.
//!
//! # no_std
//!
//! This module is `#![no_std]`-friendly, i.e. it does not require `std`.
//!
//! # Examples
//!
//! ```rust
//! use rokoko::component::{Component, With, Empty};
//!
//! ///
//! /// `Copy` + `Clone` is required for now,
//! /// will be replaced by `!Drop` when it is stable.
//! ///
//! #[derive(Copy, Clone)]
//! struct Capacity(usize);
//!
//! struct VecWithConditionalCapacity <T, W: Component> {
//!     ptr: *const T,
//!     len: usize,
//!     component: W
//! }
//!
//! impl <T, W: Component> VecWithConditionalCapacity <T, W> {
//!     // ...
//!
//!     pub fn push(&mut self, x: T) {
//!         if let Some(capacity) = self.component.get_mut::<Capacity>() {
//!             // We have capacity, do some stuff with it
//!         } else {
//!             // We don't have capacity, reserve 1 additional element
//!         }
//!     }
//!
//!     // ...
//! }
//!
//! // Size of vec without capacity is less than of vec with capacity
//! use std::mem::size_of;
//! assert!(size_of::<VecWithConditionalCapacity <i32, Empty>>() < size_of::<VecWithConditionalCapacity <i32, With <Capacity>>>());
//! ```
//!

use crate::nightly;
use core::any::TypeId;

///
/// Type that implements `Component` is a container for types
/// and thus can give a type back.
///
/// # FIXME
///
/// The `Copy + Clone` bounds are needed only to prevent dropping
/// in constant function, replace it with `!Drop` when it becomes stable.
///
/// # Examples
///
/// For the examples check module documentation.
///
pub trait Component: Copy + Clone {
    ///
    /// Get a reference tp the type `T`(if contains)
    ///
    fn get <T: 'static> (&self) -> Option <&T>;

    ///
    /// Get a mutable reference tp the type `T`(if contains)
    ///
    fn get_mut <T: 'static> (&mut self) -> Option <&mut T>;
}

///
/// Empty container, literally contains no type.
///
/// Used as a starter in long chains or as a terminator in `With`
///
#[derive(Copy, Clone)]
pub struct Empty;

#[nightly(const)]
impl Component for Empty {
    #[inline]
    fn get <T: 'static> (&self) -> Option <&T> {
        None
    }

    #[inline]
    fn get_mut <T: 'static> (&mut self) -> Option <&mut T> {
        None
    }
}

///
/// The type-level container.
///
/// Used as a middle stage in long chains.
///
#[derive(Copy, Clone)]
pub struct With <D: 'static + Copy + Clone, N: Component = Empty> {
    data: D,
    next: N
}

///
/// This function is needed only to then compare
/// `TypeId`s in a const context, since `TypeId`'s `PartialEq`
/// is not `const`.
///
#[nightly(const)]
fn id <T: 'static> () -> u64 {
    // SAFETY: safe because we know that there is an `u64` inside of `TypeId`
    unsafe { core::mem::transmute(TypeId::of::<T>()) }
}

#[nightly(const(N: Component))]
impl <D: 'static + Copy + Clone, N: Component> Component for With <D, N> {
    fn get <T: 'static> (&self) -> Option <&T> {
        if id::<T>() == id::<D>() {
            // SAFETY: safe because we just ensured that `D` and `T` are same
            Some(unsafe { &*(&self.data as *const D as *const T) })
        } else {
            self.next.get::<T>()
        }
    }

    #[inline]
    fn get_mut <T: 'static> (&mut self) -> Option <&mut T> {
        unsafe { core::mem::transmute(self.get::<T>()) }
    }
}
