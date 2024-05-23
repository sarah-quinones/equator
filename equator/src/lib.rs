#![no_std]

use core::fmt::{Debug, Formatter, Result};
use core::marker::PhantomData;

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

#[derive(Copy, Clone)]
#[doc(hidden)]
pub struct Finalize<E, Line, Col, File> {
    pub expr: E,
    pub line: Line,
    pub col: Col,
    pub file: File,
}

#[doc(hidden)]
pub mod atomic {
    #[derive(Copy, Clone)]
    pub struct EqExpr<Lhs, Rhs> {
        pub lhs: Lhs,
        pub rhs: Rhs,
    }

    #[derive(Copy, Clone)]
    pub struct NeExpr<Lhs, Rhs> {
        pub lhs: Lhs,
        pub rhs: Rhs,
    }

    #[derive(Copy, Clone)]
    pub struct LtExpr<Lhs, Rhs> {
        pub lhs: Lhs,
        pub rhs: Rhs,
    }

    #[derive(Copy, Clone)]
    pub struct LeExpr<Lhs, Rhs> {
        pub lhs: Lhs,
        pub rhs: Rhs,
    }

    #[derive(Copy, Clone)]
    pub struct GtExpr<Lhs, Rhs> {
        pub lhs: Lhs,
        pub rhs: Rhs,
    }

    #[derive(Copy, Clone)]
    pub struct GeExpr<Lhs, Rhs> {
        pub lhs: Lhs,
        pub rhs: Rhs,
    }
}

#[doc(hidden)]
#[repr(transparent)]
pub struct DebugWrapper<T>(T);
#[doc(hidden)]
#[repr(transparent)]
pub struct NoDebugWrapper<T>(T);

impl<Lhs: PartialEq<Rhs>, Rhs> PartialEq<DebugWrapper<Rhs>> for DebugWrapper<Lhs> {
    #[inline(always)]
    fn eq(&self, other: &DebugWrapper<Rhs>) -> bool {
        self.0 == other.0
    }
}
impl<Lhs: PartialEq<Rhs>, Rhs> PartialEq<DebugWrapper<Rhs>> for NoDebugWrapper<Lhs> {
    #[inline(always)]
    fn eq(&self, other: &DebugWrapper<Rhs>) -> bool {
        self.0 == other.0
    }
}
impl<Lhs: PartialEq<Rhs>, Rhs> PartialEq<NoDebugWrapper<Rhs>> for DebugWrapper<Lhs> {
    #[inline(always)]
    fn eq(&self, other: &NoDebugWrapper<Rhs>) -> bool {
        self.0 == other.0
    }
}
impl<Lhs: PartialEq<Rhs>, Rhs> PartialEq<NoDebugWrapper<Rhs>> for NoDebugWrapper<Lhs> {
    #[inline(always)]
    fn eq(&self, other: &NoDebugWrapper<Rhs>) -> bool {
        self.0 == other.0
    }
}

impl<Lhs: PartialOrd<Rhs>, Rhs> PartialOrd<DebugWrapper<Rhs>> for DebugWrapper<Lhs> {
    #[inline(always)]
    fn partial_cmp(&self, other: &DebugWrapper<Rhs>) -> Option<core::cmp::Ordering> {
        self.0.partial_cmp(&other.0)
    }
}
impl<Lhs: PartialOrd<Rhs>, Rhs> PartialOrd<DebugWrapper<Rhs>> for NoDebugWrapper<Lhs> {
    #[inline(always)]
    fn partial_cmp(&self, other: &DebugWrapper<Rhs>) -> Option<core::cmp::Ordering> {
        self.0.partial_cmp(&other.0)
    }
}
impl<Lhs: PartialOrd<Rhs>, Rhs> PartialOrd<NoDebugWrapper<Rhs>> for DebugWrapper<Lhs> {
    #[inline(always)]
    fn partial_cmp(&self, other: &NoDebugWrapper<Rhs>) -> Option<core::cmp::Ordering> {
        self.0.partial_cmp(&other.0)
    }
}
impl<Lhs: PartialOrd<Rhs>, Rhs> PartialOrd<NoDebugWrapper<Rhs>> for NoDebugWrapper<Lhs> {
    #[inline(always)]
    fn partial_cmp(&self, other: &NoDebugWrapper<Rhs>) -> Option<core::cmp::Ordering> {
        self.0.partial_cmp(&other.0)
    }
}

