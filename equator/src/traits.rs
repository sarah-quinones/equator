use crate::{
    decompose::{PtrToCmp, PtrToDebug, PtrToDeref, PtrToDisplay},
    expr, spec,
    spec::by_val::{ByVal, DerefVTable},
    spec::Wrapper,
    Cmp, CmpDisplay, CmpError, Single,
};
use core::fmt;

pub trait Expr {
    type Result: Eval;
    type Marker;

    fn eval_expr(&self) -> bool;
    #[inline(always)]
    fn __marker(&self) -> core::marker::PhantomData<Self::Marker> {
        core::marker::PhantomData
    }
}

pub trait Eval {
    fn eval(&self) -> bool;
}

impl<E> Eval for Result<(), E> {
    #[inline(always)]
    fn eval(&self) -> bool {
        self.is_ok()
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
    type Result = Result<(), ()>;
    type Marker = bool;

    #[inline(always)]
    fn eval_expr(&self) -> bool {
        *self
    }
}

impl<
        Lhs0: ?Sized,
        Lhs1: ?Sized + core::ops::Deref<Target = Lhs0>,
        Lhs2: core::ops::Deref<Target = Lhs1>,
        Lhs3: core::ops::Deref<Target = Lhs2>,
        Rhs0: ?Sized,
        Rhs1: ?Sized + core::ops::Deref<Target = Rhs0>,
        Rhs2: core::ops::Deref<Target = Rhs1>,
        Rhs3: core::ops::Deref<Target = Rhs2>,
        C: Cmp<Lhs0, Rhs0>,
    > Expr
    for expr::CmpExpr<
        &spec::by_val::CmpByValWrapper<
            spec::sized::CmpSizedWrapper<spec::debug::CmpDebugWrapper<C>>,
        >,
        &Lhs3,
        &Rhs3,
    >
{
    type Result = Result<(), C::Error>;
    type Marker = crate::CmpExpr;

    #[inline(always)]
    fn eval_expr(&self) -> bool {
        self.cmp.0 .0 .0.test(&***self.lhs, &***self.rhs).is_ok()
    }
}

impl<
        Lhs0: ?Sized,
        Lhs1: ?Sized + core::ops::Deref<Target = Lhs0>,
        Lhs2: core::ops::Deref<Target = Lhs1>,
        Lhs3: core::ops::Deref<Target = Lhs2> + fmt::Debug + DerefVTable,
        Rhs0: ?Sized,
        Rhs1: ?Sized + core::ops::Deref<Target = Rhs0>,
        Rhs2: core::ops::Deref<Target = Rhs1>,
        Rhs3: core::ops::Deref<Target = Rhs2> + fmt::Debug + DerefVTable,
        C: Cmp<Lhs0, Rhs0>,
    > Expr
    for expr::CustomCmpExpr<
        &spec::by_val::CmpByValWrapper<
            spec::sized::CmpSizedWrapper<spec::debug::CmpDebugWrapper<C>>,
        >,
        &Lhs3,
        &Rhs3,
    >
{
    type Result = Result<(), C::Error>;
    type Marker = crate::CustomCmpExpr<C::Error>;

    #[inline(always)]
    fn eval_expr(&self) -> bool {
        self.cmp.0 .0 .0.test(&***self.lhs, &***self.rhs).is_ok()
    }
}

impl<Lhs: Expr, Rhs: Expr> Expr for expr::AndExpr<Lhs, Rhs> {
    type Result = expr::AndExpr<Lhs::Result, Rhs::Result>;
    type Marker = crate::AndExpr<Lhs::Marker, Rhs::Marker>;

    #[inline(always)]
    fn eval_expr(&self) -> bool {
        self.lhs.eval_expr() && self.rhs.eval_expr()
    }
}

impl<Lhs: Expr, Rhs: Expr> Expr for expr::OrExpr<Lhs, Rhs> {
    type Result = expr::OrExpr<Lhs::Result, Rhs::Result>;
    type Marker = crate::OrExpr<Lhs::Marker, Rhs::Marker>;

    #[inline(always)]
    fn eval_expr(&self) -> bool {
        self.lhs.eval_expr() || self.rhs.eval_expr()
    }
}

pub trait DynInfoType {
    type VTable: Copy + 'static;
    const NULL_VTABLE: &'static Self::VTable;
}

pub trait DynInfo: DynInfoType {
    const VTABLE: &'static Self::VTable;

    #[inline(always)]
    fn vtable(&self) -> &'static Self::VTable {
        Self::VTABLE
    }
}

impl DynInfoType for bool {
    type VTable = ();
    const NULL_VTABLE: &'static Self::VTable = &();
}
impl DynInfo for bool {
    const VTABLE: &'static Self::VTable = &();
}

