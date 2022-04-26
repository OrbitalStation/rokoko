use super::{Callback, FnContainer, NotFnContainer, With, Empty, Equality, NotEq, NotMatching};

/// Used to obtain an actual callback
pub trait GetFn <ID: Callback> {
    /// The real type of a callback: `fn`, `{{closure}}` or a functor
    type Type: FnMut <ID::Args, Output = ID::Output>;

    /// Returns(if is contained) a callback
    fn get(&mut self) -> Option <&mut Self::Type>;
}

impl <ID: Callback> const GetFn <ID> for Empty {
    type Type = NotMatching <ID::Output>;

    #[inline(always)]
    fn get(&mut self) -> Option <&mut Self::Type> {
        None
    }
}

impl <ID: Callback, T: NotFnContainer, N: ~const GetFn <ID>> const GetFn <ID> for With <T, N> {
    type Type = N::Type;

    #[inline(always)]
    fn get(&mut self) -> Option <&mut Self::Type> {
        self.next.get()
    }
}

impl <ID: Callback, CID, Args, F: FnMut <Args>, N: ~const GetFn <ID>> const GetFn <ID> for With <FnContainer <CID, Args, F>, N> where Equality <ID, CID>: NotEq {
    type Type = N::Type;

    #[inline(always)]
    fn get(&mut self) -> Option <&mut Self::Type> {
        self.next.get()
    }
}

impl <ID: Callback, F: FnMut <ID::Args, Output = ID::Output>, N> const GetFn <ID> for With <FnContainer <ID, ID::Args, F>, N> {
    type Type = F;

    #[inline(always)]
    fn get(&mut self) -> Option <&mut Self::Type> {
        Some(&mut self.data.cb)
    }
}

// /// Represents `true`
// pub struct True;
//
// /// Represents `false`
// pub struct False;
//
// /// Does a type list contains a specified `ID`
// pub trait HasFn <ID: Callback> {
//     /// [`True`] if contains, [`False`] otherwise
//     type Has;
// }
//
// impl <ID: Callback> HasFn <ID> for Empty {
//     type Has = False;
// }
//
// impl <ID: Callback, CID, Args, F: FnMut <Args>, N: HasFn <ID>> const HasFn <ID> for With <FnContainer <CID, Args, F>, N> where Equality <ID, CID>: NotEq {
//     type Has = N::Has;
// }
//
// impl <ID: Callback, F: FnMut <ID::Args, Output = ID::Output>, N> HasFn <ID> for With <FnContainer <ID, ID::Args, F>, N> {
//     type Has = True;
// }

/// Used to obtain data-like info
pub trait GetData <T> {
    /// Returns info(if is contained)
    fn get(&self) -> Option <&T>;
}

impl <T> const GetData <T> for Empty {
    #[inline(always)]
    fn get(&self) -> Option <&T> {
        None
    }
}

impl <T, E, N: ~const GetData <T>> const GetData <T> for With <E, N> where Equality <T, E>: NotEq {
    #[inline(always)]
    fn get(&self) -> Option <&T> {
        self.next.get()
    }
}

impl <T, N> const GetData <T> for With <T, N> {
    #[inline(always)]
    fn get(&self) -> Option <&T> {
        Some(&self.data)
    }
}
