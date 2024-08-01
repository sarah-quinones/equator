use crate::{
    expr,
    structures::{DebugMessage, DebugMessageImpl},
    traits::Eval,
    DisplayCmp,
};
use core::fmt;

pub type PtrToDeref = unsafe fn(*const *const ()) -> *const ();
pub type PtrToCmp = unsafe fn(cmp: *const (), lhs: *const (), rhs: *const ()) -> bool;
pub type PtrToDebug = unsafe fn(*const ()) -> &'static dyn fmt::Debug;
pub type PtrToDisplay = unsafe fn(*const ()) -> &'static dyn DisplayCmp;

pub trait Decompose {
    type Decomposed: Recompose;
}

pub trait Recompose: Sized {
    type Result: Eval;
    type Source;
    type VTable: 'static;
    type DebugLhs: Copy;
    type DebugRhs: Copy;
    type DebugCmp: Copy;

    fn debug_impl(message: &DebugMessageImpl<'_, Self>, f: &mut fmt::Formatter) -> fmt::Result;
    fn eval_impl(
        debug_lhs: Self::DebugLhs,
        debug_rhs: Self::DebugRhs,
        debug_cmp: Self::DebugCmp,
        vtable: &Self::VTable,
    ) -> Self::Result;

    fn debug_final(full: &DebugMessage<'_, Self>, f: &mut fmt::Formatter) -> fmt::Result {
        let result = Self::eval_impl(
            full.debug_lhs,
            full.debug_rhs,
            full.debug_cmp,
            &full.source.vtable,
        );
        let message = full.message;
        let inner = DebugMessageImpl::<'_, Self> {
            result,
            source: &full.source.source,
            debug_lhs: full.debug_lhs,
            debug_rhs: full.debug_rhs,
            debug_cmp: full.debug_cmp,
            vtable: full.source.vtable,
        };
        write!(
            f,
            "Assertion failed at {}:{}:{}\n",
            full.source.file, full.source.line, full.source.col
        )?;
        if message.as_str() != Some("") {
            write!(f, "{message:#?}\n")?;
        }
        Self::debug_impl(&inner, f)
    }
}

impl Recompose for bool {
    type Result = bool;
    type Source = &'static str;
    type VTable = ();
    type DebugLhs = ();
    type DebugRhs = ();
    type DebugCmp = bool;

    fn eval_impl(
        _: Self::DebugLhs,
        _: Self::DebugRhs,
        debug_cmp: Self::DebugCmp,
        _: &Self::VTable,
    ) -> Self::Result {
        debug_cmp
    }

    fn debug_impl(message: &DebugMessageImpl<'_, Self>, f: &mut fmt::Formatter) -> fmt::Result {
        let source = *message.source;
        let debug = message.debug_cmp;
        write!(f, "Assertion failed: {source}\n")?;
        write!(f, "- {source} = {debug:#?}")
    }
}

impl Recompose for crate::CmpExpr {
    type Result = bool;
    type Source = expr::CmpExpr<(), &'static str, &'static str>;
    type VTable =
        expr::CmpExpr<(PtrToDisplay, PtrToCmp), (PtrToDebug, PtrToDeref), (PtrToDebug, PtrToDeref)>;
    type DebugLhs = *const ();
    type DebugRhs = *const ();
    type DebugCmp = ();

    fn eval_impl(
        debug_lhs: Self::DebugLhs,
        debug_rhs: Self::DebugRhs,
        _: Self::DebugCmp,
        vtable: &Self::VTable,
    ) -> Self::Result {
        let debug_lhs = unsafe { (vtable.lhs.1)(&debug_lhs) };
        let debug_rhs = unsafe { (vtable.rhs.1)(&debug_rhs) };
        unsafe {
            (vtable.cmp.1)(
                core::ptr::null::<u8>().wrapping_add(1) as *const (),
                debug_lhs,
                debug_rhs,
            )
        }
    }

