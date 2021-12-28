// Copyright 2021 Hwakyeom Kim(=just-do-halee)

#[cfg(feature = "std")]
pub use std::{
    cmp::Ordering,
    fmt,
    marker::PhantomData,
    mem,
    ops::{Add, AddAssign, Range, Sub, SubAssign},
    str,
};

#[cfg(not(feature = "std"))]
pub use core::{
    cmp::Ordering,
    fmt,
    marker::PhantomData,
    mem,
    ops::{Add, AddAssign, Range, Sub, SubAssign},
    str,
};

pub const EOF_CHAR: char = '\0';
