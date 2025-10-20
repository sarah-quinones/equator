#![no_std]

use core::fmt;

#[doc(hidden)]
pub use equator_macro as imp;

#[macro_export]
#[doc(hidden)]
macro_rules! fmt_imp {
	() => {
		()
	};
	($($tt:tt)*) => {
		::core::format_args!($($tt)*)
	};
}

#[macro_export]
#[doc(hidden)]
macro_rules! source_imp {
	(all()) => { () };
	(any()) => { () };

	(all(($($head:tt)*))) => { $crate::source_imp!($($head)*) };
	(any(($($head:tt)*))) => { $crate::source_imp!($($head)*) };

	(all(($($head:tt)*) $($tail:tt)+)) => {
		const{$crate::expr::AndExpr {
			lhs: &$crate::source_imp!($($head)*),
			rhs: &$crate::source_imp!(all($($tail)+)),
		}}
	};
	(any(($($head:tt)*) $($tail:tt)+)) => {
		const{$crate::expr::OrExpr {
			lhs: &$crate::source_imp!($($head)*),
			rhs: &$crate::source_imp!(any($($tail)+)),
		}}
	};

	(<($lhs: expr, $rhs: expr)) => {
		const{$crate::expr::CmpExpr {
			cmp: "<",
			lhs: ::core::stringify!($lhs),
			rhs: ::core::stringify!($rhs),
		}}
	};
	(>($lhs: expr, $rhs: expr)) => {
		const{$crate::expr::CmpExpr {
			cmp: ">",
			lhs: ::core::stringify!($lhs),
			rhs: ::core::stringify!($rhs),
		}}
	};
	(<=($lhs: expr, $rhs: expr)) => {
		const{$crate::expr::CmpExpr {
			cmp: "<=",
			lhs: ::core::stringify!($lhs),
			rhs: ::core::stringify!($rhs),
		}}
	};
	(>=($lhs: expr, $rhs: expr)) => {
		const{$crate::expr::CmpExpr {
			cmp: ">=",
			lhs: ::core::stringify!($lhs),
			rhs: ::core::stringify!($rhs),
		}}
	};
	(==($lhs: expr, $rhs: expr)) => {
		const{$crate::expr::CmpExpr {
			cmp: "==",
			lhs: ::core::stringify!($lhs),
			rhs: ::core::stringify!($rhs),
		}}
	};
	(!=($lhs: expr, $rhs: expr)) => {
		const{$crate::expr::CmpExpr {
			cmp: "!=",
			lhs: ::core::stringify!($lhs),
			rhs: ::core::stringify!($rhs),
		}}
	};
	($test:expr,($lhs: expr, $rhs: expr)) => {const{$crate::expr::CmpExpr {
			cmp: ::core::stringify!($test),
			lhs: ::core::stringify!($lhs),
			rhs: ::core::stringify!($rhs),
		}}};
	($test:expr,~($lhs: expr, $rhs: expr)) => {const{$crate::expr::CmpExpr {
			cmp: "~",
			lhs: ::core::stringify!($lhs),
			rhs: ::core::stringify!($rhs),
		}}};
	($cond:expr) => { ::core::stringify!($cond) };
}

