// { dg-shouldfail "out of bounds array index" }
#![feature(no_core)]
#![no_core]

fn index() -> usize {
    4
}

fn main() -> i32 {
    let mut a = [0i32; 4];

    // Index 4 is one past the end of the array, so this must abort.
    a[index()] = 1;

    0
}
