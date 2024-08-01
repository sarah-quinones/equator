use crate::{
    decompose,
    traits::{DynInfo, Expr},
};
use core::fmt;

pub struct DebugMessageImpl<'a, D: decompose::Recompose> {
    pub result: D::Result,
    pub source: &'a D::Source,
    pub debug_lhs: D::DebugLhs,
    pub debug_rhs: D::DebugRhs,
    pub debug_cmp: D::DebugCmp,
    pub vtable: &'a D::VTable,
}
pub struct DebugMessage<'a, D: decompose::Recompose> {
    pub source: &'a WithSource<D::Source, &'static D::VTable>,
    pub debug_lhs: D::DebugLhs,
    pub debug_rhs: D::DebugRhs,
    pub debug_cmp: D::DebugCmp,
    pub message: fmt::Arguments<'a>,
}
impl<D: decompose::Recompose> fmt::Debug for DebugMessage<'_, D> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        D::debug_final(self, f)
    }
}

impl<D: decompose::Recompose> Copy for DebugMessage<'_, D> {}
impl<D: decompose::Recompose> Clone for DebugMessage<'_, D> {
    fn clone(&self) -> Self {
        *self
    }
}

#[derive(Copy, Clone)]
pub struct WithSource<S, V> {
    pub source: S,
    pub file: &'static str,
    pub line: u32,
    pub col: u32,
    pub vtable: V,
}

#[derive(Copy, Clone)]
pub struct Finalize<E> {
    pub inner: E,
}

impl<E: DynInfo> DynInfo for Finalize<E> {
    type VTable = E::VTable;
    const VTABLE: &'static Self::VTable = E::VTABLE;
}

impl<E> Expr for &Finalize<E> {
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

impl<E: Expr> Expr for &&Finalize<E> {
    type Result = E::Result;

    #[inline(always)]
    fn eval_expr(&self) -> bool {
        self.inner.eval_expr()
    }

    #[inline(always)]
    fn result(&self) -> Self::Result {
        self.inner.result()
    }
}