#[macro_export]
#[doc(hidden)]
macro_rules! assert_imp {
	(all()) => { true };
	(any()) => { false };

	(all(($($head:tt)*))) => { $crate::assert_imp!($($head)*) };
	(any(($($head:tt)*))) => { $crate::assert_imp!($($head)*) };

	(all(($($head:tt)*) $($tail:tt)+)) => {
		$crate::expr::AndExpr {
			lhs: $crate::assert_imp!($($head)*),
			rhs: $crate::assert_imp!(all($($tail)+)),
		}
	};
	(any(($($head:tt)*) $($tail:tt)+)) => {
		$crate::expr::OrExpr {
			lhs: $crate::assert_imp!($($head)*),
			rhs: $crate::assert_imp!(any($($tail)+)),
		}
	};

	(<($lhs: expr, $rhs: expr)) => {
		$crate::expr::CmpExpr {
			cmp: &$crate::Lt,
			lhs: $crate::Ref{inner: &$lhs}.get(),
			rhs: $crate::Ref{inner: &$rhs}.get(),
		}
	};
	(>($lhs: expr, $rhs: expr)) => {
		$crate::expr::CmpExpr {
			cmp: &$crate::Gt,
			lhs: $crate::Ref{inner: &$lhs}.get(),
			rhs: $crate::Ref{inner: &$rhs}.get(),
		}
	};
	(<=($lhs: expr, $rhs: expr)) => {
		$crate::expr::CmpExpr {
			cmp: &$crate::Le,
			lhs: $crate::Ref{inner: &$lhs}.get(),
			rhs: $crate::Ref{inner: &$rhs}.get(),
		}
	};
	(>=($lhs: expr, $rhs: expr)) => {
		$crate::expr::CmpExpr {
			cmp: &$crate::Ge,
			lhs: $crate::Ref{inner: &$lhs}.get(),
			rhs: $crate::Ref{inner: &$rhs}.get(),
		}
	};
	(==($lhs: expr, $rhs: expr)) => {
		$crate::expr::CmpExpr {
			cmp: &$crate::Eq,
			lhs: $crate::Ref{inner: &$lhs}.get(),
			rhs: $crate::Ref{inner: &$rhs}.get(),
		}
	};
	(!=($lhs: expr, $rhs: expr)) => {
		$crate::expr::CmpExpr {
			cmp: &$crate::Ne,
			lhs: $crate::Ref{inner: &$lhs}.get(),
			rhs: $crate::Ref{inner: &$rhs}.get(),
		}
	};
	($test:expr,($lhs: expr, $rhs: expr)) => {$crate::expr::CmpExpr {
			cmp: &$test,
			lhs: $crate::Ref{inner: &$lhs}.get(),
			rhs: $crate::Ref{inner: &$rhs}.get(),
		}};
	($test:expr,~($lhs: expr, $rhs: expr)) => {$crate::expr::CmpExpr {
			cmp: &$test,
			lhs: $crate::Ref{inner: &$lhs}.get(),
			rhs: $crate::Ref{inner: &$rhs}.get(),
		}};
	($cond:expr) => { $cond };
}

#[macro_export]
macro_rules! assert {
    ($($tokens: tt)*) => {
        $crate::imp::assert!(($crate) $($tokens)*)
    };
}

#[macro_export]
macro_rules! debug_assert {
    ($($tokens: tt)*) => {
        if cfg!(debug_assertions) {
	        $crate::imp::assert!(($crate) $($tokens)*)
        }
    };
}

#[doc(hidden)]
// pub mod decompose;
#[doc(hidden)]
// pub mod spec;
#[doc(hidden)]
// pub mod structures;
#[doc(hidden)]
// pub mod traits;
#[doc(hidden)]
pub mod expr {
	#[derive(Copy, Clone, Debug)]
	#[repr(C)]
	pub struct CmpExpr<Cmp, Lhs, Rhs> {
		pub cmp: Cmp,
		pub lhs: Lhs,
		pub rhs: Rhs,
	}

	#[derive(Copy, Clone)]
	#[repr(C)]
	pub struct Cmp {
		pub cmp: *const (),
		pub lhs: core::mem::MaybeUninit<*const ()>,
		pub rhs: core::mem::MaybeUninit<*const ()>,
	}

	#[derive(Copy, Clone)]
	#[repr(C)]
	pub struct CustomCmp {
		pub cmp: *const (),
		pub lhs: *const (),
		pub rhs: *const (),
	}

	#[derive(Copy, Clone, Debug)]
	#[repr(C)]
	pub struct CustomCmpExpr<Cmp, Lhs, Rhs> {
		pub cmp: Cmp,
		pub lhs: Lhs,
		pub rhs: Rhs,
	}

	#[derive(Copy, Clone, Debug)]
	pub struct AndExpr<Lhs, Rhs> {
		pub lhs: Lhs,
		pub rhs: Rhs,
	}

	#[derive(Copy, Clone, Debug)]
	pub struct OrExpr<Lhs, Rhs> {
		pub lhs: Lhs,
		pub rhs: Rhs,
	}
}