impl<T: Debug> Debug for DebugWrapper<T> {
    #[inline(always)]
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        self.0.fmt(f)
    }
}
impl<T> Debug for NoDebugWrapper<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(
            f,
            "<object of type \"{}\" at address {:?}>",
            core::any::type_name::<T>(),
            self as *const _ as *const ()
        )
    }
}

#[doc(hidden)]
pub struct DebugWrap;
#[doc(hidden)]
pub struct NoDebugWrap;

impl DebugWrap {
    #[inline(always)]
    pub fn do_wrap<T: Debug>(self, value: &T) -> &DebugWrapper<T> {
        unsafe { &*(value as *const T as *const _) }
    }
}
impl NoDebugWrap {
    #[inline(always)]
    pub fn do_wrap<T>(self, value: &T) -> &NoDebugWrapper<T> {
        unsafe { &*(value as *const T as *const _) }
    }
}

#[doc(hidden)]
pub struct Wrapper<T>(pub T);

impl<T: Debug> TryDebugWrap for &Wrapper<T> {
    type Wrap = DebugWrap;

    #[inline]
    fn wrap(&self) -> Self::Wrap {
        DebugWrap
    }
}
impl<T> TryDebugWrap for Wrapper<T> {
    type Wrap = NoDebugWrap;

    #[inline]
    fn wrap(&self) -> Self::Wrap {
        NoDebugWrap
    }
}

#[doc(hidden)]
pub trait TryDebugWrap {
    type Wrap;
    fn wrap(&self) -> Self::Wrap;
}

#[doc(hidden)]
pub mod expr {
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

use atomic::*;
use expr::*;

#[doc(hidden)]
pub type PtrToDebug = for<'a> unsafe fn(*const ()) -> &'static dyn Debug;

unsafe fn as_debug_vptr_impl<T: Debug>(ptr: *const ()) -> &'static dyn Debug {
    core::mem::transmute::<&'_ dyn Debug, &'static dyn Debug>((&*(ptr as *const T)) as &dyn Debug)
}

#[doc(hidden)]
#[inline(always)]
pub const fn as_debug_vptr<T: Debug>() -> for<'a> unsafe fn(*const ()) -> &'static dyn Debug {
    as_debug_vptr_impl::<T>
}

#[doc(hidden)]
pub trait FromParts<'a> {
    type Result;
    type Source;
    type VTable;
    type Debug;
    fn from_parts(
        result: Self::Result,
        source: &'a Self::Source,
        vtable: &'a Self::VTable,
        debug: &'a Self::Debug,
        message: core::fmt::Arguments<'a>,
    ) -> Self;
}
impl<'a, Result, Source, Debug, VTable> FromParts<'a>
    for DebugMessage<'a, Result, Source, VTable, Debug>
{
    type Result = Result;
    type Source = Source;
    type Debug = Debug;
    type VTable = VTable;
    #[inline(always)]
    fn from_parts(
        result: Result,
        source: &'a Source,
        vtable: &'a VTable,
        debug: &'a Debug,
        message: core::fmt::Arguments<'a>,
    ) -> Self {
        Self {
            result,
            source,
            debug,
            vtable,
            message,
        }
    }
}

#[doc(hidden)]
pub struct DebugMessage<'a, Result, Source, VTable, Debug> {
    pub result: Result,
    pub source: &'a Source,
    pub debug: &'a Debug,
    pub vtable: &'a VTable,
    pub message: core::fmt::Arguments<'a>,
}

impl Debug for DebugMessage<'_, bool, &'static str, (), bool> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        let source = &self.source;
        let debug = &self.debug;
        write!(f, "Assertion failed: {source}\n")?;
        write!(f, "- {source} = {debug:#?}")
    }
}

