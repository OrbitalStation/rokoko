use core::marker::PhantomData;

///
/// Helper trait, used to contain actual callback
///
pub struct FnContainer <ID, Args, F: FnMut <Args>> {
    pub cb: F,
    _marker: PhantomData <(ID, Args)>
}

impl <ID, Args, F: FnMut <Args>> FnContainer <ID, Args, F> {
    pub const fn new(cb: F) -> Self {
        Self {
            cb,
            _marker: PhantomData
        }
    }
}

/// Asserts that a type is not an [`FnContainer`]
pub auto trait NotFnContainer {}

impl <ID, Args, F: FnMut <Args>> !NotFnContainer for FnContainer <ID, Args, F> {}

/// Convenient alias
pub type OnEventFnContainer <E, F> = FnContainer <E, <E as Callback>::Args, F>;

/// Used to specify expected arguments of a callback when
/// implemented on an `ID` type.
pub trait Callback {
    type Output;

    type Args;
}
