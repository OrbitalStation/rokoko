use core::marker::PhantomData;

/// A stub callback-like functor, used in `GetFn` when
/// no user callback is found
pub struct NotMatching <O> (PhantomData <O>);

impl <Args, O> const FnOnce <Args> for NotMatching <O> {
    type Output = O;

    /// This should never be called.
    #[inline(always)]
    extern "rust-call" fn call_once(self, _: Args) -> Self::Output {
        unreachable!()
    }
}

impl <Args, O> const FnMut <Args> for NotMatching <O> {
    /// This should never be called.
    #[inline(always)]
    extern "rust-call" fn call_mut(&mut self, _: Args) -> Self::Output {
        unreachable!()
    }
}
