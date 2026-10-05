// { dg-additional-options "-fno-inline" }
// A const fn can read memory through a reference, so two calls with the
// same reference must not be merged across a write to it.
#![feature(no_core)]
#![no_core]

const fn read(x: &i32) -> i32 {
    *x
}

fn twice(v: &mut i32) -> i32 {
    let a = read(v);
    *v = 2;
    let b = read(v);
    a * 10 + b
}

fn main() -> i32 {
    let mut v = 1;
    if twice(&mut v) != 12 {
        return 1;
    }

    0
}
