#![cfg_attr(not(any(

)), no_std)]

#![cfg_attr(feature = "nightly", feature(
    const_trait_impl,
    const_mut_refs,
    const_ptr_read,
    const_ptr_write,
    const_deref,
    const_refs_to_cell,
    const_convert,
    auto_traits,
    negative_impls,
    unboxed_closures,
    fn_traits
))]

extern crate cfg_if;
extern crate rokoko_macro;
extern crate alloc;

pub(crate) use rokoko_macro::nightly;

pub mod math;
pub mod prelude;
