//!
//! This module provides various operators and other trait implementations for vec
//!

use super::vec;
use crate::nightly;
use core::{
    ops::*,
    borrow::*,
    fmt
};

impl <T: fmt::Debug + Copy, const N: usize> fmt::Debug for vec <T, N> {
    fn fmt(&self, f: &mut fmt::Formatter <'_>) -> fmt::Result {
        let type_name = core::any::type_name::<Self>();
        let mut tuple = f.debug_tuple(&type_name[type_name.find("vec<").unwrap()..]);
        let mut i = 0;
        while i < N {
            tuple.field(unsafe { self.get_unchecked(i) });
            i += 1
        }
        tuple.finish()
    }
}

#[nightly(const)]
impl <T, const N: usize> From <[T; N]> for vec <T, N> {
    #[inline]
    fn from(x: [T; N]) -> Self {
        Self(x)
    }
}

#[nightly(const)]
impl <T: Copy, const N: usize> Into <[T; N]> for vec <T, N> {
    #[inline]
    fn into(self) -> [T; N] {
        self.0
    }
}

#[cfg(feature = "window")]
/// This module provides conversions between `vec` and types from `winit`
mod window_conversions {
    use winit::dpi::{PhysicalSize, PhysicalPosition, LogicalSize, LogicalPosition};
    use super::{vec, nightly};

    macro_rules! impls {
        ($( $size:ident $pos:ident ),*) => {$(
            impls!(@ $size, width, height);
            impls!(@ $pos, x, y);
        )*};

        (@ $t:ident, $a:ident, $b:ident) => {
            #[nightly(const)]
            impl <T: Copy> From <$t <T>> for vec <T, 2> {
                #[inline]
                fn from(x: $t <T>) -> Self {
                    Self([x.$a, x.$b])
                }
            }

            #[nightly(const)]
            impl <T: Copy> From <vec <T, 2>> for $t <T> {
                #[inline]
                fn from(x: vec <T, 2>) -> Self {
                    let x: (T, T) = x.into();
                    Self::new(x.0, x.1)
                }
            }
        };
    }

    impls!(PhysicalSize PhysicalPosition, LogicalSize LogicalPosition);
}

impl <T, const N: usize> vec <T, N> {
    ///
    /// Returns a reference to an element without bounds checking.
    ///
    /// For a safe alternative see [`index`].
    ///
    /// # Safety
    ///
    /// Calling this method with an out-of-bounds index is *[undefined behavior]*
    /// even if the resulting reference is not used.
    ///
    /// [`index`]: vec::index
    /// [undefined behavior]: https://doc.rust-lang.org/reference/behavior-considered-undefined.html
    ///
    /// # Examples
    ///
    /// ```
    /// use rokoko::prelude::*;
    ///
    /// let x = vec::from_array([7, 2, 1]);
    ///
    /// unsafe {
    ///     assert_eq!(x.get_unchecked(1), &2);
    /// }
    /// ```
    ///
    #[inline]
    pub const unsafe fn get_unchecked(&self, idx: usize) -> &T {
        let begin_address: usize = core::mem::transmute(self);
        let elem_address = begin_address + idx * core::mem::size_of::<T>();
        let elem_address = elem_address as *const T;
        &*elem_address
    }

    ///
    /// Returns a mutable reference to an element without bounds checking.
    ///
    /// For a safe alternative see [`index_mut`].
    ///
    /// # Safety
    ///
    /// Calling this method with an out-of-bounds index is *[undefined behavior]*
    /// even if the resulting reference is not used.
    ///
    /// [`index_mut`]: vec::index
    /// [undefined behavior]: https://doc.rust-lang.org/reference/behavior-considered-undefined.html
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
    /// let mut x = vec::from_array([2, 7, 1]);
    ///
    /// unsafe {
    ///     let elem = x.get_unchecked_mut(1);
    ///     *elem = 13;
    /// }
    ///
    ///
    /// ```
    ///
    #[nightly(const)]
    #[inline]
    pub unsafe fn get_unchecked_mut(&mut self, idx: usize) -> &mut T {
        // Such a cast is possible there since we know that `self` is mutable
        &mut *(self.get_unchecked(idx) as *const T as *mut T)
    }
}

