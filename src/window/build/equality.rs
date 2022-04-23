use core::marker::PhantomData;

/// Used to compare 2 types on equality.
pub struct Equality <A, B> (PhantomData <(A, B)>);

/// Helper trait
pub auto trait NotEq {}

impl <A> !NotEq for Equality <A, A> {}