pub trait Cmp<Lhs: ?Sized, Rhs: ?Sized> {
	fn test(&self, lhs: &Lhs, rhs: &Rhs) -> bool;
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

#[doc(hidden)]
pub struct CmpExpr;
#[doc(hidden)]
pub struct CustomCmpExpr<E>(pub core::marker::PhantomData<E>);
#[doc(hidden)]
pub struct AndExpr<L, R>(pub L, pub R);
#[doc(hidden)]
pub struct OrExpr<L, R>(pub L, pub R);

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

pub trait Panic: Copy {
	type Lhs: Copy;
	type Rhs: Copy;
	type Cmp: Copy;
	type Source: 'static;
	type VTable: 'static;
	type Result: Copy;

	fn into_parts(self) -> (Self::Lhs, Self::Rhs, Self::Cmp);
	unsafe fn from_parts(lhs: Self::Lhs, rhs: Self::Rhs, cmp: Self::Cmp) -> Self;
	fn explain(self, source: &Self::Source, result: Self::Result, vtable: &'static Self::VTable, f: &mut fmt::Formatter) -> fmt::Result;
	fn test(&self, vtable: &'static Self::VTable) -> Self::Result;
	fn reduce(result: Self::Result) -> bool;
}

pub trait Test: Sized {
	type Panic: Panic;
	const VTABLE: &'static <Self::Panic as Panic>::VTable;

	fn test(&self) -> bool;
	fn as_dyn(self) -> Self::Panic;
}

pub trait Fmt<'a>: Sized {
	fn fmt(&self) -> &core::fmt::Arguments<'a>;
}

impl<'a> Fmt<'a> for () {
	#[inline(always)]
	fn fmt(&self) -> &core::fmt::Arguments<'a> {
		const { &format_args!("") }
	}
}

impl<'a> Fmt<'a> for core::fmt::Arguments<'a> {
	#[inline(always)]
	fn fmt(&self) -> &core::fmt::Arguments<'a> {
		self
	}
}

impl Panic for bool {
	type Cmp = ();
	type Lhs = bool;
	type Result = bool;
	type Rhs = ();
	type Source = &'static str;
	type VTable = ();

	#[inline(always)]
	fn into_parts(self) -> (Self::Lhs, Self::Rhs, Self::Cmp) {
		(self, (), ())
	}

	#[inline(always)]
	unsafe fn from_parts(lhs: Self::Lhs, _: Self::Rhs, _: Self::Cmp) -> Self {
		lhs
	}

	fn explain(self, source: &Self::Source, result: Self::Result, _: &'static Self::VTable, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "Assertion failed: {source}\n")?;
		write!(f, "- {source} = {result:#?}")
	}

	fn test(&self, _: &'static Self::VTable) -> Self::Result {
		*self
	}

	fn reduce(result: Self::Result) -> bool {
		result
	}
}

impl Test for bool {
	type Panic = bool;

	const VTABLE: &'static <Self::Panic as Panic>::VTable = &();

	#[inline(always)]
	fn test(&self) -> bool {
		*self
	}

	#[inline(always)]
	fn as_dyn(self) -> Self::Panic {
		self
	}
}