impl <T: Copy, const N: usize> vec <T, N> {
    ///
    /// Applies `op` to elements from `self` and elements from `rhs`, constructs new `vec` and returns it.
    /// This is useful for defining a new operator on `vec` which takes two `vec`s and returns the third.
    ///
    /// # Constness
    ///
    /// Const when `nightly` feature is enabled.
    ///
    /// # Examples
    /// ```
    /// use rokoko::prelude::*;
    ///
    /// let vec1 = ivec2::from_array([1, 2]);
    /// let vec2 = ivec2::from_array([3, 4]);
    /// let multiplied = vec1.apply_binary(vec2, |a, b| a * b);
    /// assert_eq!(multiplied, ivec2::from_array([1 * 3, 2 * 4]));
    /// ```
    ///
    #[nightly(const(F: Fn(T, U) -> R))]
    pub fn apply_binary <U: Copy, R, F: Fn(T, U) -> R + Copy> (self, rhs: vec <U, N>, op: F) -> vec <R, N> {
        let mut i = 0;
        // SAFETY: all elements gain proper value in the loop below
        let mut result = unsafe { vec::uninit() };
        while i < N {
            unsafe {
                // SAFETY: safe because `i` iterates from 0 to N(exclusively)
                // and thus is never out of bounds
                let a_address = self.get_unchecked(i);
                let b_address = rhs.get_unchecked(i);

                // SAFETY: safe because addresses are guaranteed to be correct(see previous `SAFETY`)
                // and value does not need to be dropped(because both `T` and `U` are Copy)
                let a = core::ptr::read(a_address);
                let b = core::ptr::read(b_address);

                let calculated = op(a, b);

                // SAFETY: safe because `i` iterates from 0 to N(exclusively)
                // and thus is never out of bounds
                let result_address = result.get_unchecked_mut(i);

                // SAFETY: safe because address is guaranteed to be correct(see previous `SAFETY`)
                // and value does not need to be dropped(because it is not currently initialized)
                core::ptr::write(result_address, calculated);
            }
            i += 1
        }
        result
    }

    ///
    /// Applies `op` to elements from `self` and `rhs`, constructs new `vec` and returns it.
    /// This is useful for defining a new operator on `vec` which takes `vec`,
    /// a single operand and returns an another vec.
    ///
    /// # Constness
    ///
    /// Const when `nightly` feature is enabled.
    ///
    /// # Examples
    /// ```
    /// use rokoko::prelude::*;
    ///
    /// let vec = ivec2::from_array([3, 7]);
    /// let value = 4;
    /// let multiplied = vec.apply_binary_single(value, |a, b| a * b);
    /// assert_eq!(multiplied, ivec2::from_array([3 * 4, 7 * 4]));
    /// ```
    ///
    #[nightly(const(F: Fn(T, U) -> R))]
    pub fn apply_binary_single <U: Copy, R, F: Fn(T, U) -> R + Copy> (self, rhs: U, op: F) -> vec <R, N> {
        let mut i = 0;
        // SAFETY: all elements gain proper value in the loop below
        let mut result = unsafe { vec::uninit() };
        while i < N {
            unsafe {
                // SAFETY: safe because `i` iterates from 0 to N(exclusively)
                // and thus is never out of bounds
                let address = self.get_unchecked(i);

                // SAFETY: safe because address is guaranteed to be correct(see previous `SAFETY`)
                // and value does not need to be dropped(because `T` is Copy)
                let elem = core::ptr::read(address);

                let calculated = op(elem, rhs);

                // SAFETY: safe because `i` iterates from 0 to N(exclusively)
                // and thus is never out of bounds
                let result_address = result.get_unchecked_mut(i);

                // SAFETY: safe because address is guaranteed to be correct(see previous `SAFETY`)
                // and value does not need to be dropped(because it is not currently initialized)
                core::ptr::write(result_address, calculated);
            }
            i += 1
        }
        result
    }

