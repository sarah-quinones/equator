use core::hint::black_box;

#[inline(never)]
#[track_caller]
pub fn test_asm_1(a: usize, b: usize) -> usize {
    equator::assert!(a == b);
    a * b
}

#[inline(never)]
#[track_caller]
pub fn test_asm_2(a: usize, b: usize, c: usize, d: usize) {
    equator::assert!(all(a == b, c == d));
}

#[inline(never)]
#[track_caller]
pub fn test_std_asm_1(a: usize, b: usize) -> usize {
    assert_eq!(a, b);
    a * b
}

#[inline(never)]
#[track_caller]
pub fn test_std_asm_2(a: usize, b: usize, c: usize, d: usize) {
    assert_eq!(a, b);
    assert_eq!(c, d);
}

#[test]
#[should_panic]
pub fn test() {
    let x = 3;
    let y = 2;
    let z = true;
    equator::assert!(all(true == false, x < y, any(!z, z)));

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
}
