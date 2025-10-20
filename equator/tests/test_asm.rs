use core::hint::black_box;
use equator::assert;
use std::collections::HashMap;

#[test]
#[should_panic]
pub fn test() {
	black_box(test_asm_1_med as fn(_, _));
	black_box(test_asm_2_smol as fn(_, _, _, _));
	black_box(test_asm_2_med as fn(_, _, _, _));
	black_box(test_asm_2_big as fn(_, _, _, _));

	black_box(test_std_asm_1_med as fn(_, _));
	black_box(test_std_asm_2_smol as fn(_, _, _, _));
	black_box(test_std_asm_2_med as fn(_, _, _, _));
	black_box(test_std_asm_2_big as fn(_, _, _, _));

	black_box(test_assert2_asm_1_med as fn(_, _));
	black_box(test_assert2_asm_2_smol as fn(_, _, _, _));
	black_box(test_assert2_asm_2_med as fn(_, _, _, _));
	black_box(test_assert2_asm_2_big as fn(_, _, _, _));

	let x = 3;
	let y = 2;
	let z = true;

	assert!(true == false);
	assert!(all(true == false, x < y, any(!z, z)));
	assert!(all(true == false, x + 1 < y, any(!z, z)));
}

#[test]
#[should_panic]
pub fn test_different_types() {
	assert!(*[0, 1, 2usize].as_slice() == [0, 1usize]);
}

#[derive(Copy, Clone, Debug)]
struct ApproxEq {
	tol: f64,
}

impl equator::Cmp<f64, f64> for ApproxEq {
	fn test(&self, lhs: &f64, rhs: &f64) -> bool {
		(lhs - rhs).abs() <= self.tol
	}
}

#[test]
pub fn test_move() {
	let ref mut m = HashMap::<usize, Vec<()>>::new();
	let x = vec![];
	assert!(*x == []);
	assert!(*x == [], "oops {x:?}");
	assert!(m.insert(0, x).is_none());
}

#[inline(never)]
pub fn test_asm_1_med(a: usize, b: usize) {
	assert!(a == b);
}

#[inline(never)]
pub fn test_asm_2_smol(a: u8, b: u8, c: u8, d: u8) {
	assert!([a, c] == [b, d]);
}

#[inline(never)]
pub fn test_asm_2_med(a: usize, b: usize, c: usize, d: usize) {
	assert!(all(a == b, c == d));
}

#[inline(never)]
pub fn test_asm_2_big(a: usize, b: usize, c: usize, d: usize) {
	assert!([a, c] == [b, d]);
}

#[inline(never)]
pub fn test_std_asm_2_smol(a: u8, b: u8, c: u8, d: u8) {
	std::assert_eq!([a, c], [b, d]);
}

#[inline(never)]
pub fn test_std_asm_2_med(a: usize, b: usize, c: usize, d: usize) {
	std::assert_eq!(a, b);
	std::assert_eq!(c, d);
}

#[inline(never)]
pub fn test_std_asm_2_big(a: usize, b: usize, c: usize, d: usize) {
	std::assert_eq!([a, c], [b, d]);
}

#[inline(never)]
pub fn test_assert2_asm_2_smol(a: u8, b: u8, c: u8, d: u8) {
	assert2::assert!([a, c] == [b, d]);
}

#[inline(never)]
pub fn test_assert2_asm_2_med(a: usize, b: usize, c: usize, d: usize) {
	assert2::assert!(a == b);
	assert2::assert!(c == d);
}

#[inline(never)]
pub fn test_assert2_asm_2_big(a: usize, b: usize, c: usize, d: usize) {
	assert2::assert!([a, c] == [b, d]);
}

#[inline(never)]
pub fn test_std_asm_1_med(a: usize, b: usize) {
	std::assert_eq!(a, b);
}

#[inline(never)]
pub fn test_assert2_asm_1_med(a: usize, b: usize) {
	assert2::assert!(a == b);
}

#[test]
#[should_panic]
pub fn test_big_fail() {
	let x = [core::ptr::null::<()>(); 2];
	assert!(x != x);
}

#[test]
pub fn test_big() {
	let x = [core::ptr::null::<()>(); 2];
	assert!(x == x);
}

#[test]
#[should_panic]
pub fn test_custom_fail() {
	let approx_eq = ApproxEq { tol: 0.01 };

	let x = 0.1;
	assert!(all(x ~ 0.2, x ~ 0.1, x ~ 0.3));
}

#[test]
pub fn test_custom() {
	let approx_eq = ApproxEq { tol: 0.01 };

	assert!(0.1 :approx_eq: 0.10001);
	assert!(0.1 ~ 0.10001);
}
