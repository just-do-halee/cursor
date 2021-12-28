// Copyright 2021 Hwakyeom Kim(=just-do-halee)

#![allow(non_snake_case)]

/// returns (`abs`, `signum`)
#[inline]
pub fn isize_to_usize(i: isize) -> (usize, i8) {
    (i.abs() as usize, i.signum() as i8)
}

#[inline]
pub fn wrap<T, F: FnOnce() -> bool>(i: T, condition: F) -> Option<T> {
    if condition() {
        Some(i)
    } else {
        None
    }
}
