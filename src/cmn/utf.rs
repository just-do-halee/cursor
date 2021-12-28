// Copyright 2021 Hwakyeom Kim(=just-do-halee)

use super::*;

type Utf8MaxSize = u32;

#[inline]
fn unchecked_utf8_to_char(u: Utf8MaxSize) -> char {
    unsafe { char::from_u32_unchecked(u) }
}

#[inline]
pub fn from_utf8_unchecked(bytes: &[u8]) -> &str {
    unsafe { str::from_utf8_unchecked(bytes) }
}

#[inline]
pub fn next_char<'a, C: CursorTrait<'a, u8>>(cursor: &mut C) -> Option<char> {
    // decode UTF-8
    let x = *cursor.current();

    // byte1: width = 1
    // x < 128 (1000 0000) => x <= 0111 1111
    if x < 128 {
        return Some(unchecked_utf8_to_char(x as Utf8MaxSize));
    }

    let backwards = cursor.backwards();
    if backwards {
        cursor.turnaround();
    }

    // byte2: width = 2
    let mut x_body = utf8_first_byte_body(x, 2);
    // 110(0 0000)

    let y = unwrap_or_0(cursor.next());
    let y_body = utf8_cont_byte_body(y);

    // concat bits
    let mut ch: Utf8MaxSize = x_body << 6 | y_body;

    // byte3: width = 3
    x_body = utf8_first_byte_body(x, 3);
    // x >= 1110 (0000)
    if x >= 0xE0 {
        let z = unwrap_or_0(cursor.next());

        let z_body = utf8_cont_byte_body(z);

        // concat bits
        ch = x_body << 12 | y_body << 6 | z_body;

        // byte4: width = 4
        x_body = utf8_first_byte_body(x, 4);
        // x >= 1111 0(000)
        if x >= 0xF0 {
            let w = unwrap_or_0(cursor.next());

            let w_body = utf8_cont_byte_body(w);

            // concat bits
            ch = x_body << 18 | y_body << 12 | z_body << 6 | w_body;
        }
    }

    if backwards {
        cursor.turnaround();
    }
    Some(unchecked_utf8_to_char(ch))
}

#[inline]
fn utf8_first_byte_body(byte: u8, width: u8) -> Utf8MaxSize {
    utf8_filter_body_side(byte, width + 1) as Utf8MaxSize
}

#[inline]
fn utf8_cont_byte_body(byte: u8) -> Utf8MaxSize {
    utf8_filter_body_side(byte, 2) as Utf8MaxSize
}

/// WARNING: head_length must be less than 5, manually.
#[inline]
fn utf8_filter_body_side(byte: u8, head_length: u8) -> u8 {
    byte & (u8::MAX >> head_length)
}

#[inline]
fn unwrap_or_0(opt: Option<&u8>) -> u8 {
    match opt {
        Some(&byte) => byte,
        None => 0,
    }
}

/// reads the last code point out of a byte iterator (assuming a
/// UTF-8-like encoding).
#[inline]
pub fn next_back_char<'a, C: CursorTrait<'a, u8>>(cursor: &mut C) -> Option<char> {
    let x = *cursor.current();

    // decode UTF-8
    let w = match x {
        // byte1: width = 1
        // x < 128 (1000 0000) => x <= 0111 1111
        next_byte if next_byte < 128 => {
            return Some(unchecked_utf8_to_char(next_byte as Utf8MaxSize));
        }
        // or back
        back_byte => back_byte,
    };

    let backwards = cursor.backwards();
    if !backwards {
        cursor.turnaround();
    }

    // decode from a byte combination out of: [x [y [z w]]]
    let mut ch;
    let z = unwrap_or_0(cursor.next());
    ch = utf8_first_byte_body(z, 2);

    if utf8_is_cont_byte(z) {
        let y = unwrap_or_0(cursor.next());

        ch = utf8_first_byte_body(y, 3);
        if utf8_is_cont_byte(y) {
            let x = unwrap_or_0(cursor.next());

            ch = utf8_first_byte_body(x, 4);
            ch = utf8_acc_cont_byte(ch, y);
        }
        ch = utf8_acc_cont_byte(ch, z);
    }
    ch = utf8_acc_cont_byte(ch, w);

    if !backwards {
        cursor.turnaround();
    }

    Some(unchecked_utf8_to_char(ch))
}

/// returns the value of `ch` updated with continuation byte `byte`.
#[inline]
fn utf8_acc_cont_byte(ch: Utf8MaxSize, byte: u8) -> Utf8MaxSize {
    (ch << 6) | (byte & 0b0011_1111) as Utf8MaxSize
}

/// checks whether the byte is a UTF-8 continuation byte (i.e., starts with the
/// bits `10`).
#[inline]
pub fn utf8_is_cont_byte(byte: u8) -> bool {
    (byte as i8) < -64
}