impl Panic for expr::Cmp {
	type Cmp = *const ();
	type Lhs = core::mem::MaybeUninit<*const ()>;
	type Result = bool;
	type Rhs = core::mem::MaybeUninit<*const ()>;
	type Source = expr::CmpExpr<&'static str, &'static str, &'static str>;
	type VTable = expr::CmpExpr<
		fn(*const (), core::mem::MaybeUninit<*const ()>, core::mem::MaybeUninit<*const ()>) -> bool,
		fn(core::mem::MaybeUninit<*const ()>, f: &mut fmt::Formatter<'_>) -> fmt::Result,
		fn(core::mem::MaybeUninit<*const ()>, f: &mut fmt::Formatter<'_>) -> fmt::Result,
	>;

	#[inline(always)]
	fn into_parts(self) -> (Self::Lhs, Self::Rhs, Self::Cmp) {
		(self.lhs, self.rhs, self.cmp)
	}

	#[inline(always)]
	unsafe fn from_parts(lhs: Self::Lhs, rhs: Self::Rhs, cmp: Self::Cmp) -> Self {
		Self { cmp, lhs, rhs }
	}

	fn explain(self, source: &Self::Source, _: Self::Result, vtable: &'static Self::VTable, f: &mut fmt::Formatter) -> fmt::Result {
		let expr::CmpExpr { lhs, rhs, cmp } = *source;
		write!(f, "Assertion failed: {lhs} {cmp} {rhs}\n")?;
		write!(f, "- {lhs} = ")?;
		(vtable.lhs)(self.lhs, f)?;
		write!(f, "\n- {rhs} = ")?;
		(vtable.rhs)(self.rhs, f)
	}

	fn test(&self, vtable: &'static Self::VTable) -> Self::Result {
		(vtable.cmp)(self.cmp, self.lhs, self.rhs)
	}

	fn reduce(result: Self::Result) -> bool {
		result
	}
}

impl<C: Cmp<L, R>, L: fmt::Debug, R: fmt::Debug> Test for expr::CmpExpr<&C, &L, &R> {
	type Panic = expr::Cmp;

	const VTABLE: &'static <Self::Panic as Panic>::VTable = &const {
		unsafe {
			if const {
				size_of::<L>() <= size_of::<core::mem::MaybeUninit<*const ()>>()
					&& size_of::<R>() <= size_of::<core::mem::MaybeUninit<*const ()>>()
					&& !core::mem::needs_drop::<L>()
					&& !core::mem::needs_drop::<R>()
			} {
				expr::CmpExpr {
					cmp: core::mem::transmute(
						(|c, l, r| C::test(c, &*(&raw const l as *const L), &*(&raw const r as *const R)))
							as fn(&C, core::mem::MaybeUninit<*const ()>, core::mem::MaybeUninit<*const ()>) -> bool,
					),
					lhs: core::mem::transmute(
						(|x, f| L::fmt(&*(&raw const x as *const L), f))
							as fn(core::mem::MaybeUninit<*const ()>, &mut fmt::Formatter<'_>) -> fmt::Result,
					),
					rhs: core::mem::transmute(
						(|x, f| R::fmt(&*(&raw const x as *const R), f))
							as fn(core::mem::MaybeUninit<*const ()>, &mut fmt::Formatter<'_>) -> fmt::Result,
					),
				}
			} else {
				expr::CmpExpr {
					cmp: core::mem::transmute(C::test as fn(&C, &L, &R) -> bool),
					lhs: core::mem::transmute(L::fmt as fn(&L, &mut fmt::Formatter<'_>) -> fmt::Result),
					rhs: core::mem::transmute(R::fmt as fn(&R, &mut fmt::Formatter<'_>) -> fmt::Result),
				}
			}
		}
	};

	#[inline(always)]
	fn test(&self) -> bool {
		self.cmp.test(&self.lhs, &self.rhs)
	}

	#[inline(always)]
	fn as_dyn(self) -> Self::Panic {
		if const {
			size_of::<L>() <= size_of::<core::mem::MaybeUninit<*const ()>>()
				&& size_of::<R>() <= size_of::<core::mem::MaybeUninit<*const ()>>()
				&& !core::mem::needs_drop::<L>()
				&& !core::mem::needs_drop::<R>()
		} {
			let mut uninit = expr::Cmp {
				cmp: self.cmp as *const C as *const (),
				lhs: core::mem::MaybeUninit::zeroed(),
				rhs: core::mem::MaybeUninit::zeroed(),
			};
			unsafe {
				(&raw mut uninit.lhs as *mut L).write((&raw const *self.lhs).read());
				(&raw mut uninit.rhs as *mut R).write((&raw const *self.rhs).read());
			}
			uninit
		} else {
			expr::Cmp {
				cmp: self.cmp as *const C as *const (),
				lhs: core::mem::MaybeUninit::new(self.lhs as *const L as *const ()),
				rhs: core::mem::MaybeUninit::new(self.rhs as *const R as *const ()),
			}
		}
	}
}

impl<L: Panic, R: Panic> Panic for expr::AndExpr<L, R> {
	type Cmp = expr::AndExpr<L::Cmp, R::Cmp>;
	type Lhs = expr::AndExpr<L::Lhs, R::Lhs>;
	type Result = expr::AndExpr<L::Result, R::Result>;
	type Rhs = expr::AndExpr<L::Rhs, R::Rhs>;
	type Source = expr::AndExpr<&'static L::Source, &'static R::Source>;
	type VTable = expr::AndExpr<&'static L::VTable, &'static R::VTable>;

	#[inline(always)]
	fn into_parts(self) -> (Self::Lhs, Self::Rhs, Self::Cmp) {
		let lhs = self.lhs.into_parts();
		let rhs = self.rhs.into_parts();

		(
			expr::AndExpr { lhs: lhs.0, rhs: rhs.0 },
			expr::AndExpr { lhs: lhs.1, rhs: rhs.1 },
			expr::AndExpr { lhs: lhs.2, rhs: rhs.2 },
		)
	}

	#[inline(always)]
	unsafe fn from_parts(lhs: Self::Lhs, rhs: Self::Rhs, cmp: Self::Cmp) -> Self {
		Self {
			lhs: L::from_parts(lhs.lhs, rhs.lhs, cmp.lhs),
			rhs: R::from_parts(lhs.rhs, rhs.rhs, cmp.rhs),
		}
	}

	fn explain(self, source: &Self::Source, result: Self::Result, vtable: &'static Self::VTable, f: &mut fmt::Formatter) -> fmt::Result {
		let lhs_eval = L::reduce(result.lhs);
		let rhs_eval = R::reduce(result.rhs);
		if !(lhs_eval && rhs_eval) {
			if !lhs_eval {
				self.lhs.explain(source.lhs, result.lhs, vtable.lhs, f)?;
				if !rhs_eval {
					f.write_str("\n")?;
				}
			}
			if !rhs_eval {
				self.rhs.explain(source.rhs, result.rhs, vtable.rhs, f)?;
			}
		}
		Ok(())
	}

	fn test(&self, vtable: &'static Self::VTable) -> Self::Result {
		Self::Result {
			lhs: self.lhs.test(vtable.lhs),
			rhs: self.rhs.test(vtable.rhs),
		}
	}

	fn reduce(result: Self::Result) -> bool {
		L::reduce(result.lhs) && R::reduce(result.rhs)
	}
}

impl<L: Test, R: Test> Test for expr::AndExpr<L, R> {
	type Panic = expr::AndExpr<L::Panic, R::Panic>;

	const VTABLE: &'static <Self::Panic as Panic>::VTable = &const {
		expr::AndExpr {
			lhs: L::VTABLE,
			rhs: R::VTABLE,
		}
	};

	#[inline(always)]
	fn test(&self) -> bool {
		self.lhs.test() & self.rhs.test()
	}

	#[inline(always)]
	fn as_dyn(self) -> Self::Panic {
		Self::Panic {
			lhs: self.lhs.as_dyn(),
			rhs: self.rhs.as_dyn(),
		}
	}
}

impl<L: Panic, R: Panic> Panic for expr::OrExpr<L, R> {
	type Cmp = expr::OrExpr<L::Cmp, R::Cmp>;
	type Lhs = expr::OrExpr<L::Lhs, R::Lhs>;
	type Result = expr::OrExpr<L::Result, R::Result>;
	type Rhs = expr::OrExpr<L::Rhs, R::Rhs>;
	type Source = expr::OrExpr<&'static L::Source, &'static R::Source>;
	type VTable = expr::OrExpr<&'static L::VTable, &'static R::VTable>;

	#[inline(always)]
	fn into_parts(self) -> (Self::Lhs, Self::Rhs, Self::Cmp) {
		let lhs = self.lhs.into_parts();
		let rhs = self.rhs.into_parts();

		(
			expr::OrExpr { lhs: lhs.0, rhs: rhs.0 },
			expr::OrExpr { lhs: lhs.1, rhs: rhs.1 },
			expr::OrExpr { lhs: lhs.2, rhs: rhs.2 },
		)
	}

	#[inline(always)]
	unsafe fn from_parts(lhs: Self::Lhs, rhs: Self::Rhs, cmp: Self::Cmp) -> Self {
		Self {
			lhs: L::from_parts(lhs.lhs, rhs.lhs, cmp.lhs),
			rhs: R::from_parts(lhs.rhs, rhs.rhs, cmp.rhs),
		}
	}

	fn explain(self, source: &Self::Source, result: Self::Result, vtable: &'static Self::VTable, f: &mut fmt::Formatter) -> fmt::Result {
		let lhs_eval = L::reduce(result.lhs);
		let rhs_eval = R::reduce(result.rhs);
		if !(lhs_eval || rhs_eval) {
			self.lhs.explain(source.lhs, result.lhs, vtable.lhs, f)?;
			self.rhs.explain(source.rhs, result.rhs, vtable.rhs, f)?;
		}
		Ok(())
	}

	fn test(&self, vtable: &'static Self::VTable) -> Self::Result {
		Self::Result {
			lhs: self.lhs.test(vtable.lhs),
			rhs: self.rhs.test(vtable.rhs),
		}
	}

	fn reduce(result: Self::Result) -> bool {
		L::reduce(result.lhs) || R::reduce(result.rhs)
	}
}

impl<L: Test, R: Test> Test for expr::OrExpr<L, R> {
	type Panic = expr::OrExpr<L::Panic, R::Panic>;

	const VTABLE: &'static <Self::Panic as Panic>::VTable = &const {
		expr::OrExpr {
			lhs: L::VTABLE,
			rhs: R::VTABLE,
		}
	};

	#[inline(always)]
	fn test(&self) -> bool {
		self.lhs.test() | self.rhs.test()
	}

	#[inline(always)]
	fn as_dyn(self) -> Self::Panic {
		Self::Panic {
			lhs: self.lhs.as_dyn(),
			rhs: self.rhs.as_dyn(),
		}
	}
}

#[track_caller]
#[inline(always)]
pub fn do_panic<'a, T: Test>(source: &'static WithSource<<T::Panic as Panic>::Source>, test: T, fmt: impl Fmt<'a>) {
	let success = test.test();
	let panic = test.as_dyn();
	let (lhs, rhs, cmp) = panic.into_parts();
	if !success {
		do_panic_impl::<T::Panic>(lhs, rhs, cmp, source, T::VTABLE, fmt.fmt())
	}
}

#[track_caller]
#[inline(never)]
fn do_panic_impl<'a, P: Panic>(
	lhs: P::Lhs,
	rhs: P::Rhs,
	cmp: P::Cmp,
	source: &'static WithSource<P::Source>,
	vtable: &'static P::VTable,
	fmt: &core::fmt::Arguments<'_>,
) -> ! {
	struct Debug<'a, P: Panic> {
		lhs: P::Lhs,
		rhs: P::Rhs,
		cmp: P::Cmp,
		source: &'static WithSource<P::Source>,
		vtable: &'static P::VTable,
		fmt: &'a core::fmt::Arguments<'a>,
	}
	impl<P: Panic> fmt::Debug for Debug<'_, P> {
		fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
			let Self {
				lhs,
				rhs,
				cmp,
				source,
				vtable,
				fmt,
			} = self;

			write!(f, "Assertion failed at {}:{}:{}\n", source.file, source.line, source.col)?;
			if fmt.as_str() != Some("") {
				write!(f, "{fmt:#?}\n")?;
			}
			let p = unsafe { P::from_parts(*lhs, *rhs, *cmp) };

			let result = p.test(vtable);
			p.explain(&source.source, result, vtable, f)
		}
	}

	panic!(
		"{:#?}",
		Debug::<P> {
			lhs,
			rhs,
			cmp,
			source,
			vtable,
			fmt
		}
	);
}

pub struct Ref<'a, T: ?Sized> {
	pub inner: &'a T,
}

impl<'a, T> Ref<'a, T> {
	#[inline(always)]
	pub const fn get(&self) -> &'a T {
		self.inner
	}
}

impl<'a, T> Ref<'a, [T]> {
	#[inline(always)]
	pub const fn get(&self) -> &&'a [T] {
		&self.inner
	}
}

#[derive(Copy, Clone)]
pub struct WithSource<S> {
	pub source: S,
	pub file: &'static str,
	pub line: u32,
	pub col: u32,
}
