#![no_std]

use core::fmt;

#[doc(hidden)]
pub use equator_macro::assert as __assert_impl;

#[macro_export]
macro_rules! assert {
    ($($tokens: tt)*) => {
        $crate::__assert_impl!($crate, $($tokens)*)
    };
}

#[macro_export]
macro_rules! debug_assert {
    ($($tokens: tt)*) => {
        if cfg!(debug_assertions) {
            $crate::__assert_impl!($crate, $($tokens)*)
        }
    };
}

#[doc(hidden)]
pub mod decompose;
#[doc(hidden)]
pub mod spec;
#[doc(hidden)]
pub mod structures;
#[doc(hidden)]
pub mod traits;

#[doc(hidden)]
pub mod expr {
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct CmpExpr<Cmp, Lhs, Rhs> {
        pub cmp: Cmp,
        pub lhs: Lhs,
        pub rhs: Rhs,
    }

    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct CustomCmpExpr<Cmp, Lhs, Rhs> {
        pub cmp: Cmp,
        pub lhs: Lhs,
        pub rhs: Rhs,
    }

    #[derive(Copy, Clone)]
    pub struct AndExpr<Lhs, Rhs> {
        pub lhs: Lhs,
        pub rhs: Rhs,
    }

    #[derive(Copy, Clone)]
    pub struct OrExpr<Lhs, Rhs> {
        pub lhs: Lhs,
        pub rhs: Rhs,
    }
}

pub trait Cmp<Lhs: ?Sized, Rhs: ?Sized>: Sized {
    fn test(&self, lhs: &Lhs, rhs: &Rhs) -> bool;
}
pub trait DisplayCmp {
    fn fmt(&self, lhs: &str, rhs: &str, f: &mut fmt::Formatter) -> fmt::Result;
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Eq;
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Ne;
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Le;
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Ge;
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Lt;
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Gt;

impl DisplayCmp for Eq {
    fn fmt(&self, lhs: &str, rhs: &str, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{lhs} == {rhs}")
    }
}
impl DisplayCmp for Ne {
    fn fmt(&self, lhs: &str, rhs: &str, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{lhs} != {rhs}")
    }
}
impl DisplayCmp for Le {
    fn fmt(&self, lhs: &str, rhs: &str, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{lhs} <= {rhs}")
    }
}
impl DisplayCmp for Ge {
    fn fmt(&self, lhs: &str, rhs: &str, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{lhs} >= {rhs}")
    }
}
impl DisplayCmp for Lt {
    fn fmt(&self, lhs: &str, rhs: &str, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{lhs} < {rhs}")
    }
}
impl DisplayCmp for Gt {
    fn fmt(&self, lhs: &str, rhs: &str, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{lhs} > {rhs}")
    }
}

impl<Rhs: ?Sized, Lhs: ?Sized + PartialEq<Rhs>> Cmp<Lhs, Rhs> for Eq {
    #[inline(always)]
    fn test(&self, lhs: &Lhs, rhs: &Rhs) -> bool {
        *lhs == *rhs
    }
}
impl<Rhs: ?Sized, Lhs: ?Sized + PartialEq<Rhs>> Cmp<Lhs, Rhs> for Ne {
    #[inline(always)]
    fn test(&self, lhs: &Lhs, rhs: &Rhs) -> bool {
        *lhs != *rhs
    }
}
impl<Rhs: ?Sized, Lhs: ?Sized + PartialOrd<Rhs>> Cmp<Lhs, Rhs> for Le {
    #[inline(always)]
    fn test(&self, lhs: &Lhs, rhs: &Rhs) -> bool {
        *lhs <= *rhs
    }
}
impl<Rhs: ?Sized, Lhs: ?Sized + PartialOrd<Rhs>> Cmp<Lhs, Rhs> for Ge {
    #[inline(always)]
    fn test(&self, lhs: &Lhs, rhs: &Rhs) -> bool {
        *lhs >= *rhs
    }
}
impl<Rhs: ?Sized, Lhs: ?Sized + PartialOrd<Rhs>> Cmp<Lhs, Rhs> for Lt {
    #[inline(always)]
    fn test(&self, lhs: &Lhs, rhs: &Rhs) -> bool {
        *lhs < *rhs
    }
}
impl<Rhs: ?Sized, Lhs: ?Sized + PartialOrd<Rhs>> Cmp<Lhs, Rhs> for Gt {
    #[inline(always)]
    fn test(&self, lhs: &Lhs, rhs: &Rhs) -> bool {
        *lhs > *rhs
    }
}

impl<Lhs: ?Sized, Rhs: ?Sized, C: Cmp<Lhs, Rhs>> Cmp<Lhs, Rhs> for &C {
    #[inline(always)]
    fn test(&self, lhs: &Lhs, rhs: &Rhs) -> bool {
        (**self).test(lhs, rhs)
    }
}

#[doc(hidden)]
pub struct CmpExpr;
#[doc(hidden)]
pub struct CustomCmpExpr;
#[doc(hidden)]
pub struct AndExpr<L, R>(L, R);
#[doc(hidden)]
pub struct OrExpr<L, R>(L, R);

#[doc(hidden)]
pub struct Message<'a>(pub core::fmt::Arguments<'a>);
#[doc(hidden)]
pub struct NoMessage;

impl From<NoMessage> for core::fmt::Arguments<'_> {
    fn from(_: NoMessage) -> Self {
        core::format_args!("")
    }
}

impl<'a> From<Message<'a>> for core::fmt::Arguments<'a> {
    fn from(t: Message<'a>) -> Self {
        t.0
    }
}

#[inline(always)]
#[doc(hidden)]
pub const fn vtable_for<T: traits::DynInfo>(_: &T) -> &'static T::VTable {
    T::VTABLE
}

#[cold]
#[inline(never)]
#[doc(hidden)]
#[track_caller]
pub fn panic_failed_assert<'a, M: Into<core::fmt::Arguments<'a>>, D: decompose::Recompose>(
    debug_lhs: D::DebugLhs,
    debug_rhs: D::DebugRhs,
    debug_cmp: D::DebugCmp,
    source: &'static structures::WithSource<D::Source, &'static D::VTable>,
    message: M,
) -> ! {
    panic!(
        "{:#?}",
        structures::DebugMessage::<D> {
            source,
            debug_lhs,
            debug_rhs,
            debug_cmp,
            message: message.into(),
        }
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn test_assert() {
        assert!(false);
    }

    #[cfg(debug_assertions)]
    #[test]
    #[should_panic]
    fn test_debug_assert() {
        debug_assert!(false);
    }
}
