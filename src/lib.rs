#![cfg_attr(not(std), no_std)]

#![cfg_attr(nightly, feature(
    const_trait_impl,
    const_mut_refs,
    const_ptr_read,
    const_ptr_write,
    const_deref,
    const_refs_to_cell,
    const_convert,
    const_type_id,
    auto_traits,
    negative_impls,
    unboxed_closures,
    fn_traits
))]

#[cfg(std)]
pub(crate) use std as core;

extern crate cfg_if;

#[cfg(feature = "window")]
extern crate winit;

#[cfg(feature = "window")]
extern crate raw_window_handle;

#[doc(hidden)]
pub extern crate rokoko_macro;
pub use rokoko_macro::nightly;

#[cfg(feature = "window")]
pub mod window;

pub mod math;

pub mod prelude;