impl Debug
    for DebugMessage<
        '_,
        bool,
        EqExpr<&'static str, &'static str>,
        (PtrToDebug, PtrToDebug),
        EqExpr<*const (), *const ()>,
    >
{
    fn fmt(&self, f: &mut Formatter) -> Result {
        let lhs_source = &self.source.lhs;
        let rhs_source = &self.source.rhs;
        let lhs = unsafe { self.vtable.0(self.debug.lhs) };
        let rhs = unsafe { self.vtable.1(self.debug.rhs) };
        write!(f, "Assertion failed: {lhs_source} == {rhs_source}\n")?;
        write!(f, "- {lhs_source} = {lhs:#?}\n")?;
        write!(f, "- {rhs_source} = {rhs:#?}")
    }
}
impl Debug
    for DebugMessage<
        '_,
        bool,
        NeExpr<&'static str, &'static str>,
        (PtrToDebug, PtrToDebug),
        NeExpr<*const (), *const ()>,
    >
{
    fn fmt(&self, f: &mut Formatter) -> Result {
        let lhs_source = &self.source.lhs;
        let rhs_source = &self.source.rhs;
        let lhs = unsafe { self.vtable.0(self.debug.lhs) };
        let rhs = unsafe { self.vtable.1(self.debug.rhs) };
        write!(f, "Assertion failed: {lhs_source} != {rhs_source}\n")?;
        write!(f, "- {lhs_source} = {lhs:#?}\n")?;
        write!(f, "- {rhs_source} = {rhs:#?}")
    }
}
impl Debug
    for DebugMessage<
        '_,
        bool,
        LtExpr<&'static str, &'static str>,
        (PtrToDebug, PtrToDebug),
        LtExpr<*const (), *const ()>,
    >
{
    fn fmt(&self, f: &mut Formatter) -> Result {
        let lhs_source = &self.source.lhs;
        let rhs_source = &self.source.rhs;
        let lhs = unsafe { self.vtable.0(self.debug.lhs) };
        let rhs = unsafe { self.vtable.1(self.debug.rhs) };
        write!(f, "Assertion failed: {lhs_source} < {rhs_source}\n")?;
        write!(f, "- {lhs_source} = {lhs:#?}\n")?;
        write!(f, "- {rhs_source} = {rhs:#?}")
    }
}
impl Debug
    for DebugMessage<
        '_,
        bool,
        LeExpr<&'static str, &'static str>,
        (PtrToDebug, PtrToDebug),
        LeExpr<*const (), *const ()>,
    >
{
    fn fmt(&self, f: &mut Formatter) -> Result {
        let lhs_source = &self.source.lhs;
        let rhs_source = &self.source.rhs;
        let lhs = unsafe { self.vtable.0(self.debug.lhs) };
        let rhs = unsafe { self.vtable.1(self.debug.rhs) };
        write!(f, "Assertion failed: {lhs_source} <= {rhs_source}\n")?;
        write!(f, "- {lhs_source} = {lhs:#?}\n")?;
        write!(f, "- {rhs_source} = {rhs:#?}")
    }
}
impl Debug
    for DebugMessage<
        '_,
        bool,
        GtExpr<&'static str, &'static str>,
        (PtrToDebug, PtrToDebug),
        GtExpr<*const (), *const ()>,
    >
{
    fn fmt(&self, f: &mut Formatter) -> Result {
        let lhs_source = &self.source.lhs;
        let rhs_source = &self.source.rhs;
        let lhs = unsafe { self.vtable.0(self.debug.lhs) };
        let rhs = unsafe { self.vtable.1(self.debug.rhs) };
        write!(f, "Assertion failed: {lhs_source} > {rhs_source}\n")?;
        write!(f, "- {lhs_source} = {lhs:#?}\n")?;
        write!(f, "- {rhs_source} = {rhs:#?}")
    }
}
impl Debug
    for DebugMessage<
        '_,
        bool,
        GeExpr<&'static str, &'static str>,
        (PtrToDebug, PtrToDebug),
        GeExpr<*const (), *const ()>,
    >
{
    fn fmt(&self, f: &mut Formatter) -> Result {
        let lhs_source = &self.source.lhs;
        let rhs_source = &self.source.rhs;
        let lhs = unsafe { self.vtable.0(self.debug.lhs) };
        let rhs = unsafe { self.vtable.1(self.debug.rhs) };
        write!(f, "Assertion failed: {lhs_source} >= {rhs_source}\n")?;
        write!(f, "- {lhs_source} = {lhs:#?}\n")?;
        write!(f, "- {rhs_source} = {rhs:#?}")
    }
}

