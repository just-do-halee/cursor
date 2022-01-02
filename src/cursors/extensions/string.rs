// Copyright 2021 Hwakyeom Kim(=just-do-halee)

use super::*;

#[derive(PartialEq, Eq, Clone)]
pub struct StrCursor<'s, E: Extras<char> = NoneExtras<char>> {
    cursor: Cursor<'s, u8, NoneExtras<u8>>,
    len: Option<usize>,
    info: StrCursorInfo<E>,
    saved_info: StrCursorInfo<E>,
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
        self.info.extras.clone()
    }
}

/// this will reset the newer cursor
impl<E: Extras<char>> ToCursor<u8> for StrCursor<'_, E> {}

impl<E: Extras<char>> AsRef<[u8]> for StrCursor<'_, E> {
    #[inline]
    fn as_ref(&self) -> &[u8] {
        self.as_bytes()
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

#[inline]
fn str_cursor_new<EXTRAS: Extras<char>>(string: &str) -> StrCursor<EXTRAS> {
    StrCursor {
        cursor: Cursor::new(string.as_bytes()),
        len: None,
        info: StrCursorInfo::new(),
        saved_info: StrCursorInfo::new(),
    }
}

impl<'s> StrCursor<'s, NoneExtras<char>> {
    #[inline]
    pub fn new(string: &'s str) -> Self {
        str_cursor_new(string)
    }
    #[inline]
    pub fn new_with_extras<EXTRAS: Extras<char>>(string: &str) -> StrCursor<EXTRAS> {
        str_cursor_new(string)
    }
}

impl<'s, E: Extras<char>> StrCursor<'s, E> {
    // ------ private ------
    #[inline]
    fn set_current(&mut self, val: char) {
        self.info.current = val;
    }
    /// * WARNING: directly sets backwards. no effects.
    #[inline]
    pub fn unsafe_set_backwards(&mut self, new_backwards: bool) {
        *self.cursor.backwards_mut() = new_backwards;
    }
    #[inline]
    pub fn set_backwards(&mut self, new_backwards: bool) {
        if self.backwards() != new_backwards {
            self.load_char_start_pos();
        }
        *self.cursor.backwards_mut() = new_backwards;
    }
    #[inline]
    fn set_char_start_pos(&mut self, val: usize) {
        self.info.char_start_pos = val;
    }
    /// load code point
    #[inline]
    fn load_char_start_pos(&mut self) {
        if self.pos_as_bytes() != self.char_start_pos() {
            self.cursor.unsafe_set_pos(self.char_start_pos());
        }
    }
    /// purely sets position + blushes extras. returns current().
    #[inline]
    fn set_pos(&mut self, new_pos: usize) -> Option<char> {
        if new_pos != self.pos() {
            self.info.pos = new_pos;
            self.blush_extras();
        }

        Some(self.current())
    }
    /// * WARNING: directly sets byte position. no effects.
    #[inline]
    fn unsafe_set_pos_as_bytes(&mut self, new_pos: usize) {
        self.cursor.unsafe_set_pos(new_pos);
    }
    #[inline]
    fn blush_extras(&mut self) {
        if !self.noeffects() {
            self.info.extras.change(&self.current(), self.pos());
        }
    }

    #[inline]
    fn jump_to_added(&mut self, rhs: usize) -> Option<char> {
        self.jump_to_offset(rhs as isize)
    }
    #[inline]
    fn jump_to_subed(&mut self, rhs: usize) -> Option<char> {
        self.jump_to_offset(-(rhs as isize))
    }

    // ------ public ------
    #[inline]
    pub fn unwrapped_next(&mut self) -> char {
        self.next().unwrap()
    }
}

impl<'s, E: Extras<char>> Iterator for StrCursor<'s, E> {
    type Item = char;
    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        // 1 byte next and then scanning char
        // save the first code point pos
        self.cursor.next()?;
        let byte_pos = self.pos_as_bytes();
        let (ch, pos) = match self.backwards() {
            false => {
                let ch = utf::next_char(&mut self.cursor)?;
                (ch, self.pos().checked_add(1)?)
            }
            true => {
                let ch = utf::next_back_char(&mut self.cursor)?;
                (ch, self.pos().checked_sub(1)?)
            }
        };
        self.set_char_start_pos(byte_pos);
        if self.is_init() {
            self.set_current(ch);
            self.set_pos(pos)
        } else {
            self.set_current(ch);
            self.blush_extras();
            Some(self.current())
        }
    }
}

