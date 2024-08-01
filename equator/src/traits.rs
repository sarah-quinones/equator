use crate::{
    decompose::{PtrToCmp, PtrToDebug, PtrToDeref, PtrToDisplay},
    expr,
    spec::by_val::{CmpByValWrapper, DebugVTable, DerefVTable},
    Cmp, DisplayCmp,
};

pub trait Expr {
    type Result: Eval;

    fn eval_expr(&self) -> bool;
    fn result(&self) -> Self::Result;
}

pub trait Eval: Copy {
    fn eval(&self) -> bool;
}

impl Eval for bool {
    #[inline(always)]
    fn eval(&self) -> bool {
        *self
    }
}
impl<Lhs: Eval, Rhs: Eval> Eval for expr::AndExpr<Lhs, Rhs> {
    #[inline(always)]
    fn eval(&self) -> bool {
        self.lhs.eval() && self.rhs.eval()
    }
}
impl<Lhs: Eval, Rhs: Eval> Eval for expr::OrExpr<Lhs, Rhs> {
    #[inline(always)]
    fn eval(&self) -> bool {
        self.lhs.eval() || self.rhs.eval()
    }
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

impl<Lhs, Rhs, C: Cmp<Lhs, Rhs>> Expr for expr::CmpExpr<C, Lhs, Rhs> {
    type Result = bool;

    #[inline(always)]
    fn eval_expr(&self) -> bool {
        self.result()
    }

    #[inline(always)]
    fn result(&self) -> Self::Result {
        self.cmp.test(&self.lhs, &self.rhs)
    }
}

impl<Lhs, Rhs, C: Cmp<Lhs, Rhs>> Expr for expr::CustomCmpExpr<C, Lhs, Rhs> {
    type Result = bool;

    #[inline(always)]
    fn eval_expr(&self) -> bool {
        self.result()
    }

    #[inline(always)]
    fn result(&self) -> Self::Result {
        self.cmp.test(&self.lhs, &self.rhs)
    }
}

impl<Lhs: Expr, Rhs: Expr> Expr for expr::AndExpr<Lhs, Rhs> {
    type Result = expr::AndExpr<Lhs::Result, Rhs::Result>;

    #[inline(always)]
    fn eval_expr(&self) -> bool {
        self.lhs.eval_expr() && self.rhs.eval_expr()
    }

    #[inline(always)]
    fn result(&self) -> Self::Result {
        expr::AndExpr {
            lhs: self.lhs.result(),
            rhs: self.rhs.result(),
        }
    }
}

impl<Lhs: Expr, Rhs: Expr> Expr for expr::OrExpr<Lhs, Rhs> {
    type Result = expr::OrExpr<Lhs::Result, Rhs::Result>;

    #[inline(always)]
    fn eval_expr(&self) -> bool {
        self.lhs.eval_expr() || self.rhs.eval_expr()
    }

    #[inline(always)]
    fn result(&self) -> Self::Result {
        expr::OrExpr {
            lhs: self.lhs.result(),
            rhs: self.rhs.result(),
        }
    }
}

pub trait DynInfo {
    type VTable: Copy + 'static;
    const VTABLE: &'static Self::VTable;
}

impl DynInfo for bool {
    type VTable = ();
    const VTABLE: &'static Self::VTable = &();
}

impl<Lhs: DynInfo, Rhs: DynInfo> DynInfo for expr::AndExpr<Lhs, Rhs> {
    type VTable = expr::AndExpr<&'static Lhs::VTable, &'static Rhs::VTable>;
    const VTABLE: &'static Self::VTable = &expr::AndExpr {
        lhs: Lhs::VTABLE,
        rhs: Rhs::VTABLE,
    };
}
impl<Lhs: DynInfo, Rhs: DynInfo> DynInfo for expr::OrExpr<Lhs, Rhs> {
    type VTable = expr::OrExpr<&'static Lhs::VTable, &'static Rhs::VTable>;
    const VTABLE: &'static Self::VTable = &expr::OrExpr {
        lhs: Lhs::VTABLE,
        rhs: Rhs::VTABLE,
    };
}

unsafe fn as_display_vptr<'a, T: DisplayCmp>(ptr: *const ()) -> &'a dyn DisplayCmp {
    core::mem::transmute::<&'_ dyn DisplayCmp, &'static dyn DisplayCmp>(
        (&*(ptr as *const _ as *const T)) as &dyn DisplayCmp,
    )
}

unsafe fn as_cmp_vptr<Lhs, Rhs, C: Cmp<Lhs, Rhs>>(
    cmp: *const (),
    lhs: *const (),
    rhs: *const (),
) -> bool {
    let cmp = &*(cmp as *const C);
    let lhs = &*(lhs as *const Lhs);
    let rhs = &*(rhs as *const Rhs);
    cmp.test(lhs, rhs)
}

impl<
        C: DisplayCmp + Cmp<Lhs::Inner, Rhs::Inner>,
        Lhs: DebugVTable + DerefVTable,
        Rhs: DebugVTable + DerefVTable,
    > DynInfo for expr::CmpExpr<&CmpByValWrapper<C>, Lhs, Rhs>
{
    type VTable =
        expr::CmpExpr<(PtrToDisplay, PtrToCmp), (PtrToDebug, PtrToDeref), (PtrToDebug, PtrToDeref)>;
    const VTABLE: &'static Self::VTable = &expr::CmpExpr {
        cmp: (
            as_display_vptr::<C>,
            as_cmp_vptr::<Lhs::Inner, Rhs::Inner, C>,
        ),
        lhs: (<Lhs as DebugVTable>::VTABLE, <Lhs as DerefVTable>::VTABLE),
        rhs: (<Rhs as DebugVTable>::VTABLE, <Rhs as DerefVTable>::VTABLE),
    };
}

impl<
        C: DisplayCmp + Cmp<Lhs::Inner, Rhs::Inner>,
        Lhs: DebugVTable + DerefVTable,
        Rhs: DebugVTable + DerefVTable,
    > DynInfo for expr::CustomCmpExpr<&CmpByValWrapper<C>, Lhs, Rhs>
{
    type VTable = expr::CustomCmpExpr<
        (PtrToDisplay, PtrToCmp),
        (PtrToDebug, PtrToDeref),
        (PtrToDebug, PtrToDeref),
    >;
    const VTABLE: &'static Self::VTable = &expr::CustomCmpExpr {
        cmp: (
            as_display_vptr::<C>,
            as_cmp_vptr::<Lhs::Inner, Rhs::Inner, C>,
        ),
        lhs: (<Lhs as DebugVTable>::VTABLE, <Lhs as DerefVTable>::VTABLE),
        rhs: (<Rhs as DebugVTable>::VTABLE, <Rhs as DerefVTable>::VTABLE),
    };
}