impl<
        'a,
        LhsResult: Eval,
        RhsResult: Eval,
        LhsSource,
        RhsSource,
        LhsVTable,
        RhsVTable,
        LhsDebug,
        RhsDebug,
    > Debug
    for DebugMessage<
        'a,
        AndExpr<LhsResult, RhsResult>,
        AndExpr<LhsSource, RhsSource>,
        (&'static LhsVTable, &'static RhsVTable),
        AndExpr<LhsDebug, RhsDebug>,
    >
where
    DebugMessage<'a, LhsResult, LhsSource, LhsVTable, LhsDebug>: Debug,
    DebugMessage<'a, RhsResult, RhsSource, RhsVTable, RhsDebug>: Debug,
{
    fn fmt(&self, f: &mut Formatter) -> Result {
        let lhs = DebugMessage {
            result: self.result.lhs,
            source: &self.source.lhs,
            vtable: self.vtable.0,
            debug: &self.debug.lhs,
            message: core::format_args!(""),
        };
        let rhs = DebugMessage {
            result: self.result.rhs,
            source: &self.source.rhs,
            vtable: self.vtable.1,
            debug: &self.debug.rhs,
            message: core::format_args!(""),
        };

        let lhs_eval = lhs.result.eval();
        let rhs_eval = rhs.result.eval();
        if !(lhs_eval && rhs_eval) {
            if !lhs_eval {
                lhs.fmt(f)?;
                if !rhs_eval {
                    f.write_str("\n")?;
                }
            }
            if !rhs_eval {
                rhs.fmt(f)?;
            }
        }
        Ok(())
    }
}

impl<
        'a,
        LhsResult: Eval,
        RhsResult: Eval,
        LhsSource,
        RhsSource,
        LhsVTable,
        RhsVTable,
        LhsDebug,
        RhsDebug,
    > Debug
    for DebugMessage<
        'a,
        OrExpr<LhsResult, RhsResult>,
        OrExpr<LhsSource, RhsSource>,
        (&'static LhsVTable, &'static RhsVTable),
        OrExpr<LhsDebug, RhsDebug>,
    >
where
    DebugMessage<'a, LhsResult, LhsSource, LhsVTable, LhsDebug>: Debug,
    DebugMessage<'a, RhsResult, RhsSource, RhsVTable, RhsDebug>: Debug,
{
    fn fmt(&self, f: &mut Formatter) -> Result {
        let lhs = DebugMessage {
            result: self.result.lhs,
            source: &self.source.lhs,
            vtable: self.vtable.0,
            debug: &self.debug.lhs,
            message: core::format_args!(""),
        };
        let rhs = DebugMessage {
            result: self.result.rhs,
            source: &self.source.rhs,
            vtable: self.vtable.1,
            debug: &self.debug.rhs,
            message: core::format_args!(""),
        };

        let lhs_eval = lhs.result.eval();
        let rhs_eval = rhs.result.eval();
        if !(lhs_eval || rhs_eval) {
            if !lhs_eval {
                lhs.fmt(f)?;
                if !rhs_eval {
                    f.write_str("\n")?;
                }
            }
            if !rhs_eval {
                rhs.fmt(f)?;
            }
        }
        Ok(())
    }
}

impl<'a, Result: Copy, Source, VTable, Debug> core::fmt::Debug
    for DebugMessage<
        'a,
        Result,
        Finalize<Source, u32, u32, &'static str>,
        VTable,
        Finalize<Debug, (), (), ()>,
    >