impl<Lhs: DynInfoType, Rhs: DynInfoType> DynInfoType for expr::AndExpr<Lhs, Rhs> {
    type VTable = expr::AndExpr<&'static Lhs::VTable, &'static Rhs::VTable>;
    const NULL_VTABLE: &'static Self::VTable = &expr::AndExpr {
        lhs: Lhs::NULL_VTABLE,
        rhs: Rhs::NULL_VTABLE,
    };
}
impl<Lhs: DynInfoType, Rhs: DynInfoType> DynInfoType for expr::OrExpr<Lhs, Rhs> {
    type VTable = expr::OrExpr<&'static Lhs::VTable, &'static Rhs::VTable>;
    const NULL_VTABLE: &'static Self::VTable = &expr::OrExpr {
        lhs: Lhs::NULL_VTABLE,
        rhs: Rhs::NULL_VTABLE,
    };
}

impl<Lhs: DynInfo, Rhs: DynInfo> DynInfo for expr::AndExpr<Lhs, Rhs> {
    const VTABLE: &'static Self::VTable = &expr::AndExpr {
        lhs: Lhs::VTABLE,
        rhs: Rhs::VTABLE,
    };
}
impl<Lhs: DynInfo, Rhs: DynInfo> DynInfo for expr::OrExpr<Lhs, Rhs> {
    const VTABLE: &'static Self::VTable = &expr::OrExpr {
        lhs: Lhs::VTABLE,
        rhs: Rhs::VTABLE,
    };
}

unsafe fn as_display_vptr<'a, C, Lhs, Rhs, E: CmpDisplay<C, Lhs, Rhs>>(
    ptr: *const (),
) -> &'a dyn CmpDisplay<*const (), dyn fmt::Debug, dyn fmt::Debug> {
    #[repr(transparent)]
    struct DynDisplay<E, C, Lhs, Rhs>(
        E,
        core::marker::PhantomData<C>,
        core::marker::PhantomData<Lhs>,
        core::marker::PhantomData<Rhs>,
    );

    impl<Lhs, Rhs, C, E: CmpDisplay<C, Lhs, Rhs>>
        CmpDisplay<*const (), dyn fmt::Debug, dyn fmt::Debug> for DynDisplay<E, C, Lhs, Rhs>
    {
        fn fmt(
            &self,
            cmp: &*const (),
            lhs: &dyn fmt::Debug,
            lhs_source: &str,
            lhs_debug: &dyn fmt::Debug,
            rhs: &dyn fmt::Debug,
            rhs_source: &str,
            rhs_debug: &dyn fmt::Debug,
            f: &mut fmt::Formatter,
        ) -> fmt::Result {
            unsafe {
                let lhs = &*(lhs as *const dyn fmt::Debug as *const Lhs);
                let rhs = &*(rhs as *const dyn fmt::Debug as *const Rhs);
                let cmp = &*((*cmp) as *const C);
                let err = &self.0;

                err.fmt(
                    cmp, lhs, lhs_source, lhs_debug, rhs, rhs_source, rhs_debug, f,
                )
            }
        }
    }

    core::mem::transmute::<
        &'_ dyn CmpDisplay<*const (), dyn fmt::Debug, dyn fmt::Debug>,
        &'static dyn CmpDisplay<*const (), dyn fmt::Debug, dyn fmt::Debug>,
    >(
        (&*(ptr as *const DynDisplay<E, C, Lhs, Rhs>))
            as &dyn CmpDisplay<*const (), dyn fmt::Debug, dyn fmt::Debug>,
    )
}