impl<'s, E: Extras<char>> StrCursorTrait<'s, E> for StrCursor<'s, E> {
    #[inline]
    fn is_init(&self) -> bool {
        self.current() != EOF_CHAR
    }
    /// if `next` or `jump` can effect the [`Extras`](Extras).
    #[inline]
    fn noeffects(&self) -> bool {
        self.info.noeffects
    }
    #[inline]
    fn noeffects_mut(&mut self) -> &mut bool {
        &mut self.info.noeffects
    }
    #[inline]
    fn backwards(&self) -> bool {
        self.cursor.backwards()
    }
    #[inline]
    fn turnaround(&mut self) {
        self.load_char_start_pos();
        self.cursor.turnaround();
    }
    #[inline]
    fn pos(&self) -> usize {
        self.info.pos
    }
    #[inline]
    fn pos_as_bytes(&self) -> usize {
        self.cursor.pos()
    }
    #[inline]
    fn char_start_pos(&self) -> usize {
        self.info.char_start_pos
    }
    /// if you've never tried it before,
    /// this method will create remainder iterator and then
    /// consume it to count number of chars.
    #[inline]
    fn len(&mut self) -> usize {
        if let Some(n) = self.len {
            n
        } else {
            let offset = self
                .as_right_side_bytes()
                .iter()
                .filter(|&&byte| !utf::utf8_is_cont_byte(byte))
                .count();
            self.len = Some(self.pos() + offset + 1);
            self.len()
        }
    }
    #[inline]
    fn is_len(&self) -> bool {
        self.len.is_some()
    }
    #[inline]
    fn len_as_bytes(&self) -> usize {
        self.cursor.len()
    }
    #[inline]
    fn is_empty(&self) -> bool {
        self.as_bytes().len() == 0
    }
    #[inline]
    fn as_bytes(&self) -> &'s [u8] {
        self.cursor.as_slice()
    }
    #[inline]
    fn extras(&self) -> &E {
        &self.info.extras
    }
    #[inline]
    fn current(&self) -> char {
        self.info.current
    }
    #[inline]
    fn reset(&mut self) {
        self.info.reset();
        self.cursor.reset();
    }
    #[inline]
    fn save(&mut self) {
        // sets inner info at the first-time
        self.info.inner = self.cursor.info.clone();
        self.saved_info = self.info.clone();
    }
    #[inline]
    fn saved(&self) -> &StrCursorInfo<E> {
        &self.saved_info
    }
    #[inline]
    fn load(&mut self) {
        self.info = self.saved_info.clone();
    }
    #[inline]
    fn jump_to_last(&mut self) -> char {
        let last_pos = self.len().saturating_sub(1);
        self.jump(last_pos).unwrap()
    }
    /// - if you had tried `len` before,
    /// - this method does more performance.
    /// * *[inline function]*
    #[inline]
    fn jump(&mut self, pos: usize) -> Option<char> {
        let ch = match pos {
            _ if self.is_init() && pos == self.pos() => return Some(self.current()),
            0 => {
                self.unsafe_set_pos_as_bytes(0);
                let ch = utf::next_char(&mut self.cursor)?;
                if self.backwards() {
                    self.set_char_start_pos(self.pos_as_bytes());
                    self.unsafe_set_pos_as_bytes(0);
                } else {
                    self.set_char_start_pos(0);
                }
                ch
            }
            _ if matches!(self.len, Some(len) if len.saturating_sub(1) == pos) => {
                let byte_last_pos = self.len_as_bytes().saturating_sub(1);
                self.unsafe_set_pos_as_bytes(byte_last_pos);
                let ch = utf::next_back_char(&mut self.cursor)?;
                if self.backwards() {
                    self.set_char_start_pos(byte_last_pos);
                } else {
                    self.set_char_start_pos(self.pos_as_bytes());
                    self.unsafe_set_pos_as_bytes(byte_last_pos);
                }
                ch
            }
            _ => {
                // =-=-=-=-=-=-=-=-=-=-=-=-=-=
                let (dist, is_dist) = self.pos().detailed_diff(pos);
                let new_byte_pos = match is_dist {
                    Ordering::Greater => {
                        let (offset, _) = self
                            .cursor
                            .as_right_side_slice()
                            .iter()
                            .enumerate()
                            .filter(|(_, &byte)| !utf::utf8_is_cont_byte(byte))
                            .take(dist)
                            .last()?;
                        self.cursor.pos() + offset + 1
                    }
                    Ordering::Equal => return Some(self.current()),
                    Ordering::Less => {
                        let (offset, _) = self
                            .cursor
                            .as_left_side_slice_include_current()
                            .iter()
                            .rev()
                            .enumerate()
                            .filter(|(_, &byte)| !utf::utf8_is_cont_byte(byte))
                            .take(dist + 1)
                            .last()?;
                        self.cursor.pos() - offset
                    }
                };
                self.unsafe_set_pos_as_bytes(new_byte_pos);

                // =-=-=-=-=-=-=-=-=-=-=-=-=-=
                if !self.cursor.is_init() {
                    self.cursor.set_init(true);
                }
                let ch;
                if self.backwards() {
                    ch = utf::next_char(&mut self.cursor)?;
                    self.set_char_start_pos(self.cursor.pos());
                    self.unsafe_set_pos_as_bytes(new_byte_pos);
                } else {
                    self.set_char_start_pos(new_byte_pos);
                    ch = utf::next_char(&mut self.cursor)?;
                }
                if self.pos_as_bytes() == self.len_as_bytes().saturating_sub(1) {
                    self.len = Some(pos); // sets length
                }
                ch
            }
        };
        self.set_current(ch);
        self.set_pos(pos)
    }
}

