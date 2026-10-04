#![feature(no_core)]
#![no_core]

const fn get(a: [i32; 4], i: usize) -> i32 {
    a[i] // { dg-error {evaluation of constant value failed \[E0080\]} }
}

const IN_BOUNDS: i32 = get([1, 2, 3, 4], 3);
const OUT_OF_BOUNDS: i32 = get([1, 2, 3, 4], 9);

fn main() -> i32 {
    IN_BOUNDS + OUT_OF_BOUNDS
}
