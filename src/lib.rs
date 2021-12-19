// Copyright 2021 Hwakyeom Kim(=just-do-halee)
//! # **`cursor`**
//!
//! A more free Rust-Iterator.<br>
//!
//! <a href="https://github.com/just-do-halee/cursor/tree/main/examples">Examples</a>

use std::{
    fmt,
    marker::PhantomData,
    mem,
    ops::{Add, AddAssign, Range, Sub, SubAssign},
    str,
};

mod extras;
mod traits;
mod utf;

pub use extras::*;
pub use traits::*;

mod cursors;
pub use cursors::*;
