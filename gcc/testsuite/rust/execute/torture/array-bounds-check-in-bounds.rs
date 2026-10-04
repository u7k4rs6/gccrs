// Indexing within bounds is unaffected by the bounds check, and the index
// is evaluated once.
#![feature(no_core)]
#![no_core]

static mut N: usize = 0;

fn next() -> usize {
    unsafe {
        N += 1;
        N - 1
    }
}

fn main() -> i32 {
    let mut a = [10i32, 20, 30, 40];
    a[3] = 41;
    a[next()] = 11;
    let r = &mut a[2];
    *r = 31;

    let mut m = [[0u8; 3]; 2];
    m[1][2] = 7;

    if a[0] != 11 || a[1] != 20 || a[2] != 31 || a[3] != 41 {
        return 1;
    }
    if m[1][2] != 7 || m[0][2] != 0 {
        return 2;
    }
    if unsafe { N } != 1 {
        return 3;
    }

    0
}
