// Copyright 2021 Hwakyeom Kim(=just-do-halee)

use super::*;

pub use super::str::from_utf8_unchecked;

const CONT_MASK: u8 = 0b0011_1111;

#[inline]
pub fn next_code_point<'a, C: CursorTrait<'a, u8>>(cursor: &mut C) -> (Option<u32>, usize) {
    let mut times = 0;

    // Decode UTF-8
    let x = if let Some(c) = cursor.next() {
        times += 1;
        *c
    } else {
        return (None, times);
    };

    if x < 128 {
        return (Some(x as u32), times);
    }

    // Multibyte case follows
    // Decode from a byte combination out of: [[[x y] z] w]
    // NOTE: Performance is sensitive to the exact formulation here
    let init = utf8_first_byte(x, 2);
    let y = unwrap_or_0(cursor.next());
    times += 1;

    let mut ch = utf8_acc_cont_byte(init, y);
    if x >= 0xE0 {
        // [[x y z] w] case
        // 5th bit in 0xE0 .. 0xEF is always clear, so `init` is still valid
        let z = unwrap_or_0(cursor.next());
        times += 1;

        let y_z = utf8_acc_cont_byte((y & CONT_MASK) as u32, z);
        ch = init << 12 | y_z;
        if x >= 0xF0 {
            // [x y z w] case
            // use only the lower 3 bits of `init`
            let w = unwrap_or_0(cursor.next());
            times += 1;

            ch = (init & 7) << 18 | utf8_acc_cont_byte(y_z, w);
        }
    }

    (Some(ch), times)
}

#[inline]
fn utf8_first_byte(byte: u8, width: u32) -> u32 {
    (byte & (0x7F >> width)) as u32
}
/// Returns the value of `ch` updated with continuation byte `byte`.
#[inline]
fn utf8_acc_cont_byte(ch: u32, byte: u8) -> u32 {
    (ch << 6) | (byte & CONT_MASK) as u32
}

#[inline]
fn unwrap_or_0(opt: Option<&u8>) -> u8 {
    match opt {
        Some(&byte) => byte,
        None => 0,
    }
}

/// Reads the last code point out of a byte iterator (assuming a
/// UTF-8-like encoding).
#[inline]
pub fn next_code_point_reverse<'a, C: CursorTrait<'a, u8>>(cursor: &mut C) -> (Option<u32>, usize) {
    let mut times = 0;

    let x = if let Some(c) = cursor.next() {
        times += 1;
        *c
    } else {
        return (None, times);
    };

    // Decode UTF-8
    let w = match x {
        next_byte if next_byte < 128 => return (Some(next_byte as u32), times),
        back_byte => back_byte,
    };

    // Multibyte case follows
    // Decode from a byte combination out of: [x [y [z w]]]
    let mut ch;
    let z = unwrap_or_0(cursor.next());
    times += 1;

    ch = utf8_first_byte(z, 2);
    if utf8_is_cont_byte(z) {
        let y = unwrap_or_0(cursor.next());
        times += 1;

        ch = utf8_first_byte(y, 3);
        if utf8_is_cont_byte(y) {
            let x = unwrap_or_0(cursor.next());
            times += 1;

            ch = utf8_first_byte(x, 4);
            ch = utf8_acc_cont_byte(ch, y);
        }
        ch = utf8_acc_cont_byte(ch, z);
    }
    ch = utf8_acc_cont_byte(ch, w);

    (Some(ch), times)
}

/// Checks whether the byte is a UTF-8 continuation byte (i.e., starts with the
/// bits `10`).
#[inline]
fn utf8_is_cont_byte(byte: u8) -> bool {
    (byte as i8) < -64
}
