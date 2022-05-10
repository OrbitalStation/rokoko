//!
//! This module provides statics that collect info during
//! `data` and `events` sections and release in `create`
//!

use core::mem::take;
use syn::{
    Attribute,
    __private::ToTokens
};

/// A data to use in process of creation `create`
pub struct Data {
    /// The lowercase name of data, e.g. `title`
    pub lower: String,

    ///
    /// The default value for data of corresponding type,
    /// e.g. `"rokoko window"`.
    ///
    /// Empty string if no default is specified
    ///
    pub default: String,

    ///
    /// The other data that one conflicts with, i.e.
    /// user cannot specify both.
    ///
    /// Note that if `A` specify this with `B`, but `B`
    /// does not do the same with `A`, it will panic
    ///
    pub conflict: Vec <String>,

    ///
    /// Set of other data this one requires to be specified
    ///
    pub require: Vec <String>,

    ///
    /// The actual definition of usage inside of `create`
    ///
    pub usage: String,

    /// `true` if data does not contain anything
    pub short: bool
}

impl Data {
    pub fn add(lower: String, short: bool, attrs: &mut Vec <Attribute>) {
        let mut default = String::new();
        let mut conflict = Vec::new();
        let mut require = Vec::new();
        let mut usage = String::new();

        let mut i = 0;
        while i < attrs.len() {
            let path = attrs[i].path.to_token_stream().to_string();
            let mut remove = true;

            match path.as_str() {
                "default" => {
                    assert!(default.is_empty(), "cannot have multiple defaults");
                    assert!(!short, "fields without inners cannot have defaults");
                    default = after_eq(&attrs[i])
                },
                "conflict" => conflict.push(after_eq(&attrs[i])),
                "require" => require.push(after_eq(&attrs[i])),
                "usage" => {
                    assert!(usage.is_empty(), "cannot have multiple usages");
                    usage = after_eq(&attrs[i])
                }
                _ => {
                    remove = false;
                    i += 1
                }
            }

            if remove {
                attrs.remove(i);
            }
        }

        assert!(!usage.is_empty() || !require.is_empty(), "#[usage] or 1+ #[require] must be specified");

        unsafe {
            DATA.push(Self {
                lower,
                default,
                conflict,
                require,
                usage,
                short
            })
        }
    }

    pub fn get() -> Vec <Data> {
        unsafe { take(&mut DATA) }
    }
}

static mut DATA: Vec <Data> = Vec::new();

/// A callback used in `create`
pub struct Callback {
    /// The lowercase name of data, e.g. `title`
    pub lower: String,

    ///
    /// Represents a callback that cannot be specified
    /// through any other thing and is unique in its own way.
    ///
    /// `""` if is normal
    ///
    pub unique: String,

    /// Specify the behaviour if the callback is not specified
    ///
    /// `""` means nop
    pub default: String,

    /// Specify the event to be called on
    pub on: String,

    /// List of variables(separated with comma) to be used as arguments
    pub args: String
}

impl Callback {
    pub fn add(lower: String, args: String, attrs: &mut Vec <Attribute>) {
        let mut unique = String::new();
        let mut default = String::new();
        let mut on = String::new();

        let mut i = 0;
        while i < attrs.len() {
            let path = attrs[i].path.to_token_stream().to_string();
            let mut remove = true;

            match path.as_str() {
                "unique" => {
                    assert!(unique.is_empty(), "cannot specify multiple #[unique]s");
                    unique = expect_double_quotes(after_eq(&attrs[i]))
                },
                "default" => {
                    assert!(default.is_empty(), "cannot specify multiple defaults");
                    default = after_eq(&attrs[i])
                },
                "on" => {
                    assert!(on.is_empty(), "cannot specify multiple #[on]s");
                    on = after_eq(&attrs[i])
                },
                _ => {
                    remove = false;
                    i += 1
                }
            }

            if remove {
                attrs.remove(i);
            }
        }

        assert!(!on.is_empty() || !unique.is_empty(), "#[on] or #[unique] must be specified");

        unsafe {
            CALLBACKS.push(Self {
                lower,
                unique,
                default,
                on,
                args
            })
        }
    }

    pub fn get() -> Vec <Callback> {
        unsafe { take(&mut CALLBACKS) }
    }
}

static mut CALLBACKS: Vec <Callback> = Vec::new();

pub fn add_trait(ty: String) {
    unsafe {
        TRAITS.push_str(&ty);
        TRAITS.push('+')
    }
}

pub fn traits() -> String {
    unsafe { take(&mut TRAITS) }
}

static mut TRAITS: String = String::new();

pub fn add_lifetimes(ty: String) {
    unsafe {
        LIFETIMES.push_str(&ty)
    }
}

pub fn lifetimes() -> String {
    unsafe { take(&mut LIFETIMES) }
}

static mut LIFETIMES: String = String::new();

/// Splits the attribute after `=` and returns the trimmed latter
fn after_eq(attr: &Attribute) -> String {
    attr.tokens.to_string().split_once('=').expect("expected `=`").1.trim().to_string()
}

/// Expects and removes double quotes from around the string
fn expect_double_quotes(s: String) -> String {
    assert!(s.chars().next().unwrap() == '"' && s.chars().next_back().unwrap() == '"', "expected double quotes");
    s[1..s.len() - 1].to_string()
}
