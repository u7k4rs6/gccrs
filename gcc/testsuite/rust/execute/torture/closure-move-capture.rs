// A move closure captures its variables by value, so it can outlive the
// frame that created it and does not see later changes to the variables.
#![feature(no_core, lang_items, unboxed_closures)]
#![no_core]

#[lang = "sized"]
pub trait Sized {}

#[lang = "fn_once"]
pub trait FnOnce<Args> {
    #[lang = "fn_once_output"]
    type Output;

    extern "rust-call" fn call_once(self, args: Args) -> Self::Output;
}

struct Pair {
    a: i32,
    b: i32,
}

fn make(n: i32) -> impl FnOnce() -> i32 {
    let value = n;
    move || value
}

fn make_pair(a: i32, b: i32) -> impl FnOnce(i32) -> i32 {
    let p = Pair { a, b };
    let scale = 10;
    move |x: i32| (p.a * scale + p.b) * x
}

fn other_frame(a: i32, b: i32, c: i32, d: i32) -> i32 {
    let x = [a, b, c, d, a, b, c, d];
    x[0] + x[7]
}

fn main() -> i32 {
    // The closure owns its copy of value after make returns.
    let f = make(123);
    other_frame(7, 7, 7, 7);
    if f() != 123 {
        return 1;
    }

    let g = make_pair(4, 2);
    other_frame(9, 9, 9, 9);
    if g(2) != 84 {
        return 2;
    }

    // A move closure keeps the value it captured.
    let mut v = 1;
    let h = move || v;
    v = 2;
    if h() != 1 || v != 2 {
        return 3;
    }

    // A move closure changes its own copy, not the variable.
    let mut x = 1;
    let mut c = move || {
        x += 10;
        x
    };
    if c() != 11 || x != 1 {
        return 5;
    }

    // Capturing a reference by value still reads through it.
    let y = 7;
    let r = &y;
    let d = move || *r;
    if d() != 7 {
        return 6;
    }

    // A closure without move still captures by reference.
    let mut count = 0;
    let mut inc = || count += 5;
    inc();
    if count != 5 {
        return 4;
    }

    0
}
