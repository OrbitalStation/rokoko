//!
//! This module provides a powerful `new` function to vec.
//!
//! Basically, everything here is directed for inner usage.
//!
//! However, I know how painful it is when something in library is
//! not `pub` because developer thought it won't be useful for users,
//! but you actually would be more than happy to use it.
//!
//! So, just in case, I'll leave this all `pub`.
//!
//! # Examples
//!
//! ```nightly
//! use rokoko::prelude::*;
//!
//! // Basic form
//! assert_eq!(ivec3::new(1, 2, 3), ivec3::from([1, 2, 3]));
//!
//! // Defaults
//! assert_eq!(ivec3::new(7, 8), ivec3::from([7, 8, 0]));
//!
//! // Vecs can be used in arguments
//! let v1 = bvec2::new(true, false);
//! assert_eq!(bvec3::new(v1, false), bvec3::new(true, false, false));
//!
//! let position = fvec3::new(0.0, -0.5);
//! assert_eq!(fvec4::new(position, 1.0), fvec4::new(0.0, -0.5, 0.0, 1.0));
//!
//! // Lists too
//! assert_eq!(dvec4::new(27.27, [0.0], [[-1.17]], [[[3.0]], []]), dvec4::new(27.27, 0.0, -1.17, 3.0));
//!
//! // And even tuples(up to 10 elements)!
//! assert_eq!(fvec3::new(0.1, (), ((), ()), (13.21, (((), ()))), f32::MIN), fvec3::new(0.1, 13.21, f32::MIN));
//! ```
//!
//! # FIXME
//! There is an idea of replacing all the `From`s here into some sort of `MyFrom`, which is
//! basically a `From`, but also supports conversion between types that can be converted using `as`
//! but do not implement `From` between each other.
//!
//! Should I implement that or shouldn't, that's the question.
//!

use crate::nightly;
use super::super::vec::vec;
use core::marker::PhantomData;

///
/// A type implementing `Piece <T>` could be used as an argument in `new`.
///
/// Well, now expanded.
///
/// # How the `new` works
///
/// The main wish was that `vec` should have some constructor,
/// that would accept different arguments and would create a vec from all of them.
///
/// But... How?
///
/// Firstly, I needed to decide how such a constructor
/// (by the way at the time name `new` had already appeared)
/// would take different amount of args, such as `(1, 2)` and `([1, 8], 3)`.
///
/// Secondly, there should be filling non-specified arguments with default values.
///
/// After reading some docs I found out these lovely traits: `Fn`, `FnMut` and `FnOnce`.
///
/// The idea is that `new` is *not* actually a function, but a const unit struct, which implements
/// one of `Fn*` traits(from family of `Fn*` traits I have chosen `Fn` - the most simple one)
/// multiple times or something like that and thus can accept different args.
///
/// After long period of thinking I've decided that there should be
/// no multiple implementations of `Fn`, but a single one -
/// all the generic stuff is now here -> in `Piece`.
///
/// Arguments in `Fn` are given packed in tuple, so another idea appeared -
/// what if tuple is also a `Piece`, and `Piece` is also a recursive trait?
///
/// Unfortunately, Rust does not (*yet*) support trait impls for tuples as a whole,
/// so I decided that `new` taking, let's say, 10 parameters is enough.
///
/// Now it works as follows:
///
/// You call `fvec3::new(1.0, 2)`.
///
/// The packed args `(1.0f64, 2i32)` are given to `new`.
///
/// Tuple `(f64, i32)` implements `Piece` as 2 <= 10
///     and both `f64` and `i32` are convertible to `f32`,
///     so that we can continue.
///
/// Uninit vec of 3 elements is created.
///
/// 2(number of given args) < 3(len of vec), so the last value is filled
///     with a default one (`0.0f32` in this case).
///
/// `embed` on args is called.
///
/// `embed` on `1.0f64` is called:
/// - `1.0f64` gets converted into `1.0f32`
/// - `1.0f32` is placed into the first slot of a future vec.
///
/// `embed` on `2i32` is called:
/// - `2i32` gets converted into `2.0f32`
/// - `2.0f32` is placed into the second slot of a future vec.
///
/// Voila! Our `vec` is now ready and smells tasty, filled with
/// `[1.0f32, 2.0f32, 0.0f32]`.
///
#[nightly]
pub trait Piece <T> : Copy {
    ///
    /// How many slots in vec should the type gain
    ///
    const N: usize;

    ///
    /// This function should put itself into the `place`.
    ///
    /// # Safety
    ///
    /// Implementation has to guarantee it will use exactly `N` slots,
    /// no one less, no one more.
    ///
    unsafe fn embed(self, place: *mut T);
}