where
    DebugMessage<'a, Result, Source, VTable, Debug>: core::fmt::Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        let inner = DebugMessage {
            result: self.result,
            source: &self.source.expr,
            debug: &self.debug.expr,
            vtable: self.vtable,
            message: format_args!(""),
        };
        let message = self.message;
        write!(
            f,
            "Assertion failed at {}:{}:{}\n",
            self.source.file, self.source.line, self.source.col
        )?;
        match message.as_str() {
            Some(s) if s.len() == 0 => {}
            _ => write!(f, "{message:#?}\n")?,
        }
        inner.fmt(f)
    }
}

#[doc(hidden)]
pub trait Eval: Copy {
    fn eval(&self) -> bool;
}

impl Eval for bool {
    #[inline(always)]
    fn eval(&self) -> bool {
        *self
    }
}
impl<Lhs: Eval, Rhs: Eval> Eval for AndExpr<Lhs, Rhs> {
    #[inline(always)]
    fn eval(&self) -> bool {
        self.lhs.eval() && self.rhs.eval()
    }
}
impl<Lhs: Eval, Rhs: Eval> Eval for OrExpr<Lhs, Rhs> {
    #[inline(always)]
    fn eval(&self) -> bool {
        self.lhs.eval() || self.rhs.eval()
    }
}

#[doc(hidden)]
pub trait DynDebug {
    type VTable: Copy + 'static;
    const VTABLE: &'static Self::VTable;
}

impl DynDebug for bool {
    type VTable = ();
    const VTABLE: &'static Self::VTable = &();
}
impl<Lhs: Debug, Rhs: Debug> DynDebug for EqExpr<&Lhs, &Rhs> {
    type VTable = (PtrToDebug, PtrToDebug);
    const VTABLE: &'static Self::VTable = &(as_debug_vptr::<Lhs>(), as_debug_vptr::<Rhs>());
}
impl<Lhs: Debug, Rhs: Debug> DynDebug for NeExpr<&Lhs, &Rhs> {
    type VTable = (PtrToDebug, PtrToDebug);
    const VTABLE: &'static Self::VTable = &(as_debug_vptr::<Lhs>(), as_debug_vptr::<Rhs>());
}
impl<Lhs: Debug, Rhs: Debug> DynDebug for LtExpr<&Lhs, &Rhs> {
    type VTable = (PtrToDebug, PtrToDebug);
    const VTABLE: &'static Self::VTable = &(as_debug_vptr::<Lhs>(), as_debug_vptr::<Rhs>());
}
impl<Lhs: Debug, Rhs: Debug> DynDebug for LeExpr<&Lhs, &Rhs> {
    type VTable = (PtrToDebug, PtrToDebug);
    const VTABLE: &'static Self::VTable = &(as_debug_vptr::<Lhs>(), as_debug_vptr::<Rhs>());
}
impl<Lhs: Debug, Rhs: Debug> DynDebug for GeExpr<&Lhs, &Rhs> {
    type VTable = (PtrToDebug, PtrToDebug);
    const VTABLE: &'static Self::VTable = &(as_debug_vptr::<Lhs>(), as_debug_vptr::<Rhs>());
}
impl<Lhs: Debug, Rhs: Debug> DynDebug for GtExpr<&Lhs, &Rhs> {
    type VTable = (PtrToDebug, PtrToDebug);
    const VTABLE: &'static Self::VTable = &(as_debug_vptr::<Lhs>(), as_debug_vptr::<Rhs>());
}
impl<Lhs: DynDebug, Rhs: DynDebug> DynDebug for AndExpr<Lhs, Rhs> {
    type VTable = (&'static Lhs::VTable, &'static Rhs::VTable);
    const VTABLE: &'static Self::VTable = &(Lhs::VTABLE, Rhs::VTABLE);
}
impl<Lhs: DynDebug, Rhs: DynDebug> DynDebug for OrExpr<Lhs, Rhs> {
    type VTable = (&'static Lhs::VTable, &'static Rhs::VTable);
    const VTABLE: &'static Self::VTable = &(Lhs::VTABLE, Rhs::VTABLE);
}
impl<E: DynDebug> DynDebug for Finalize<E, (), (), ()> {
    type VTable = E::VTable;
    const VTABLE: &'static Self::VTable = E::VTABLE;
}

