use crate::{spec::Wrapper, Cmp, DisplayCmp};
use core::fmt;

#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct DebugWrapper<T: ?Sized>(pub T);
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct NoDebugWrapper<T: ?Sized>(pub T);

impl<C: DisplayCmp> DisplayCmp for CmpDebugWrapper<C> {
    #[inline(always)]
    fn fmt(&self, lhs: &str, rhs: &str, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(lhs, rhs, f)
    }
}

impl<Lhs: ?Sized, Rhs: ?Sized, C: Cmp<Lhs, Rhs>> Cmp<DebugWrapper<Lhs>, DebugWrapper<Rhs>>
    for CmpDebugWrapper<C>
{
    #[inline(always)]
    fn test(&self, lhs: &DebugWrapper<Lhs>, rhs: &DebugWrapper<Rhs>) -> bool {
        self.0.test(&lhs.0, &rhs.0)
    }
}
impl<Lhs: ?Sized, Rhs: ?Sized, C: Cmp<Lhs, Rhs>> Cmp<NoDebugWrapper<Lhs>, DebugWrapper<Rhs>>
    for CmpDebugWrapper<C>
{
    #[inline(always)]
    fn test(&self, lhs: &NoDebugWrapper<Lhs>, rhs: &DebugWrapper<Rhs>) -> bool {
        self.0.test(&lhs.0, &rhs.0)
    }
}
impl<Lhs: ?Sized, Rhs: ?Sized, C: Cmp<Lhs, Rhs>> Cmp<DebugWrapper<Lhs>, NoDebugWrapper<Rhs>>
    for CmpDebugWrapper<C>
{
    #[inline(always)]
    fn test(&self, lhs: &DebugWrapper<Lhs>, rhs: &NoDebugWrapper<Rhs>) -> bool {
        self.0.test(&lhs.0, &rhs.0)
    }
}
impl<Lhs: ?Sized, Rhs: ?Sized, C: Cmp<Lhs, Rhs>> Cmp<NoDebugWrapper<Lhs>, NoDebugWrapper<Rhs>>
    for CmpDebugWrapper<C>
{
    #[inline(always)]
    fn test(&self, lhs: &NoDebugWrapper<Lhs>, rhs: &NoDebugWrapper<Rhs>) -> bool {
        self.0.test(&lhs.0, &rhs.0)
    }
}

impl<T: ?Sized + fmt::Debug> fmt::Debug for DebugWrapper<T> {
    #[inline(always)]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}
impl<T: ?Sized> fmt::Debug for NoDebugWrapper<T> {
    #[inline(always)]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "<object of type \"{}\" at address {:?}>",
            core::any::type_name::<T>(),
            self as *const _ as *const ()
        )
    }
}

pub struct DebugWrap;
pub struct NoDebugWrap;

impl DebugWrap {
    #[inline(always)]
    pub fn do_wrap<T: ?Sized>(self, value: &T) -> &DebugWrapper<T> {
        unsafe { &*(value as *const T as *const DebugWrapper<T>) }
    }
}
impl NoDebugWrap {
    #[inline(always)]
    pub fn do_wrap<T: ?Sized>(self, value: &T) -> &NoDebugWrapper<T> {
        unsafe { &*(value as *const T as *const NoDebugWrapper<T>) }
    }
}

#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct CmpDebugWrapper<T>(pub T);

pub trait TryDebugWrap {
    type Wrap;
    fn wrap_debug(&self) -> Self::Wrap;
}

impl<T: fmt::Debug + ?Sized> TryDebugWrap for &Wrapper<T> {
    type Wrap = DebugWrap;

    #[inline]
    fn wrap_debug(&self) -> Self::Wrap {
        DebugWrap
    }
}
impl<T: ?Sized> TryDebugWrap for Wrapper<T> {
    type Wrap = NoDebugWrap;

    #[inline]
    fn wrap_debug(&self) -> Self::Wrap {
        NoDebugWrap
    }
}