    fn debug_impl(message: &DebugMessageImpl<'_, Self>, f: &mut fmt::Formatter) -> fmt::Result {
        let lhs_source = message.source.lhs;
        let rhs_source = message.source.rhs;
        let debug_lhs = unsafe { (message.vtable.lhs.1)(&message.debug_lhs) };
        let debug_rhs = unsafe { (message.vtable.rhs.1)(&message.debug_rhs) };
        let debug_cmp = core::ptr::null::<u8>().wrapping_add(1) as *const ();

        let lhs = unsafe { (message.vtable.lhs.0)(debug_lhs) };
        let rhs = unsafe { (message.vtable.rhs.0)(debug_rhs) };

        struct Display<'a>(&'a dyn DisplayCmp, &'a str, &'a str);
        impl core::fmt::Display for Display<'_> {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                self.0.fmt(self.1, self.2, f)
            }
        }

        let cmp = Display(
            unsafe { (message.vtable.cmp.0)(debug_cmp) },
            lhs_source,
            &rhs_source,
        );

        write!(f, "Assertion failed: {cmp}\n")?;
        write!(f, "- {lhs_source} = {lhs:#?}\n")?;
        write!(f, "- {rhs_source} = {rhs:#?}")
    }
}

impl Recompose for crate::CustomCmpExpr {
    type Result = bool;
    type Source = expr::CustomCmpExpr<(), &'static str, &'static str>;
    type VTable = expr::CustomCmpExpr<
        (PtrToDisplay, PtrToCmp),
        (PtrToDebug, PtrToDeref),
        (PtrToDebug, PtrToDeref),
    >;
    type DebugLhs = *const ();
    type DebugRhs = *const ();
    type DebugCmp = *const ();

    fn eval_impl(
        debug_lhs: Self::DebugLhs,
        debug_rhs: Self::DebugRhs,
        debug_cmp: Self::DebugCmp,
        vtable: &Self::VTable,
    ) -> Self::Result {
        let debug_lhs = unsafe { (vtable.lhs.1)(&debug_lhs) };
        let debug_rhs = unsafe { (vtable.rhs.1)(&debug_rhs) };
        let debug_cmp = if debug_cmp.is_null() {
            core::ptr::NonNull::dangling().as_ptr() as *const ()
        } else {
            debug_cmp
        };
        unsafe { (vtable.cmp.1)(debug_cmp, debug_lhs, debug_rhs) }
    }

    fn debug_impl(message: &DebugMessageImpl<'_, Self>, f: &mut fmt::Formatter) -> fmt::Result {
        let lhs_source = message.source.lhs;
        let rhs_source = message.source.rhs;
        let debug_lhs = unsafe { (message.vtable.lhs.1)(&message.debug_lhs) };
        let debug_rhs = unsafe { (message.vtable.rhs.1)(&message.debug_rhs) };
        let debug_cmp = if message.debug_cmp.is_null() {
            core::ptr::NonNull::dangling().as_ptr() as *const ()
        } else {
            message.debug_cmp
        };

        let lhs = unsafe { (message.vtable.lhs.0)(debug_lhs) };
        let rhs = unsafe { (message.vtable.rhs.0)(debug_rhs) };

        struct Display<'a>(&'a dyn DisplayCmp, &'a str, &'a str);
        impl core::fmt::Display for Display<'_> {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                self.0.fmt(self.1, self.2, f)
            }
        }

        let cmp = Display(
            unsafe { (message.vtable.cmp.0)(debug_cmp) },
            lhs_source,
            &rhs_source,
        );

        write!(f, "Assertion failed: {cmp}\n")?;
        write!(f, "- {lhs_source} = {lhs:#?}\n")?;
        write!(f, "- {rhs_source} = {rhs:#?}")
    }
}

impl<L: Recompose, R: Recompose> Recompose for crate::AndExpr<L, R> {
    type Result = expr::AndExpr<L::Result, R::Result>;
    type Source = expr::AndExpr<L::Source, R::Source>;
    type VTable = expr::AndExpr<&'static L::VTable, &'static R::VTable>;
    type DebugCmp = expr::AndExpr<L::DebugCmp, R::DebugCmp>;
    type DebugLhs = expr::AndExpr<L::DebugLhs, R::DebugLhs>;
    type DebugRhs = expr::AndExpr<L::DebugRhs, R::DebugRhs>;

    fn eval_impl(
        debug_lhs: Self::DebugLhs,
        debug_rhs: Self::DebugRhs,
        debug_cmp: Self::DebugCmp,
        vtable: &Self::VTable,
    ) -> Self::Result {
        let lhs = L::eval_impl(debug_lhs.lhs, debug_rhs.lhs, debug_cmp.lhs, vtable.lhs);
        let rhs = R::eval_impl(debug_lhs.rhs, debug_rhs.rhs, debug_cmp.rhs, vtable.rhs);
        expr::AndExpr { lhs, rhs }
    }