    ///
    /// Applies `op` to all elements and returns the result.
    /// This is useful for defining a new operator on `vec` which takes `vec`
    /// and returns a transformed `vec`
    ///
    /// # Constness
    ///
    /// Const when `nightly` feature is enabled.
    ///
    /// # Examples
    /// ```
    /// use rokoko::prelude::*;
    ///
    /// let vec = bvec4::from_array([false, false, true, false]);
    /// let negated = vec.apply_unary(|e| !e);
    /// assert_eq!(negated, bvec4::from_array([true, true, false, true]));
    /// ```
    ///
    #[nightly(const(F: Fn(T) -> R))]
    pub fn apply_unary <R, F: Fn(T) -> R + Copy> (self, op: F) -> vec <R, N> {
        let mut i = 0;
        // SAFETY: all elements gain proper value in the loop below
        let mut result = unsafe { vec::uninit() };
        while i < N {
            unsafe {
                // SAFETY: safe because `i` iterates from 0 to N(exclusively)
                // and thus is never out of bounds
                let address = self.get_unchecked(i);

                // SAFETY: safe because address is guaranteed to be correct(see previous `SAFETY`)
                // and value does not need to be dropped(because `T` is Copy)
                let elem = core::ptr::read(address);

                let calculated = op(elem);

                // SAFETY: safe because `i` iterates from 0 to N(exclusively)
                // and thus is never out of bounds
                let result_address = result.get_unchecked_mut(i);

                // SAFETY: safe because address is guaranteed to be correct(see previous `SAFETY`)
                // and value does not need to be dropped(because it is not currently initialized)
                core::ptr::write(result_address, calculated);
            }
            i += 1
        }
        result
    }

    ///
    /// Modifies all elements in `self` by applying to each `op` with corresponding elements from `rhs`.
    /// This is useful for defining a new operator on `vec` which modifies itself using another `vec`.
    ///
    /// # Constness
    ///
    /// Const when `nightly` feature is enabled.
    ///
    /// # Examples
    /// ```
    /// use rokoko::prelude::*;
    ///
    /// let mut vec1 = ivec2::from_array([1, 2]);
    /// let vec2 = ivec2::from_array([3, 4]);
    /// vec1.modify_binary(vec2, |a, b| a * b);
    /// assert_eq!(vec1, ivec2::from_array([1 * 3, 2 * 4]));
    /// ```
    ///
    #[nightly(const(F: Fn(T, U) -> R, R: Into <T>))]
    pub fn modify_binary <U: Copy, R: Into <T>, F: Fn(T, U) -> R + Copy> (&mut self, rhs: vec <U, N>, op: F) {
        let mut i = 0;
        while i < N {
            unsafe {
                // SAFETY: safe because `i` iterates from 0 to N(exclusively)
                // and thus is never out of bounds
                let a_address = self.get_unchecked_mut(i);

                let b_address = rhs.get_unchecked(i);

                // SAFETY: safe because addresses are guaranteed to be correct(see previous `SAFETY`)
                // and value does not need to be dropped(because both `T` and `U` are Copy)
                let a = core::ptr::read(a_address);
                let b = core::ptr::read(b_address);

                let calculated = op(a, b).into();

                // SAFETY: safe because address is guaranteed to be correct(see previous `SAFETY`)
                // and value does not need to be dropped(because `T` is Copy)
                core::ptr::write(a_address, calculated);
            }
            i += 1
        }
    }