#[doc(hidden)]
pub trait Expr {
    type Result: Eval;

    fn eval_expr(&self) -> bool;
    fn result(&self) -> Self::Result;
}

impl Expr for bool {
    type Result = bool;

    #[inline(always)]
    fn eval_expr(&self) -> bool {
        *self
    }

    #[inline(always)]
    fn result(&self) -> Self::Result {
        *self
    }
}

impl<Lhs: PartialEq<Rhs>, Rhs> Expr for EqExpr<Lhs, Rhs> {
    type Result = bool;

    #[inline(always)]
    fn eval_expr(&self) -> bool {
        self.result()
    }

    #[inline(always)]
    fn result(&self) -> Self::Result {
        self.lhs == self.rhs
    }
}

impl<Lhs: PartialEq<Rhs>, Rhs> Expr for NeExpr<Lhs, Rhs> {
    type Result = bool;

    #[inline(always)]
    fn eval_expr(&self) -> bool {
        self.result()
    }

    #[inline(always)]
    fn result(&self) -> Self::Result {
        self.lhs != self.rhs
    }
}

impl<Lhs: PartialOrd<Rhs>, Rhs> Expr for LtExpr<Lhs, Rhs> {
    type Result = bool;

    #[inline(always)]
    fn eval_expr(&self) -> bool {
        self.result()
    }

    #[inline(always)]
    fn result(&self) -> Self::Result {
        self.lhs < self.rhs
    }
}

impl<Lhs: PartialOrd<Rhs>, Rhs> Expr for LeExpr<Lhs, Rhs> {
    type Result = bool;

    #[inline(always)]
    fn eval_expr(&self) -> bool {
        self.result()
    }

    #[inline(always)]
    fn result(&self) -> Self::Result {
        self.lhs <= self.rhs
    }
}

impl<Lhs: PartialOrd<Rhs>, Rhs> Expr for GtExpr<Lhs, Rhs> {
    type Result = bool;

    #[inline(always)]
    fn eval_expr(&self) -> bool {
        self.result()
    }

    #[inline(always)]
    fn result(&self) -> Self::Result {
        self.lhs > self.rhs
    }
}

impl<Lhs: PartialOrd<Rhs>, Rhs> Expr for GeExpr<Lhs, Rhs> {
    type Result = bool;

    #[inline(always)]
    fn eval_expr(&self) -> bool {
        self.result()
    }

    #[inline(always)]
    fn result(&self) -> Self::Result {
        self.lhs >= self.rhs
    }
}

#[inline(always)]
#[doc(hidden)]
pub fn marker<T>(_: &T) -> PhantomData<T> {
    PhantomData
}

impl<Lhs: Expr, Rhs: Expr> Expr for AndExpr<Lhs, Rhs> {
    type Result = AndExpr<Lhs::Result, Rhs::Result>;

    #[inline(always)]
    fn eval_expr(&self) -> bool {
        self.lhs.eval_expr() && self.rhs.eval_expr()
    }

    #[inline(always)]
    fn result(&self) -> Self::Result {
        AndExpr {
            lhs: self.lhs.result(),
            rhs: self.rhs.result(),
        }
    }
}

impl<Lhs: Expr, Rhs: Expr> Expr for OrExpr<Lhs, Rhs> {
    type Result = OrExpr<Lhs::Result, Rhs::Result>;

    #[inline(always)]
    fn eval_expr(&self) -> bool {
        self.lhs.eval_expr() || self.rhs.eval_expr()
    }

    #[inline(always)]
    fn result(&self) -> Self::Result {
        OrExpr {
            lhs: self.lhs.result(),
            rhs: self.rhs.result(),
        }
    }
}

impl<E: Expr> Expr for Finalize<E, (), (), ()> {
    type Result = E::Result;

    #[inline(always)]
    fn eval_expr(&self) -> bool {
        self.expr.eval_expr()
    }

    #[inline(always)]
    fn result(&self) -> Self::Result {
        self.expr.result()
    }
}

impl<E> Expr for &Finalize<E, (), (), ()> {
    type Result = bool;

    #[inline(always)]
    fn eval_expr(&self) -> bool {
        unimplemented!()
    }

