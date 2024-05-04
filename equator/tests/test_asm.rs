use core::hint::black_box;
use std::collections::HashMap;

#[inline(never)]
#[track_caller]
pub fn test_asm_1(a: usize, b: usize) {
    equator::assert!(a == b, "failed with first = {} and second = {b}", a);
}

#[inline(never)]
#[track_caller]
pub fn test_asm_2(a: usize, b: usize, c: usize, d: usize) {
    equator::assert!(all(a == b, c == d));
}

#[inline(never)]
#[track_caller]
pub fn test_std_asm_1(a: usize, b: usize) {
    assert_eq!(a, b);
}

#[inline(never)]
#[track_caller]
pub fn test_std_asm_2(a: usize, b: usize, c: usize, d: usize) {
    assert_eq!(a, b);
    assert_eq!(c, d);
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
    equator::assert!(all(true == false, x < y, any(!z, z)));
    equator::assert!(all(true == false, x + 1 < y, any(!z, z)));

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
    equator::assert!([0, 1, 2].as_slice() == &[0, 1]);
}

#[test]
pub fn test_move() {
    let ref mut m = HashMap::<usize, Vec<()>>::new();
    let x = vec![];
    equator::assert!(m.insert(0, x).is_none());
}