    ///
    /// Modifies all elements in `self` by applying to each `op` with `rhs`.
    /// This is useful for defining a new operator on `vec` which modifies itself using some value.
    ///
    /// # Constness
    ///
    /// Const when `nightly` feature is enabled.
    ///
    /// # Examples
    /// ```
    /// use rokoko::prelude::*;
    ///
    /// let mut vec = ivec2::from_array([1, 2]);
    /// let value = 5;
    /// vec.modify_binary_single(value, |a, b| a * b);
    /// assert_eq!(vec, ivec2::from_array([1 * 5, 2 * 5]));
    /// ```
    ///
    #[nightly(const(F: Fn(T, U) -> R, R: Into <T>))]
    pub fn modify_binary_single <U: Copy, R: Into <T>, F: Fn(T, U) -> R + Copy> (&mut self, rhs: U, op: F) {
        let mut i = 0;
        while i < N {
            unsafe {
                // SAFETY: safe because `i` iterates from 0 to N(exclusively)
                // and thus is never out of bounds
                let address = self.get_unchecked_mut(i);

                // SAFETY: safe because addresses are guaranteed to be correct(see previous `SAFETY`)
                // and value does not need to be dropped(because both `T` and `U` are Copy)
                let elem = core::ptr::read(address);

                let calculated = op(elem, rhs).into();

                // SAFETY: safe because address is guaranteed to be correct(see previous `SAFETY`)
                // and value does not need to be dropped(because `T` is Copy)
                core::ptr::write(address, calculated);
            }
            i += 1
        }
    }

    ///
    /// Modify all elements in `self` by applying `op`
    ///
    /// # Constness
    ///
    /// Const when `nightly` feature is enabled.
    ///
    /// # Examples
    /// ```
    /// use rokoko::prelude::*;
    ///
    /// let mut vec = bvec4::from_array([false, false, true, false]);
    /// vec.modify_unary(|e| !e);
    /// assert_eq!(vec, bvec4::from_array([true, true, false, true]));
    /// ```
    ///
    #[nightly(const(F: Fn(T) -> R, R: Into <T>))]
    pub fn modify_unary <R: Into <T>, F: Fn(T) -> R + Copy> (&mut self, op: F) {
        let mut i = 0;
        while i < N {
            unsafe {
                // SAFETY: safe because `i` iterates from 0 to N(exclusively)
                // and thus is never out of bounds
                let address = self.get_unchecked_mut(i);

                // SAFETY: safe because address is guaranteed to be correct(see previous `SAFETY`)
                // and value does not need to be dropped(because `T` is Copy)
                let elem = core::ptr::read(address);

                let calculated = op(elem).into();

                // SAFETY: safe because address is guaranteed to be correct(see previous `SAFETY`)
                // and value does not need to be dropped(because `T` is Copy)
                core::ptr::write(address, calculated);
            }
            i += 1
        }
    }

    ///
    /// Applies `op` to elements from `self` and elements from `rhs` and returns true if all conditions are matched.
    /// This is useful for defining a new operator on `vec` which takes two `vec`s and returns the logical result.
    ///
    /// # Constness
    ///
    /// Const when `nightly` feature is enabled.
    ///
    /// # Examples
    /// ```
    /// use rokoko::prelude::*;
    ///
    /// let vec1 = ivec3::from_array([3, 17, 21]);
    /// let vec2 = ivec3::from_array([3, 17, 21]);
    /// assert!(vec1.apply_binary_bool(vec2, |a, b| a == b));
    /// ```
    ///
    #[nightly(const(F: Fn(T, U) -> bool))]
    pub fn apply_binary_bool <U: Copy, F: Fn(T, U) -> bool + Copy> (self, rhs: vec <U, N>, op: F) -> bool {
        let mut i = 0;
        while i < N {
            unsafe {
                // SAFETY: safe because `i` iterates from 0 to N(exclusively)
                // and thus is never out of bounds
                let a_address = self.get_unchecked(i);
                let b_address = rhs.get_unchecked(i);

                // SAFETY: safe because addresses are guaranteed to be correct(see previous `SAFETY`)
                // and value does not need to be dropped(because both `T` and `U` are Copy)
                let a = core::ptr::read(a_address);
                let b = core::ptr::read(b_address);

                let calculated = op(a, b);

                if !calculated {
                    return false
                }
            }
            i += 1
        }
        true
    }