    #[inline(always)]
    fn result(&self) -> Self::Result {
        unimplemented!()
    }
}

impl<E: Expr> Expr for &&Finalize<E, (), (), ()> {
    type Result = E::Result;

    #[inline(always)]
    fn eval_expr(&self) -> bool {
        self.expr.eval_expr()
    }

    #[inline(always)]
    fn result(&self) -> Self::Result {
        self.expr.result()
    }
}

#[inline(always)]
#[doc(hidden)]
pub const fn vtable_for<T: DynDebug>(_: &T) -> &'static T::VTable {
    T::VTABLE
}

#[cold]
#[inline(never)]
#[doc(hidden)]
#[track_caller]
pub fn panic_failed_assert<'a, M: core::fmt::Debug + FromParts<'a>>(
    __marker: PhantomData<M>,
    result: M::Result,
    source: &'a M::Source,
    vtable: &'a M::VTable,
    debug: &'a M::Debug,
) -> ! {
    panic!(
        "{:#?}",
        M::from_parts(result, source, vtable, debug, core::format_args!(""))
    )
}

#[cold]
#[inline(never)]
#[doc(hidden)]
#[track_caller]
pub fn panic_failed_assert_with_message<'a, M: core::fmt::Debug + FromParts<'a>>(
    __marker: PhantomData<M>,
    message: core::fmt::Arguments<'a>,
    result: M::Result,
    source: &'a M::Source,
    vtable: &'a M::VTable,
    debug: &'a M::Debug,
) -> ! {
    panic!(
        "{:#?}",
        M::from_parts(result, source, vtable, debug, message)
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! test_expr {
        ($e: expr,  $debug: expr, $source: expr $(,)?) => {{
            let e = $e;
            if !e.eval_expr() {
                let message = $crate::DebugMessage {
                    result: e.result(),
                    source: $source,
                    vtable: vtable_for(&e),
                    debug: $debug,
                    message: format_args!(""),
                };
                let __marker = $crate::marker(&message);
                $crate::panic_failed_assert(
                    __marker,
                    message.result,
                    message.source,
                    message.vtable,
                    message.debug,
                );
            }
        }};
    }

    #[test]
    #[should_panic]
    fn test_bool_expr() {
        test_expr!(false, &false, &"oops");
    }

    #[test]
    #[should_panic]
    fn test_eq_expr() {
        test_expr!(
            EqExpr {
                lhs: &0i32,
                rhs: &1i32,
            },
            &EqExpr {
                lhs: (&0i32) as *const _ as *const (),
                rhs: (&1i32) as *const _ as *const (),
            },
            &EqExpr { lhs: "a", rhs: "b" },
        );
    }

    #[test]
    #[should_panic]
    fn test_and_expr() {
        test_expr!(
            AndExpr {
                lhs: false,
                rhs: OrExpr {
                    lhs: EqExpr { lhs: &4, rhs: &4 },
                    rhs: EqExpr {
                        lhs: &0i32,
                        rhs: &1i32,
                    },
                },
            },
            &AndExpr {
                lhs: false,
                rhs: OrExpr {
                    lhs: EqExpr {
                        lhs: (&4i32) as *const _ as _,
                        rhs: (&4i32) as *const _ as _,
                    },
                    rhs: EqExpr {
                        lhs: (&0i32) as *const _ as _,
                        rhs: (&1i32) as *const _ as _,
                    },
                },
            },
            &AndExpr {
                lhs: "some_bool",
                rhs: OrExpr {
                    lhs: EqExpr { lhs: "c", rhs: "d" },
                    rhs: EqExpr { lhs: "a", rhs: "b" },
                },
            },
        );
    }

    mod macro_export {
        use super::*;

        #[test]
        #[should_panic]
        fn test_assert() {
            assert!(false);
        }

        #[cfg(not(debug_assertions))]
        #[test]
        fn test_debug_assert() {
            debug_assert!(false);
        }

        #[cfg(debug_assertions)]
        #[test]
        #[should_panic]
        fn test_debug_assert() {
            debug_assert!(false);
        }
    }
}
