use crate::{spec::Wrapper, Cmp, DisplayCmp};
use core::fmt;

#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct SizedWrapper<T>(pub T);
#[repr(transparent)]
pub struct NoSizedWrapper<'a, T: ?Sized>(pub &'a T);

impl<T: ?Sized> Copy for NoSizedWrapper<'_, T> {}
impl<T: ?Sized> Clone for NoSizedWrapper<'_, T> {
    #[inline(always)]
    fn clone(&self) -> Self {
        *self
    }
}

impl<T: fmt::Debug> fmt::Debug for SizedWrapper<T> {
    #[inline(always)]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl<T: ?Sized + fmt::Debug> fmt::Debug for NoSizedWrapper<'_, T> {
    #[inline(always)]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl<C: DisplayCmp> DisplayCmp for CmpSizedWrapper<C> {
    #[inline(always)]
    fn fmt(&self, lhs: &str, rhs: &str, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(lhs, rhs, f)
    }
}

impl<T> SizedWrapper<T> {
    #[inline(always)]
    pub fn get(&self) -> &Self {
        self
    }
}
impl<T: ?Sized> NoSizedWrapper<'_, T> {
    #[inline(always)]
    pub fn get(&self) -> &Self {
        self
    }
}

impl<Lhs, Rhs, C: Cmp<Lhs, Rhs>> Cmp<SizedWrapper<Lhs>, SizedWrapper<Rhs>> for CmpSizedWrapper<C> {
    #[inline(always)]
    fn test(&self, lhs: &SizedWrapper<Lhs>, rhs: &SizedWrapper<Rhs>) -> bool {
        self.0.test(&lhs.0, &rhs.0)
    }
}
impl<Lhs: ?Sized, Rhs, C: Cmp<Lhs, Rhs>> Cmp<NoSizedWrapper<'_, Lhs>, SizedWrapper<Rhs>>
    for CmpSizedWrapper<C>
{
    #[inline(always)]
    fn test(&self, lhs: &NoSizedWrapper<'_, Lhs>, rhs: &SizedWrapper<Rhs>) -> bool {
        self.0.test(&lhs.0, &rhs.0)
    }
}
impl<Lhs, Rhs: ?Sized, C: Cmp<Lhs, Rhs>> Cmp<SizedWrapper<Lhs>, NoSizedWrapper<'_, Rhs>>
    for CmpSizedWrapper<C>
{
    #[inline(always)]
    fn test(&self, lhs: &SizedWrapper<Lhs>, rhs: &NoSizedWrapper<Rhs>) -> bool {
        self.0.test(&lhs.0, &rhs.0)
    }
}
impl<Lhs: ?Sized, Rhs: ?Sized, C: Cmp<Lhs, Rhs>>
    Cmp<NoSizedWrapper<'_, Lhs>, NoSizedWrapper<'_, Rhs>> for CmpSizedWrapper<C>
{
    #[inline(always)]
    fn test(&self, lhs: &NoSizedWrapper<Lhs>, rhs: &NoSizedWrapper<Rhs>) -> bool {
        self.0.test(&lhs.0, &rhs.0)
    }
}

impl SizedWrap {
    #[inline(always)]
    pub fn do_wrap<T>(self, value: &T) -> &SizedWrapper<T> {
        unsafe { &*(value as *const T as *const _) }
    }
}
impl NoSizedWrap {
    #[inline(always)]
    pub fn do_wrap<T: ?Sized>(self, value: &T) -> NoSizedWrapper<'_, T> {
        NoSizedWrapper(value)
    }
}

#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct CmpSizedWrapper<T>(pub T);

pub struct SizedWrap;
pub struct NoSizedWrap;

pub trait TrySizedWrap {
    type Wrap;
    fn wrap_sized(&self) -> Self::Wrap;
}

impl<T: Sized> TrySizedWrap for &Wrapper<&T> {
    type Wrap = SizedWrap;

    #[inline]
    fn wrap_sized(&self) -> Self::Wrap {
        SizedWrap
    }
}
impl<T: ?Sized> TrySizedWrap for Wrapper<&T> {
    type Wrap = NoSizedWrap;

    #[inline]
    fn wrap_sized(&self) -> Self::Wrap {
        NoSizedWrap
    }
}