unsafe fn as_dyn_display_vptr<'a, C, E: CmpDisplay<C, dyn fmt::Debug, dyn fmt::Debug>>(
    ptr: *const (),
) -> &'a dyn CmpDisplay<*const (), dyn fmt::Debug, dyn fmt::Debug> {
    #[repr(transparent)]
    struct DynDisplay<E, C>(E, core::marker::PhantomData<C>);

    impl<C, E: CmpDisplay<C, dyn fmt::Debug, dyn fmt::Debug>>
        CmpDisplay<*const (), dyn fmt::Debug, dyn fmt::Debug> for DynDisplay<E, C>
    {
        fn fmt(
            &self,
            cmp: &*const (),
            lhs: &(dyn fmt::Debug + 'static),
            lhs_source: &str,
            lhs_debug: &dyn fmt::Debug,
            rhs: &(dyn fmt::Debug + 'static),
            rhs_source: &str,
            rhs_debug: &dyn fmt::Debug,
            f: &mut fmt::Formatter,
        ) -> fmt::Result {
            unsafe {
                let cmp = &*((*cmp) as *const C);
                let err = &self.0;
                err.fmt(
                    cmp, lhs, lhs_source, lhs_debug, rhs, rhs_source, rhs_debug, f,
                )
            }
        }
    }

    core::mem::transmute::<
        &'_ dyn CmpDisplay<*const (), dyn fmt::Debug, dyn fmt::Debug>,
        &'static dyn CmpDisplay<*const (), dyn fmt::Debug, dyn fmt::Debug>,
    >(
        (&*(ptr as *const DynDisplay<E, C>))
            as &dyn CmpDisplay<*const (), dyn fmt::Debug, dyn fmt::Debug>,
    )
}

unsafe fn as_cmp_vptr<
    Lhs2: core::ops::Deref,
    Rhs2: core::ops::Deref,
    C: Cmp<<Lhs2::Target as core::ops::Deref>::Target, <Rhs2::Target as core::ops::Deref>::Target>,
>(
    out: *mut (),
    cmp: *const (),
    lhs: *const (),
    rhs: *const (),
) where
    Lhs2::Target: core::ops::Deref,
    Rhs2::Target: core::ops::Deref,
{
    let out = out as *mut Result<(), C::Error>;
    let cmp = &*(cmp as *const C);
    let lhs = &*(lhs as *const Lhs2);
    let rhs = &*(rhs as *const Rhs2);
    out.write(cmp.test(&**lhs, &**rhs));
}

unsafe fn as_debug_vptr<T: fmt::Debug>(ptr: *const ()) -> &'static dyn fmt::Debug {
    core::mem::transmute::<&'_ dyn fmt::Debug, &'static dyn fmt::Debug>(
        (&*(ptr as *const T)) as &dyn fmt::Debug,
    )
}

impl<
        Lhs0: ?Sized,
        Lhs1: ?Sized + core::ops::Deref<Target = Lhs0>,
        Lhs2: core::ops::Deref<Target = Lhs1>,
        Lhs3: core::ops::Deref<Target = Lhs2> + fmt::Debug + DerefVTable,
        Rhs0: ?Sized,
        Rhs1: ?Sized + core::ops::Deref<Target = Rhs0>,
        Rhs2: core::ops::Deref<Target = Rhs1>,
        Rhs3: core::ops::Deref<Target = Rhs2> + fmt::Debug + DerefVTable,
        C: CmpError<C, Lhs0, Rhs0>,
    > DynInfoType
    for expr::CmpExpr<
        &spec::by_val::CmpByValWrapper<
            spec::sized::CmpSizedWrapper<spec::debug::CmpDebugWrapper<C>>,
        >,
        &Lhs3,
        &Rhs3,
    >
where
    C::Error: CmpDisplay<C, Single<dyn fmt::Debug>, Single<dyn fmt::Debug>>,
{
    type VTable =
        expr::CmpExpr<(PtrToDisplay, PtrToCmp), (PtrToDebug, PtrToDeref), (PtrToDebug, PtrToDeref)>;
    const NULL_VTABLE: &'static Self::VTable = &expr::CmpExpr {
        cmp: (
            as_dyn_display_vptr::<
                spec::by_val::CmpByValWrapper<
                    spec::sized::CmpSizedWrapper<spec::debug::CmpDebugWrapper<C>>,
                >,
                spec::by_val::CmpByValWrapper<
                    spec::sized::CmpSizedWrapper<spec::debug::CmpDebugWrapper<C::Error>>,
                >,
            >,
            as_cmp_vptr::<
                ByVal<ByVal<crate::Single<()>>>,
                ByVal<ByVal<crate::Single<()>>>,
                crate::Eq,
            >,
        ),
        lhs: (as_debug_vptr::<Lhs3>, Lhs3::VTABLE),
        rhs: (as_debug_vptr::<Rhs3>, Rhs3::VTABLE),
    };
}

impl<
        Lhs0: ?Sized,
        Lhs1: ?Sized + core::ops::Deref<Target = Lhs0>,
        Lhs2: core::ops::Deref<Target = Lhs1>,
        Lhs3: core::ops::Deref<Target = Lhs2> + fmt::Debug + DerefVTable,
        Rhs0: ?Sized,
        Rhs1: ?Sized + core::ops::Deref<Target = Rhs0>,
        Rhs2: core::ops::Deref<Target = Rhs1>,
        Rhs3: core::ops::Deref<Target = Rhs2> + fmt::Debug + DerefVTable,
        C: CmpError<C, Lhs0, Rhs0>,
    > DynInfoType
    for expr::CustomCmpExpr<
        &spec::by_val::CmpByValWrapper<
            spec::sized::CmpSizedWrapper<spec::debug::CmpDebugWrapper<C>>,
        >,
        &Lhs3,
        &Rhs3,
    >
