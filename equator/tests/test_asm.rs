use core::hint::black_box;
use equator::assert;
use std::collections::HashMap;

#[inline(never)]
#[track_caller]
pub fn test_asm_2(a: usize, b: usize, c: usize, d: usize) {
    assert!(all(a == b, c == d));
}

#[inline(never)]
#[track_caller]
pub fn test_asm_3(a: usize, b: usize, c: usize, d: usize) {
    assert!(all(a + 1 == b - 3, c + 4 == d - 4));
}

#[inline(never)]
#[track_caller]
pub fn test_std_asm_1(a: usize, b: usize) {
    std::assert_eq!(a, b);
}

#[inline(never)]
#[track_caller]
pub fn test_std_asm_2(a: usize, b: usize, c: usize, d: usize) {
    std::assert_eq!(a, b);
    std::assert_eq!(c, d);
}

#[inline(never)]
#[track_caller]
pub fn test_std_asm_3(a: usize, b: usize, c: usize, d: usize) {
    std::assert_eq!(a + 1, b - 3);
    std::assert_eq!(c + 4, d - 4);
}

#[inline(never)]
#[track_caller]
pub fn test_assert2_asm_1(a: usize, b: usize) {
    assert2::assert!(a == b);
}

#[inline(never)]
#[track_caller]
pub fn test_assert2_asm_2(a: usize, b: usize, c: usize, d: usize) {
    assert2::assert!(a == b);
    assert2::assert!(c == d);
}

#[test]
#[should_panic]
pub fn test() {
    let x = 3;
    let y = 2;
    let z = true;
    if black_box(true) {
        assert!(all(true == false, x < y, any(!z, z)));
        assert!(all(true == false, x + 1 < y, any(!z, z)));
    }

    black_box(test_asm_1(black_box(2), black_box(4)));
    black_box(test_asm_2(
        black_box(3),
        black_box(5),
        black_box(6),
        black_box(7),
    ));
    black_box(test_asm_3(
        black_box(3),
        black_box(5),
        black_box(6),
        black_box(7),
    ));
    black_box(test_std_asm_1(black_box(2), black_box(4)));
    black_box(test_std_asm_2(
        black_box(3),
        black_box(5),
        black_box(6),
        black_box(7),
    ));
    black_box(test_std_asm_3(
        black_box(3),
        black_box(5),
        black_box(6),
        black_box(7),
    ));
}

#[test]
#[should_panic]
pub fn test_asm() {
    black_box(test_asm_1(black_box(2), black_box(4)));
    black_box(test_asm_2(
        black_box(3),
        black_box(5),
        black_box(6),
        black_box(7),
    ));
    black_box(test_std_asm_1(black_box(2), black_box(4)));
    black_box(test_std_asm_2(
        black_box(3),
        black_box(5),
        black_box(6),
        black_box(7),
    ));
    black_box(test_assert2_asm_1(black_box(2), black_box(4)));
    black_box(test_assert2_asm_2(
        black_box(3),
        black_box(5),
        black_box(6),
        black_box(7),
    ));
}

#[test]
#[should_panic]
pub fn test_different_types() {
    assert!(*[0, 1, 2].as_slice() == [0, 1]);
}

#[derive(Copy, Clone)]
struct ApproxEq {
    symbol: &'static str,
    tol: f64,
}

impl<Lhs, Rhs> equator::Cmp<&Lhs, &Rhs> for ApproxEq
where
    ApproxEq: equator::Cmp<Lhs, Rhs>,
{
    fn test(&self, lhs: &&Lhs, rhs: &&Rhs) -> bool {
        self.test(*lhs, *rhs)
    }
}
impl equator::Cmp<f64, f64> for ApproxEq {
    fn test(&self, lhs: &f64, rhs: &f64) -> bool {
        (lhs - rhs).abs() < self.tol
    }
}
impl equator::DisplayCmp for ApproxEq {
    fn fmt(&self, lhs: &str, rhs: &str, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let Self { symbol, tol } = *self;
        write!(f, "{lhs} {symbol} {rhs}, with absolute tolerance {tol:.1e}")
    }
}

#[test]
pub fn test_custom() {
    let approx_eq = ApproxEq {
        tol: 0.01,
        symbol: "~",
    };

    assert!(0.1 :approx_eq: 0.10001);
    assert!(&0.1 :approx_eq: &0.10001);
    assert!(0.1 ~ 0.10001);
}

#[test]
#[should_panic]
pub fn test_custom_fail() {
    let approx_eq = ApproxEq {
        tol: 0.01,
        symbol: "~",
    };

    let x = 0.1;
    assert!(x ~ 0.2);
}

#[test]
pub fn test_move() {
    let ref mut m = HashMap::<usize, Vec<()>>::new();
    let x = vec![];
    assert!(*x == []);
    assert!(m.insert(0, x).is_none());
}

#[inline(never)]
#[track_caller]
pub fn test_asm_1(a: usize, b: usize) {
    assert!(a == b);
}