    ///
    /// Applies `op` to all elements and returns true if the conditions are matched.
    /// This is useful for defining a new operator on `vec` which takes `vec` and returns the logical result.
    ///
    /// # Constness
    ///
    /// Const when `nightly` feature is enabled.
    ///
    /// # Examples
    /// ```
    /// use rokoko::prelude::*;
    ///
    /// let vec = ivec2::from_array([4, 16]);
    /// assert!(vec.apply_unary_bool(|e| e % 2 == 0));
    /// ```
    ///
    #[nightly(const(F: Fn(T) -> bool))]
    pub fn apply_unary_bool <F: Fn(T) -> bool + Copy> (self, op: F) -> bool {
        let mut i = 0;
        while i < N {
            unsafe {
                // SAFETY: safe because `i` iterates from 0 to N(exclusively)
                // and thus is never out of bounds
                let address = self.get_unchecked(i);

                // SAFETY: safe because address is guaranteed to be correct(see previous `SAFETY`)
                // and value does not need to be dropped(because `T` is Copy)
                let elem = core::ptr::read(address);

                let calculated = op(elem);

                if !calculated {
                    return false
                }
            }
            i += 1
        }
        true
    }
}

// Sole procedure macro and not `macro_rules!` because it requires
// modifications of given names
rokoko_macro::impl_bin_ops_for_vec! {
    add
    sub
    mul
    div
    rem
    shr
    shl
    bitand
    bitor
    bitxor
}

///
/// `macro_rules!` and not proc macro because it's not a big deal to pass only
/// `big` and `low`
///
macro_rules! unop {
    ($( $big:ident $low:ident )*) => {$(
        ///
        /// Strange workaround compiler's inability to understand in-place `T::$low`,
        /// but with separate function(which does absolutely the same thing) it *magically* works.
        /// Weird.
        ///
        #[nightly(const(T: $big))]
        #[inline(always)]
        fn $low <T: $big> (x: T) -> T::Output {
            T::$low(x)
        }

        #[nightly(const(T: $big))]
        impl <T: Copy + $big, const N: usize> $big for vec <T, N> {
            type Output = vec <T::Output, N>;

            #[inline]
            fn $low(self) -> Self::Output {
                self.apply_unary($low)
            }
        }
    )*};
}

unop!(Not not Neg neg);

///
/// Workaround difference between PartialEq's `fn eq(&T, &T) -> bool`
/// and `fn {...}(T, T) -> ...` of other traits in `core::ops`.
///
/// This function simply translates `(T, T)` into `(&T, &T)`.
///
#[inline(always)]
#[nightly(const(T: PartialEq <T>))]
fn eq <T: PartialEq <T> + Copy> (a: T, b: T) -> bool {
    a == b
}

///
/// Neither `macro_rules!` nor proc macro because `PartialEq`
/// is the only trait that needs to be implemented as follows
///
#[nightly(const(T: PartialEq <T>))]
impl <T: Copy + PartialEq <T>, const N: usize> PartialEq for vec <T, N> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.apply_binary_bool(*other, eq)
    }
}

#[nightly(const)]
impl <T, const N: usize> Index <usize> for vec <T, N> {
    type Output = T;

    #[inline]
    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

#[nightly(const)]
impl <T, const N: usize> IndexMut <usize> for vec <T, N> {
    #[inline]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}

#[nightly(const)]
impl <T, const N: usize> Deref for vec <T, N> {
    type Target = [T; N];

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[nightly(const)]
impl <T, const N: usize> DerefMut for vec <T, N> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[nightly(const)]
impl <T, const N: usize> AsRef <[T; N]> for vec <T, N> {
    #[inline]
    fn as_ref(&self) -> &[T; N] {
        &self.0
    }
}

#[nightly(const)]
impl <T, const N: usize> AsMut <[T; N]> for vec <T, N> {
    #[inline]
    fn as_mut(&mut self) -> &mut [T; N] {
        &mut self.0
    }
}

#[nightly(const)]
impl <T, const N: usize> Borrow <[T; N]> for vec <T, N> {
    #[inline]
    fn borrow(&self) -> &[T; N] {
        &self.0
    }
}

#[nightly(const)]
impl <T, const N: usize> BorrowMut <[T; N]> for vec <T, N> {
    #[inline]
    fn borrow_mut(&mut self) -> &mut [T; N] {
        &mut self.0
    }
}