// ------- WARNING: isize -------

impl<'s, E: Extras<char>> AddAssign<usize> for StrCursor<'s, E> {
    #[inline]
    fn add_assign(&mut self, rhs: usize) {
        self.jump_to_added(rhs);
    }
}

impl<'s, E: Extras<char>> Add<usize> for StrCursor<'s, E> {
    type Output = Option<char>;
    #[inline]
    fn add(mut self, rhs: usize) -> Self::Output {
        self.jump_to_added(rhs)
    }
}

impl<'s, E: Extras<char>> Add<usize> for &mut StrCursor<'s, E> {
    type Output = Option<char>;
    #[inline]
    fn add(self, rhs: usize) -> Self::Output {
        self.jump_to_added(rhs)
    }
}

impl<'s, E: Extras<char>> SubAssign<usize> for StrCursor<'s, E> {
    #[inline]
    fn sub_assign(&mut self, rhs: usize) {
        self.jump_to_subed(rhs);
    }
}

impl<'s, E: Extras<char>> Sub<usize> for StrCursor<'s, E> {
    type Output = Option<char>;
    #[inline]
    fn sub(mut self, rhs: usize) -> Self::Output {
        self.jump_to_subed(rhs)
    }
}

impl<'s, E: Extras<char>> Sub<usize> for &mut StrCursor<'s, E> {
    type Output = Option<char>;
    #[inline]
    fn sub(self, rhs: usize) -> Self::Output {
        self.jump_to_subed(rhs)
    }
}