where
    C::Error: CmpDisplay<C, Lhs0, Rhs0>,
{
    type VTable = expr::CustomCmpExpr<
        (PtrToDisplay, PtrToCmp),
        (PtrToDebug, PtrToDeref),
        (PtrToDebug, PtrToDeref),
    >;
    const NULL_VTABLE: &'static Self::VTable = &expr::CustomCmpExpr {
        cmp: (
            as_display_vptr::<
                spec::by_val::CmpByValWrapper<
                    spec::sized::CmpSizedWrapper<spec::debug::CmpDebugWrapper<C>>,
                >,
                Wrapper<Lhs3>,
                Wrapper<Rhs3>,
                spec::by_val::CmpByValWrapper<
                    spec::sized::CmpSizedWrapper<spec::debug::CmpDebugWrapper<C::Error>>,
                >,
            >,
            as_cmp_vptr::<
                ByVal<ByVal<crate::Single<()>>>,
                ByVal<ByVal<crate::Single<()>>>,
                crate::Eq,
            >,
        ),
        lhs: (as_debug_vptr::<Lhs3>, Lhs3::VTABLE),
        rhs: (as_debug_vptr::<Rhs3>, Rhs3::VTABLE),
    };
}

impl<
        Lhs0: ?Sized,
        Lhs1: ?Sized + core::ops::Deref<Target = Lhs0>,
        Lhs2: core::ops::Deref<Target = Lhs1>,
        Lhs3: core::ops::Deref<Target = Lhs2> + fmt::Debug + DerefVTable,
        Rhs0: ?Sized,
        Rhs1: ?Sized + core::ops::Deref<Target = Rhs0>,
        Rhs2: core::ops::Deref<Target = Rhs1>,
        Rhs3: core::ops::Deref<Target = Rhs2> + fmt::Debug + DerefVTable,
        C: Cmp<Lhs0, Rhs0>,
    > DynInfo
    for expr::CmpExpr<
        &spec::by_val::CmpByValWrapper<
            spec::sized::CmpSizedWrapper<spec::debug::CmpDebugWrapper<C>>,
        >,
        &Lhs3,
        &Rhs3,
    >
where
    C::Error: CmpDisplay<C, Single<dyn fmt::Debug>, Single<dyn fmt::Debug>>,
{
    const VTABLE: &'static Self::VTable = &expr::CmpExpr {
        cmp: (
            as_dyn_display_vptr::<
                spec::by_val::CmpByValWrapper<
                    spec::sized::CmpSizedWrapper<spec::debug::CmpDebugWrapper<C>>,
                >,
                spec::by_val::CmpByValWrapper<
                    spec::sized::CmpSizedWrapper<spec::debug::CmpDebugWrapper<C::Error>>,
                >,
            >,
            as_cmp_vptr::<Lhs2, Rhs2, C>,
        ),
        lhs: (as_debug_vptr::<Lhs3>, Lhs3::VTABLE),
        rhs: (as_debug_vptr::<Rhs3>, Rhs3::VTABLE),
    };
}

impl<
        Lhs0: ?Sized,
        Lhs1: ?Sized + core::ops::Deref<Target = Lhs0>,
        Lhs2: core::ops::Deref<Target = Lhs1>,
        Lhs3: core::ops::Deref<Target = Lhs2> + fmt::Debug + DerefVTable,
        Rhs0: ?Sized,
        Rhs1: ?Sized + core::ops::Deref<Target = Rhs0>,
        Rhs2: core::ops::Deref<Target = Rhs1>,
        Rhs3: core::ops::Deref<Target = Rhs2> + fmt::Debug + DerefVTable,
        C: Cmp<Lhs0, Rhs0>,
    > DynInfo
    for expr::CustomCmpExpr<
        &spec::by_val::CmpByValWrapper<
            spec::sized::CmpSizedWrapper<spec::debug::CmpDebugWrapper<C>>,
        >,
        &Lhs3,
        &Rhs3,
    >
where
    C::Error: CmpDisplay<C, Lhs0, Rhs0>,
{
    const VTABLE: &'static Self::VTable = &expr::CustomCmpExpr {
        cmp: (
            as_display_vptr::<
                spec::by_val::CmpByValWrapper<
                    spec::sized::CmpSizedWrapper<spec::debug::CmpDebugWrapper<C>>,
                >,
                Wrapper<Lhs3>,
                Wrapper<Rhs3>,
                spec::by_val::CmpByValWrapper<
                    spec::sized::CmpSizedWrapper<spec::debug::CmpDebugWrapper<C::Error>>,
                >,
            >,
            as_cmp_vptr::<Lhs2, Rhs2, C>,
        ),
        lhs: (as_debug_vptr::<Lhs3>, Lhs3::VTABLE),
        rhs: (as_debug_vptr::<Rhs3>, Rhs3::VTABLE),
    };
}
