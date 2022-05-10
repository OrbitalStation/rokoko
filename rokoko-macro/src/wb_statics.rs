//!
//! This module provides statics that collect info during
//! `data` and `events` sections and release in `create`
//!

use core::mem::take;

static mut TRAITS: String = String::new();
static mut LIFETIMES: String = String::new();

pub fn add_trait(ty: String) {
    unsafe {
        TRAITS.push_str(&ty);
        TRAITS.push('+')
    }
}

pub fn add_lifetimes(ty: String) {
    unsafe {
        LIFETIMES.push_str(&ty)
    }
}

pub fn traits() -> String {
    unsafe { take(&mut TRAITS) }
}

pub fn lifetimes() -> String {
    unsafe { take(&mut LIFETIMES) }
}