///
/// Offset `array` ptr by `offset` elements,
/// i.e. `sizeof(T) * offset` bytes
/// and return the result.
///
/// # Safety
///
/// Caller has to guarantee that `array` is a valid pointer and
/// that `array` after offset will still be correct.
///
/// # Toolchain
///
/// This fn is nightly-only available, since it is not used in stable code.
///
#[nightly]
pub const unsafe fn offset <T> (array: *mut T, offset: usize) -> *mut T {
    (core::mem::transmute::<_, usize>(array) + core::mem::size_of::<T>() * offset) as *mut T
}

///
/// Indicates that type is not a tuple
///
/// # Toolchain
///
/// This trait is nightly-only available, since it is not used in stable code
/// (and also because `auto trait` is not stable yet).
///
#[nightly]
pub auto trait NotTuple {}

rokoko_macro::impl_not_tuple_and_piece_and_conversions_to_and_from_vec_for_tuples!(10);

///
/// Indicates that type is not an array
///
/// # Toolchain
///
/// This trait is nightly-only available, since it is not used in stable code
/// (and also because `auto trait` is not stable yet).
///
#[nightly]
pub auto trait NotArray {}

#[nightly]
impl <T> !NotArray for [T] {}

#[nightly]
impl <T, const N: usize> !NotArray for [T; N] {}

///
/// Single type convertible to `T` can be used in `new`
///
#[nightly(const_force(T: From <U>))]
impl <T: From <U> + Copy, U: Copy + NotArray + NotTuple> Piece <T> for U {
    const N: usize = 1;

    #[inline]
    unsafe fn embed(self, array: *mut T) {
        *array = T::from(self)
    }
}

///
/// Array of convertible to `T` types can be used in `new`
///
#[nightly(const_force(U: Piece <T>))]
impl <T: Copy, U: Piece <T>, const N: usize> Piece <T> for [U; N] {
    const N: usize = N * U::N;

    unsafe fn embed(self, mut place: *mut T) {
        let mut i = 0;
        let v = vec::from(self);
        while i < N {
            v.get_unchecked(i).embed(place);
            place = offset(place, U::N);
            i += 1
        }
    }
}

///
/// Vec of convertible to `T` types can be used in `new`
///
#[nightly(const_force(U: Piece <T>))]
impl <T: Copy, U: Piece <T>, const N: usize> Piece <T> for vec <U, N> {
    const N: usize = N * U::N;

    #[inline]
    unsafe fn embed(self, place: *mut T) {
        self.0.embed(place)
    }
}

pub struct New <T, const N: usize> (PhantomData <vec <T, N>>);

#[nightly(const_force(Args: Piece <T>, T: Default))]
impl <Args: Piece <T>, T: Default + Copy, const N: usize> FnOnce <Args> for New <T, N> {
    type Output = vec <T, N>;

    #[inline(always)]
    extern "rust-call" fn call_once(self, args: Args) -> Self::Output {
        self.call(args)
    }
}

#[nightly(const_force(Args: Piece <T>, T: Default))]
impl <Args: Piece <T>, T: Default + Copy, const N: usize> FnMut <Args> for New <T, N> {
    #[inline(always)]
    extern "rust-call" fn call_mut(&mut self, args: Args) -> Self::Output {
        self.call(args)
    }
}

#[nightly(const_force(Args: Piece <T>, T: Default))]
impl <Args: Piece <T>, T: Default + Copy, const N: usize> Fn <Args> for New <T, N> {
    extern "rust-call" fn call(&self, args: Args) -> Self::Output {
        assert!(Args::N <= N, "too many args");

        // SAFETY: safe because values are filled in the loop & `embed` below
        let mut result = unsafe { vec::uninit() };

        // SAFETY: safe because `ptr` is used only if N >= Args::N
        // so we are not out of bounds
        let mut ptr = unsafe { result.get_unchecked_mut(Args::N) } as *mut T;
        let mut i = 0;
        while i < (N - Args::N) {
            // SAFETY: safe because ptr is guaranteed to be correct(see previous `SAFETY`)
            unsafe {
                *ptr = Default::default();
                ptr = offset(ptr, 1)
            }
            i += 1
        }

        // SAFETY: safe because `Args` is tuple and implementation for tuples
        // is written by me, so that impl is safe unless some bugs,
        // but it seems there's none of them :)
        unsafe {
            args.embed(&mut result as *mut vec <T, N> as *mut T)
        }

        result
    }
}

impl <T: Copy + Default, const N: usize> vec <T, N> {
    ///
    /// For explanations, see [`Piece`].
    ///
    /// For examples, see module documentation.
    ///
    #[nightly]
    #[allow(non_upper_case_globals)]
    pub const new: New <T, N> = New(PhantomData);
}
