// The assigned value of an assignment is evaluated before the place.
#![feature(no_core)]
#![no_core]

static mut N: i32 = 0;

fn tick() -> i32 {
    unsafe {
        N += 1;
        N
    }
}

fn main() -> i32 {
    let mut arr = [0i32; 4];

    // The value runs first (tick() == 1), then the index (tick() == 2).
    arr[tick() as usize] = tick();
    if arr[0] != 0 || arr[1] != 0 || arr[2] != 1 || arr[3] != 0 {
        return 1;
    }

    unsafe { N = 0 };
    let mut arr = [0i32; 4];
    arr[tick() as usize] = tick() * 10;
    if arr[0] != 0 || arr[1] != 0 || arr[2] != 10 || arr[3] != 0 {
        return 2;
    }

    // The value only reads i, but the place changes it, so the value must
    // still be evaluated first and see i == 0.
    let mut i = 0usize;
    let mut arr = [0usize; 4];
    arr[{
        i += 1;
        i
    }] = i * 10;
    if arr[1] != 0 {
        return 3;
    }

    0
}
