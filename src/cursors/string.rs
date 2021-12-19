// Copyright 2021 Hwakyeom Kim(=just-do-halee)

use super::{utf::*, *};

#[derive(PartialEq, Eq, Clone)]
pub struct StrCursor<'s, E: Extras<char> = NoneExtras<char>> {
    cursor: Cursor<'s, u8, NoneExtras<u8>>,
    current: char,
    pos: usize,
    times: usize,
    init: bool,
    saved_pos: usize,
    extras: E,
}

impl<E: Extras<char>> fmt::Debug for StrCursor<'_, E> {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("StrCursor")
            .field(&self.as_preserved_str())
            .field(&self.current())
            .field(&self.as_remaining_str())
            .finish()
    }
}

impl<E: Extras<char>> ToExtras<E> for StrCursor<'_, E> {
    type Input = char;
    #[inline]
    fn to_extras(&self) -> E {
        self.extras.clone()
    }
}

/// this will reset the newer cursor
impl<E: Extras<char>> ToCursor<u8> for StrCursor<'_, E> {}

impl<E: Extras<char>> AsRef<[u8]> for StrCursor<'_, E> {
    #[inline]
    fn as_ref(&self) -> &[u8] {
        self.as_slice()
    }
}

const EOF: char = '\0';

impl<'s> StrCursor<'s, NoneExtras<char>> {
    #[inline]
    pub fn new(string: &'s str) -> Self {
        StrCursor {
            cursor: Cursor::new(string.as_bytes()),
            current: EOF,
            pos: 0,
            times: 0,
            init: true,
            saved_pos: 0,
            extras: Extras::new(),
        }
    }
}

impl StrCursor<'_, NoneExtras<char>> {
    #[inline]
    pub fn new_with_extras<EXTRAS: Extras<char>>(string: &str) -> StrCursor<EXTRAS> {
        StrCursor {
            cursor: Cursor::new(string.as_bytes()),
            current: EOF,
            pos: 0,
            times: 0,
            init: true,
            saved_pos: 0,
            extras: Extras::new(),
        }
    }
}

impl<'s, E: Extras<char>> StrCursor<'s, E> {
    #[inline]
    pub fn reset(&mut self) {
        self.cursor.reset();
        self.current = EOF;
        self.pos = 0;
        self.times = 0;
        self.init = true;
        self.saved_pos = 0;
        self.extras.reset();
    }
    #[inline]
    pub fn into_extras(self) -> E {
        self.extras
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
    fn set_pos(&mut self, pos: usize) {
        let prev_pos = self.pos;
        self.pos = pos;
        if prev_pos != pos {
            self.extras.change(&self.current());
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
                _ if !cursor_init => self.extras.change(&self.current()),
                true => self.set_pos(self.pos - 1),
                false => self.set_pos(self.pos + 1),
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
        self.set_pos(0);
        self.cursor.shift_first();
    }
    /// This method will call next repeatedly until [None] is encountered
    #[inline]
    pub fn shift_last(&mut self) {
        self.set_to_right();
        while self.bump().is_some() {}
    }
    #[inline]
    pub fn first_to_last(&mut self) {
        self.set_pos(0);
        self.cursor.shift_first();
        self.shift_last()
    }
    #[inline]
    pub fn next_to_last(&mut self) {
        self.shift_last()
    }
    #[inline]
    pub fn unwrapped_next(&mut self) -> char {
        self.next().unwrap()
    }
}

impl<'s, E: Extras<char>> AsRef<Self> for StrCursor<'s, E> {
    #[inline]
    fn as_ref(&self) -> &Self {
        self
    }
}

impl<'s, E: Extras<char>> AsMut<Self> for StrCursor<'s, E> {
    #[inline]
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl<'s, E: Extras<char>> Iterator for StrCursor<'s, E> {
    type Item = char;
    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.bump()
    }
}

impl<'s, E: Extras<char>> AddAssign<usize> for StrCursor<'s, E> {
    #[inline]
    fn add_assign(&mut self, rhs: usize) {
        self.right_shift(rhs);
    }
}

impl<'s, E: Extras<char>> Add<usize> for StrCursor<'s, E> {
    type Output = Option<char>;
    #[inline]
    fn add(mut self, rhs: usize) -> Self::Output {
        self.right_shift(rhs)
    }
}

impl<'s, E: Extras<char>> Add<usize> for &mut StrCursor<'s, E> {
    type Output = Option<char>;
    #[inline]
    fn add(self, rhs: usize) -> Self::Output {
        (*self).right_shift(rhs)
    }
}

impl<'s, E: Extras<char>> SubAssign<usize> for StrCursor<'s, E> {
    #[inline]
    fn sub_assign(&mut self, rhs: usize) {
        self.left_shift(rhs);
    }
}

impl<'s, E: Extras<char>> Sub<usize> for StrCursor<'s, E> {
    type Output = Option<char>;
    #[inline]
    fn sub(mut self, rhs: usize) -> Self::Output {
        self.left_shift(rhs)
    }
}

impl<'s, E: Extras<char>> Sub<usize> for &mut StrCursor<'s, E> {
    type Output = Option<char>;
    #[inline]
    fn sub(self, rhs: usize) -> Self::Output {
        (*self).left_shift(rhs)
    }
}
