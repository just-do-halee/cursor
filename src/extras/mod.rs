// Copyright 2021 Hwakyeom Kim(=just-do-halee)

use super::{utf::*, *};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct StrCursor<'s> {
    cursor: Cursor<'s, u8>,
    current: char,
    pos: usize,
    times: usize,
    init: bool,
    saved_pos: usize,
}

impl<'s> StrCursor<'s> {
    pub const EOF: char = '\0';

    #[inline]
    pub fn new(string: &'s str) -> Self {
        StrCursor {
            cursor: Cursor::new(string.as_bytes()),
            current: Self::EOF,
            pos: 0,
            times: 0,
            init: true,
            saved_pos: 0,
        }
    }
    #[inline]
    pub fn raw_range(&self) -> Range<usize> {
        0..self.raw_len()
    }
    #[inline]
    pub fn raw_len(&self) -> usize {
        self.cursor.len()
    }
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.cursor.is_empty()
    }
    #[inline]
    pub fn pos(&self) -> usize {
        self.pos
    }
    #[inline]
    pub fn saved_pos(&self) -> usize {
        self.saved_pos
    }
    #[inline]
    pub fn backwards(&mut self) -> bool {
        self.cursor.backwards
    }
    #[inline]
    pub fn current(&self) -> char {
        self.current
    }
    #[inline]
    pub fn turnaround(&mut self) {
        self.init = false;
        self.cursor.turnaround();
    }
    #[inline]
    pub fn save(&mut self) {
        self.saved_pos = self.pos;
        self.cursor.save();
    }
    #[inline]
    pub fn load_str(&self) -> &'s str {
        let Cursor {
            slice,
            saved_pos,
            pos,
            ..
        } = self.cursor;

        let out = if saved_pos < pos {
            &slice[saved_pos..pos.saturating_add(1)]
        } else {
            &slice[pos..saved_pos.saturating_add(1)]
        };

        if out.is_empty() {
            return "";
        }
        unsafe { from_utf8_unchecked(out) }
    }
    #[inline]
    fn as_slice(&self) -> &'s [u8] {
        self.cursor.as_slice()
    }
    #[inline]
    fn as_remaining_slice(&self) -> &'s [u8] {
        self.cursor.as_remaining_slice()
    }
    #[inline]
    fn as_preserved_slice(&self) -> &'s [u8] {
        self.cursor.as_preserved_slice()
    }
    #[inline]
    pub fn as_str(&self) -> &'s str {
        unsafe { from_utf8_unchecked(self.as_slice()) }
    }
    #[inline]
    pub fn as_remaining_str(&self) -> &'s str {
        let slice = self.as_remaining_slice();
        unsafe { from_utf8_unchecked(slice) }
    }
    #[inline]
    pub fn as_preserved_str(&self) -> &'s str {
        let slice = self.as_preserved_slice();
        if slice.is_empty() {
            return "";
        }
        unsafe {
            from_utf8_unchecked(&slice[..slice.len().saturating_sub(self.times.saturating_sub(1))])
        }
    }
    #[inline]
    fn set_to_left(&mut self) {
        if !self.backwards() {
            self.turnaround();
        }
    }
    #[inline]
    fn set_to_right(&mut self) {
        if self.backwards() {
            self.turnaround();
        }
    }
    #[inline]
    fn bump(&mut self) -> Option<char> {
        let cursor_init = self.cursor.init;
        if !self.init {
            for _ in 0..self.times.saturating_sub(1) {
                self.cursor.next();
            }
            self.init = true;
        }
        let (ch, times) = if self.backwards() {
            next_code_point_reverse(&mut self.cursor)
        } else {
            next_code_point(&mut self.cursor)
        };

        let out = ch.map(|ch| {
            // SAFETY: `str` invariant says `ch` is a valid Unicode Scalar Value.
            unsafe { char::from_u32_unchecked(ch) }
        });
        if let Some(v) = out {
            self.current = v;
            match self.backwards() {
                _ if !cursor_init => {}
                true => self.pos -= 1,
                false => self.pos += 1,
            }
        }
        self.times = times;
        out
    }
    #[inline]
    fn shift(&mut self, n: usize) -> Option<char> {
        for _ in 0..n {
            self.bump()?;
        }
        Some(self.current)
    }
    #[inline]
    pub fn left_shift(&mut self, n: usize) -> Option<char> {
        let mut n = n;
        if !self.cursor.init && n == 1 {
            n = 0;
        }
        self.set_to_left();
        self.shift(n)
    }
    #[inline]
    pub fn right_shift(&mut self, n: usize) -> Option<char> {
        let mut n = n;
        if !self.cursor.init && n == 1 {
            n = 0;
        }
        self.set_to_right();
        self.shift(n + if self.cursor.init { 0 } else { 1 })
    }
    #[inline]
    pub fn shift_first(&mut self) {
        self.set_to_left();
        self.pos = 0;
        self.cursor.shift_first();
    }
    /// This method will call next repeatedly until [None] is encountered
    #[inline]
    pub fn shift_last(&mut self) {
        self.set_to_right();
        while self.bump().is_some() {}
    }
    #[inline]
    pub fn unwrapped_next(&mut self) -> char {
        self.next().unwrap()
    }
}

impl<'s> AsRef<Self> for StrCursor<'s> {
    #[inline]
    fn as_ref(&self) -> &Self {
        self
    }
}

impl<'s> AsMut<Self> for StrCursor<'s> {
    #[inline]
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl<'s> Iterator for StrCursor<'s> {
    type Item = char;
    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.bump()
    }
}

impl<'s> AddAssign<usize> for StrCursor<'s> {
    fn add_assign(&mut self, rhs: usize) {
        self.right_shift(rhs);
    }
}

impl<'s> Add<usize> for StrCursor<'s> {
    type Output = Option<char>;
    fn add(mut self, rhs: usize) -> Self::Output {
        self.right_shift(rhs)
    }
}

impl<'s> Add<usize> for &mut StrCursor<'s> {
    type Output = Option<char>;
    fn add(self, rhs: usize) -> Self::Output {
        (*self).right_shift(rhs)
    }
}

impl<'s> SubAssign<usize> for StrCursor<'s> {
    fn sub_assign(&mut self, rhs: usize) {
        self.left_shift(rhs);
    }
}

impl<'s> Sub<usize> for StrCursor<'s> {
    type Output = Option<char>;
    fn sub(mut self, rhs: usize) -> Self::Output {
        self.left_shift(rhs)
    }
}

impl<'s> Sub<usize> for &mut StrCursor<'s> {
    type Output = Option<char>;
    fn sub(self, rhs: usize) -> Self::Output {
        (*self).left_shift(rhs)
    }
}
