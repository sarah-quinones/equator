use crate::{
    decompose::{PtrToDebug, PtrToDeref},
    spec::{debug::CmpDebugWrapper, sized::CmpSizedWrapper, Wrapper},
    Cmp, DisplayCmp,
};
use core::fmt;

#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct ByVal<T>(pub T);

#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct ByRef<T>(pub T);

impl<T: fmt::Debug> fmt::Debug for ByVal<T> {
    #[inline(always)]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl<T: fmt::Debug> fmt::Debug for ByRef<T> {
    #[inline(always)]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl<T> ByRef<T> {
    #[inline(always)]
    pub fn get_ptr(&self) -> *const () {
        self as *const _ as *const ()
    }
}

impl<T: Copy> ByVal<T> {
    const FIT_IN_PTR: bool = core::mem::size_of::<T>() <= core::mem::size_of::<*const ()>()
        && core::mem::align_of::<T>() <= core::mem::align_of::<*const ()>();

    #[inline(always)]
    pub fn get_ptr(&self) -> *const () {
        if Self::FIT_IN_PTR {
            let mut out = core::ptr::null::<()>();
            unsafe {
                *((&mut out) as *mut *const () as *mut T) = self.0;
            };
            out
        } else {
            self as *const _ as *const ()
        }
    }
}

impl<C: DisplayCmp> DisplayCmp for CmpByValWrapper<C> {
    #[inline(always)]
    fn fmt(&self, lhs: &str, rhs: &str, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(lhs, rhs, f)
    }
}

impl<Lhs, Rhs, C: Cmp<Lhs, Rhs>> Cmp<&ByVal<Lhs>, &ByVal<Rhs>> for CmpByValWrapper<C> {
    #[inline(always)]
    fn test(&self, lhs: &&ByVal<Lhs>, rhs: &&ByVal<Rhs>) -> bool {
        self.0.test(&lhs.0, &rhs.0)
    }
}
impl<Lhs, Rhs, C: Cmp<Lhs, Rhs>> Cmp<&ByRef<Lhs>, &ByVal<Rhs>> for CmpByValWrapper<C> {
    #[inline(always)]
    fn test(&self, lhs: &&ByRef<Lhs>, rhs: &&ByVal<Rhs>) -> bool {
        self.0.test(&lhs.0, &rhs.0)
    }
}
impl<Lhs, Rhs, C: Cmp<Lhs, Rhs>> Cmp<&ByVal<Lhs>, &ByRef<Rhs>> for CmpByValWrapper<C> {
    #[inline(always)]
    fn test(&self, lhs: &&ByVal<Lhs>, rhs: &&ByRef<Rhs>) -> bool {
        self.0.test(&lhs.0, &rhs.0)
    }
}
impl<Lhs, Rhs, C: Cmp<Lhs, Rhs>> Cmp<&ByRef<Lhs>, &ByRef<Rhs>> for CmpByValWrapper<C> {
    #[inline(always)]
    fn test(&self, lhs: &&ByRef<Lhs>, rhs: &&ByRef<Rhs>) -> bool {
        self.0.test(&lhs.0, &rhs.0)
    }
}

impl ByValWrap {
    #[inline(always)]
    pub fn do_wrap<T: Copy>(self, value: &T) -> &ByVal<T> {
        unsafe { &*(value as *const T as *const _) }
    }
}
impl ByRefWrap {
    #[inline(always)]
    pub fn do_wrap<T>(self, value: &T) -> &ByRef<T> {
        unsafe { &*(value as *const T as *const _) }
    }
}

impl<'a, C> CmpByValWrapper<CmpSizedWrapper<CmpDebugWrapper<&'a C>>> {
    #[inline(always)]
    pub fn __wrap_ref(self) -> &'a CmpByValWrapper<CmpSizedWrapper<CmpDebugWrapper<C>>> {
        unsafe {
            &*(self.0 .0 .0 as *const C
                as *const CmpByValWrapper<CmpSizedWrapper<CmpDebugWrapper<C>>>)
        }
    }
}
impl<T: Copy> TryByValWrap for &Wrapper<&T> {
    type Wrap = ByValWrap;

    #[inline]
    fn wrap_by_val(&self) -> Self::Wrap {
        ByValWrap
    }
}
impl<T> TryByValWrap for Wrapper<T> {
    type Wrap = ByRefWrap;

    #[inline]
    fn wrap_by_val(&self) -> Self::Wrap {
        ByRefWrap
    }
}

pub struct ByValWrap;
pub struct ByRefWrap;

#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct CmpByValWrapper<T>(pub T);

pub trait TryByValWrap {
    type Wrap;
    fn wrap_by_val(&self) -> Self::Wrap;
}

pub(crate) trait DebugVTable {
    const VTABLE: PtrToDebug;
}
pub(crate) trait DerefVTable {
    type Inner;
    const VTABLE: unsafe fn(*const *const ()) -> *const ();
}

unsafe fn no_deref(ptr: *const *const ()) -> *const () {
    ptr as *const ()
}
unsafe fn deref(ptr: *const *const ()) -> *const () {
    *ptr
}

impl<T: Copy> DerefVTable for &ByVal<T> {
    type Inner = T;
    const VTABLE: PtrToDeref = {
        if ByVal::<T>::FIT_IN_PTR {
            no_deref
        } else {
            deref
        }
    };
}
impl<T> DerefVTable for &ByRef<T> {
    type Inner = T;
    const VTABLE: PtrToDeref = { deref };
}

impl<T: fmt::Debug> DebugVTable for &ByVal<T> {
    const VTABLE: PtrToDebug = as_debug_vptr::<T>;
}
impl<T: fmt::Debug> DebugVTable for &ByRef<T> {
    const VTABLE: PtrToDebug = as_debug_vptr::<T>;
}

unsafe fn as_debug_vptr<T: fmt::Debug>(ptr: *const ()) -> &'static dyn fmt::Debug {
    core::mem::transmute::<&'_ dyn fmt::Debug, &'static dyn fmt::Debug>(
        (&*(ptr as *const T)) as &dyn fmt::Debug,
    )
}
