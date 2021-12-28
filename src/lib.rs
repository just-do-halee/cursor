// Copyright 2021 Hwakyeom Kim(=just-do-halee)
//! # **`cursor`**
//!
//! A more free Rust-Iterator.<br>
//!
//! <a href="https://github.com/just-do-halee/cursor/tree/main/examples">Examples</a>

#![cfg_attr(all(not(feature = "std"), not(test)), no_std)]

mod cmn;
pub use cmn::*;

mod traits;
pub use traits::*;

mod cursors;
pub use cursors::*;
