#![feature(no_core)]
#![no_core]

// A guarded arm may not match, so it does not count as covering its pattern.

enum T {
    A(),
    B(),
}

fn all_guarded(t: T, c: bool) -> i32 {
    match t {
        // { dg-error "non-exhaustive patterns: 'T::A..' and 'T::B..' not covered" "" { target *-*-* } .-1 }
        T::A() if c => 1,
        T::B() if c => 2,
    }
}

// This is a valid match
fn later_unguarded(t: T, c: bool) -> i32 {
    match t {
        T::A() if c => 1,
        T::A() => 2,
        T::B() => 3,
    }
}

fn guarded_wildcard(t: T, c: bool) -> i32 {
    match t {
        // { dg-error "non-exhaustive patterns: 'T::B..' not covered" "" { target *-*-* } .-1 }
        T::A() => 1,
        _ if c => 2,
    }
}

enum Foo {
    X(),
    Y(),
}

enum U {
    A(Foo),
    B(),
}

fn nested(u: U, c: bool) {
    match u {
        // { dg-error "non-exhaustive patterns: 'U::A.Foo::X...' not covered" "" { target *-*-* } .-1 }
        U::A(Foo::X()) if c => {}
        U::A(Foo::Y()) => {}
        U::B() => {}
    }
}

fn main() {}
