// Character literals outside ASCII have their Unicode scalar value.
#![feature(no_core)]
#![no_core]

const E_ACUTE: char = 'é';

fn kind(c: char) -> i32 {
    match c {
        'é' => 1,
        'ÿ' => 2,
        'Σ' => 3,
        '\u{10FFFF}' => 4,
        _ => 0,
    }
}

fn main() -> i32 {
    if 'é' as u32 != 0xE9 {
        return 1;
    }
    if E_ACUTE as u32 != 0xE9 {
        return 2;
    }
    if 'Σ' as u32 != 0x3A3 {
        return 3;
    }
    if '\u{10FFFF}' as u32 != 0x10FFFF {
        return 4;
    }
    if 'é' as u8 != 0xE9 {
        return 5;
    }
    // 'é' and 'ÿ' share the UTF-8 lead byte 0xC3.
    if 'é' == 'ÿ' {
        return 6;
    }
    if kind('é') != 1 || kind('ÿ') != 2 || kind('Σ') != 3
        || kind('\u{10FFFF}') != 4 || kind('e') != 0 {
        return 7;
    }

    0
}
