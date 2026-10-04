// The place of a compound assignment is evaluated exactly once, after the
// right operand.
#![feature(no_core)]
#![no_core]

static mut N: i32 = 0;

fn tick() -> i32 {
    unsafe {
        N += 1;
        N
    }
}

fn pick(arr: &mut [i32; 4]) -> &mut i32 {
    tick();
    &mut arr[2]
}

fn main() -> i32 {
    let mut arr = [0i32; 4];

    // The right operand runs first (tick() == 1), then the index
    // (tick() == 2), and the index is not evaluated again for the store.
    arr[tick() as usize] += tick() * 10;
    if arr[0] != 0 || arr[1] != 0 || arr[2] != 10 || arr[3] != 0 {
        return 1;
    }
    if unsafe { N } != 2 {
        return 2;
    }

    unsafe { N = 0 };
    *pick(&mut arr) += 5;
    if arr[2] != 15 {
        return 3;
    }
    if unsafe { N } != 1 {
        return 4;
    }

    // The right operand only reads i, but the place changes it, so the right
    // operand must still be evaluated first and see i == 0.
    let mut i = 0usize;
    let mut b = [5usize; 4];
    b[{
        i += 1;
        i
    }] += i * 10;
    if b[1] != 5 {
        return 5;
    }

    0
}