    fn debug_impl(message: &DebugMessageImpl<'_, Self>, f: &mut fmt::Formatter) -> fmt::Result {
        let lhs = DebugMessageImpl::<'_, L> {
            result: message.result.lhs,
            source: &message.source.lhs,
            vtable: message.vtable.lhs,
            debug_lhs: message.debug_lhs.lhs,
            debug_rhs: message.debug_rhs.lhs,
            debug_cmp: message.debug_cmp.lhs,
        };
        let rhs = DebugMessageImpl::<'_, R> {
            result: message.result.rhs,
            source: &message.source.rhs,
            vtable: message.vtable.rhs,
            debug_lhs: message.debug_lhs.rhs,
            debug_rhs: message.debug_rhs.rhs,
            debug_cmp: message.debug_cmp.rhs,
        };

        let lhs_eval = lhs.result.eval();
        let rhs_eval = rhs.result.eval();
        if !(lhs_eval && rhs_eval) {
            if !lhs_eval {
                L::debug_impl(&lhs, f)?;
                if !rhs_eval {
                    f.write_str("\n")?;
                }
            }
            if !rhs_eval {
                R::debug_impl(&rhs, f)?;
            }
        }
        Ok(())
    }
}

impl<L: Recompose, R: Recompose> Recompose for crate::OrExpr<L, R> {
    type Result = expr::OrExpr<L::Result, R::Result>;
    type Source = expr::OrExpr<L::Source, R::Source>;
    type VTable = expr::OrExpr<&'static L::VTable, &'static R::VTable>;
    type DebugCmp = expr::AndExpr<L::DebugCmp, R::DebugCmp>;
    type DebugLhs = expr::AndExpr<L::DebugLhs, R::DebugLhs>;
    type DebugRhs = expr::AndExpr<L::DebugRhs, R::DebugRhs>;

    fn eval_impl(
        debug_lhs: Self::DebugLhs,
        debug_rhs: Self::DebugRhs,
        debug_cmp: Self::DebugCmp,
        vtable: &Self::VTable,
    ) -> Self::Result {
        let lhs = L::eval_impl(debug_lhs.lhs, debug_rhs.lhs, debug_cmp.lhs, vtable.lhs);
        let rhs = R::eval_impl(debug_lhs.rhs, debug_rhs.rhs, debug_cmp.rhs, vtable.rhs);
        expr::OrExpr { lhs, rhs }
    }

    fn debug_impl(message: &DebugMessageImpl<'_, Self>, f: &mut fmt::Formatter) -> fmt::Result {
        let lhs = DebugMessageImpl::<'_, L> {
            result: message.result.lhs,
            source: &message.source.lhs,
            vtable: message.vtable.lhs,
            debug_lhs: message.debug_lhs.lhs,
            debug_rhs: message.debug_rhs.lhs,
            debug_cmp: message.debug_cmp.lhs,
        };
        let rhs = DebugMessageImpl::<'_, R> {
            result: message.result.rhs,
            source: &message.source.rhs,
            vtable: message.vtable.rhs,
            debug_lhs: message.debug_lhs.rhs,
            debug_rhs: message.debug_rhs.rhs,
            debug_cmp: message.debug_cmp.rhs,
        };

        let lhs_eval = lhs.result.eval();
        let rhs_eval = rhs.result.eval();
        if !(lhs_eval || rhs_eval) {
            if !lhs_eval {
                L::debug_impl(&lhs, f)?;
                if !rhs_eval {
                    f.write_str("\n")?;
                }
            }
            if !rhs_eval {
                R::debug_impl(&rhs, f)?;
            }
        }
        Ok(())
    }
}

impl Decompose for &'static str {
    type Decomposed = bool;
}
impl Decompose for expr::CmpExpr<(), &'static str, &'static str> {
    type Decomposed = crate::CmpExpr;
}
impl Decompose for expr::CustomCmpExpr<(), &'static str, &'static str> {
    type Decomposed = crate::CustomCmpExpr;
}
impl<L: Decompose, R: Decompose> Decompose for expr::AndExpr<L, R> {
    type Decomposed = crate::AndExpr<L::Decomposed, R::Decomposed>;
}
impl<L: Decompose, R: Decompose> Decompose for expr::OrExpr<L, R> {
    type Decomposed = crate::OrExpr<L::Decomposed, R::Decomposed>;
}
