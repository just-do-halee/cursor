// Copyright 2021 Hwakyeom Kim(=just-do-halee)

use super::*;

pub trait Extras<Input> {
    fn new() -> Self;
    fn clone(&self) -> Self;
    fn change(&mut self, input: &Input, pos: usize);
    fn reset(&mut self);
}

pub trait ToExtras<E: Extras<Self::Input>> {
    type Input;
    fn to_extras(&self) -> E;
    #[inline]
    fn into_extras(self) -> E
    where
        Self: Sized,
    {
        self.to_extras()
    }
}

pub trait ToCursor<T, E: Extras<T> = NoneExtras<T>>
where
    Self: AsRef<[T]>,
{
    #[inline]
    fn to_cursor(&self) -> Cursor<T, E> {
        Cursor::new_with_extras::<E>(self.as_ref())
    }
}

pub trait CursorTrait<'s, T: 's, E = NoneExtras<T>>
where
    Self: Iterator<Item = &'s T>, // with .next()
    E: Extras<T>,
{
    #[inline]
    fn item_size(&self) -> usize {
        mem::size_of::<T>()
    }
    #[inline]
    fn range(&self) -> Range<usize> {
        0..self.len()
    }
    #[inline]
    fn len(&self) -> usize {
        self.as_slice().len()
    }
    #[inline]
    fn is_empty(&self) -> bool {
        self.as_slice().is_empty()
    }
    fn is_init(&self) -> bool;

    fn noeffects(&self) -> bool;
    fn noeffects_mut(&mut self) -> &mut bool;
    #[inline]
    fn noeffects_on(&mut self) {
        *self.noeffects_mut() = true;
    }
    #[inline]
    fn noeffects_off(&mut self) {
        *self.noeffects_mut() = false;
    }

    fn backwards(&self) -> bool;
    fn backwards_mut(&mut self) -> &mut bool;

    #[inline]
    fn turnaround(&mut self) {
        *self.backwards_mut() = !self.backwards();
    }
    #[inline]
    fn head_to_left(&mut self) {
        if !self.backwards() {
            self.turnaround();
        }
    }
    #[inline]
    fn head_to_right(&mut self) {
        if self.backwards() {
            self.turnaround();
        }
    }
    #[inline]
    fn head_to_pos(&mut self, pos: usize) {
        match self.pos().cmp(&pos) {
            Ordering::Greater => self.head_to_left(),
            Ordering::Equal => {}
            Ordering::Less => self.head_to_right(),
        }
    }

    fn pos(&self) -> usize;
    fn extras(&self) -> &E;

    /// cloning `saved().extras` to `self.extras()`.
    #[inline]
    fn to_range_extras(&self) -> Range<E> {
        self.saved().extras.clone()..self.extras().clone()
    }

    fn reset(&mut self);
    fn save(&mut self);
    fn saved(&self) -> &CursorInfo<T, E>;
    fn load(&mut self);

    fn as_slice(&self) -> &'s [T];

    /// saved pos to current pos.
    #[inline]
    fn as_slice_loaded(&self) -> &'s [T] {
        let slice = self.as_slice();
        let pos = self.pos();
        let saved_pos = self.saved().pos;
        match pos.cmp(&saved_pos) {
            Ordering::Greater => &slice[saved_pos..pos.saturating_add(1)],
            Ordering::Equal => &slice[pos..1],
            Ordering::Less => &slice[pos..saved_pos.saturating_add(1)],
        }
    }

    #[inline]
    fn as_preserved_slice(&self) -> &'s [T] {
        if self.backwards() {
            self.as_right_side_slice()
        } else {
            self.as_left_side_slice()
        }
    }

    #[inline]
    fn as_preserved_slice_include_current(&self) -> &'s [T] {
        if self.backwards() {
            self.as_right_side_slice_include_current()
        } else {
            self.as_left_side_slice_include_current()
        }
    }

    #[inline]
    fn current(&self) -> &'s T {
        &self.as_slice()[self.pos()]
    }
    #[inline]
    fn current_deref(&self) -> T
    where
        T: Copy,
    {
        self.as_slice()[self.pos()]
    }

    #[inline]
    fn as_remaining_slice(&self) -> &'s [T] {
        if self.backwards() {
            self.as_left_side_slice()
        } else {
            self.as_right_side_slice()
        }
    }

    #[inline]
    fn as_remaining_slice_include_current(&self) -> &'s [T] {
        if self.backwards() {
            self.as_left_side_slice_include_current()
        } else {
            self.as_right_side_slice_include_current()
        }
    }

    #[inline]
    fn as_left_side_slice(&self) -> &'s [T] {
        &self.as_slice()[..self.pos()]
    }

    #[inline]
    fn as_right_side_slice(&self) -> &'s [T] {
        &self.as_slice()[self.pos().saturating_add(1)..]
    }

    #[inline]
    fn as_left_side_slice_include_current(&self) -> &'s [T] {
        &self.as_slice()[..self.pos().saturating_add(1)]
    }

    #[inline]
    fn as_right_side_slice_include_current(&self) -> &'s [T] {
        &self.as_slice()[self.pos()..]
    }

    // ------------ JUMP ------------

    fn jump(&mut self, pos: usize) -> Option<&'s T>;

    /// if abs == 1, initial coordinate is -1 or just 0
    #[inline]
    fn jump_to_offset(&mut self, offset: isize) -> Option<&'s T> {
        if offset == 0 {
            return Some(self.current());
        }
        let (abs, sig) = isize_to_usize(offset);
        match (abs, sig) {
            (1, 1) => {
                if self.backwards() {
                    self.turnaround();
                    let ch = self.next();
                    self.turnaround();
                    ch
                } else {
                    self.next()
                }
            }
            (_, 1) => self.jump(self.pos().checked_add(abs)?),
            (1, -1) => {
                if self.backwards() {
                    self.next()
                } else {
                    self.turnaround();
                    let ch = self.next();
                    self.turnaround();
                    ch
                }
            }
            (_, -1) => self.jump(self.pos().checked_sub(abs)?),
            _ => unreachable!(),
        }
    }
    #[inline]
    fn jump_to_first(&mut self) -> &'s T {
        self.jump(0).unwrap()
    }
    #[inline]
    fn jump_to_last(&mut self) -> &'s T {
        self.jump(self.len().saturating_sub(1)).unwrap()
    }
    /// same with
    /// ```ignore
    /// {
    ///     self.jump_to_first();
    ///     self.next_to_last();
    /// }
    /// ```
    #[inline]
    fn first_to_last(&mut self) {
        self.jump_to_first();
        self.next_to_last();
    }
    /// jump to the saved pos.
    #[inline]
    fn jump_to_load(&mut self) -> &'s T {
        self.jump(self.saved().pos).unwrap()
    }
    /// WARNING: isize
    #[inline]
    fn jump_cycle(&mut self, pos: isize) -> &'s T {
        let len = self.len() as isize;
        if pos < 0 {
            self.jump((len.saturating_sub(pos)) as usize).unwrap()
        } else if len <= pos {
            self.jump((pos.saturating_sub(len).saturating_sub(1)) as usize)
                .unwrap()
        } else {
            self.jump(pos as usize).unwrap()
        }
    }
    /// WARNING: isize
    #[inline]
    fn jump_to_offset_cycle(&mut self, offset: isize) -> &'s T {
        let dst = (self.pos() as isize) + offset;
        let len = self.len() as isize;
        if dst == 0 {
            self.current()
        } else if dst < 0 {
            self.jump((len.saturating_sub(dst)) as usize).unwrap()
        } else if len <= dst {
            self.jump((dst.saturating_sub(len).saturating_sub(1)) as usize)
                .unwrap()
        } else {
            self.jump_to_offset(offset).unwrap()
        }
    }

    // ------------ NEXT ------------

    #[inline]
    fn next_to_pos(&mut self, pos: usize) -> Option<&'s T> {
        if pos >= self.len() {
            return None;
        }

        let (diff, is_pos) = self.pos().detailed_diff(pos);
        match is_pos {
            Ordering::Greater => self.head_to_right(),
            Ordering::Equal => {}
            Ordering::Less => self.head_to_left(),
        }

        for _ in 1..diff {
            self.next()?;
        }
        self.next()
    }

    #[inline]
    fn next_to_offset(&mut self, offset: isize) -> Option<&'s T> {
        if self.pos().checked_move_offset(offset)? >= self.len() {
            return None;
        }

        let (abs, sig) = isize_to_usize(offset);
        match sig {
            1 => self.head_to_right(),
            0 => {}
            -1 => self.head_to_left(),
            _ => unreachable!(),
        }

        for _ in 1..abs {
            self.next()?;
        }
        self.next()
    }
    #[inline]
    fn next_to_first(&mut self) -> &'s T {
        self.head_to_left();
        while self.next().is_some() {}
        self.current()
    }
    #[inline]
    fn next_to_last(&mut self) -> &'s T {
        self.head_to_right();
        while self.next().is_some() {}
        self.current()
    }
    #[inline]
    fn next_to_left(&mut self) -> Option<&'s T> {
        self.head_to_left();
        self.next()
    }
    #[inline]
    fn next_to_right(&mut self) -> Option<&'s T> {
        self.head_to_right();
        self.next()
    }
    /// bump until meets f() = `true`.
    #[inline]
    fn next_to_until(&mut self, f: fn(&T) -> bool) -> &'s T {
        #[allow(clippy::while_let_on_iterator)]
        while let Some(item) = self.next() {
            if f(item) {
                break;
            }
        }
        self.current()
    }
    /// bump while f() = `true`.
    #[inline]
    fn next_to_while(&mut self, f: fn(&T) -> bool) -> &'s T {
        #[allow(clippy::while_let_on_iterator)]
        while let Some(item) = self.next() {
            if !f(item) {
                break;
            }
        }
        self.current()
    }
    /// bump until meets saved pos.
    #[inline]
    fn next_to_load(&mut self) -> &'s T {
        self.next_to_pos(self.saved().pos).unwrap()
    }
    /// bump until meets saved pos.
    #[inline]
    fn next_cycle(&mut self) -> &'s T {
        if let Some(v) = self.next() {
            v
        } else {
            self.turnaround();
            if self.backwards() {
                self.jump_to_last()
            } else {
                self.jump_to_first()
            }
        }
    }
    /// bump until meets saved pos.
    #[inline]
    fn next_to_offset_cycle(&mut self, offset: isize) -> &'s T {
        let (abs, sig) = isize_to_usize(offset);
        match sig {
            1 => self.head_to_right(),
            0 => {}
            -1 => self.head_to_left(),
            _ => unreachable!(),
        }

        for _ in 1..abs {
            self.next_cycle();
        }
        self.next_cycle()
    }
}

// ----------------

pub trait StrCursorTrait<'s, E = NoneExtras<char>>
where
    Self: Iterator<Item = char>,
    E: Extras<char>,
{
    #[inline]
    fn item_size(&self) -> usize {
        mem::size_of::<char>()
    }
    /// if you've never tried `len()` before,
    /// this method will excute `len()` method.
    #[inline]
    fn range(&mut self) -> Range<usize> {
        0..self.len()
    }

    fn len(&mut self) -> usize;
    fn is_len(&self) -> bool;
    #[inline]
    fn len_as_bytes(&self) -> usize {
        self.as_bytes().len()
    }

    #[inline]
    fn is_empty(&self) -> bool {
        self.as_bytes().is_empty()
    }

    fn is_init(&self) -> bool;

    fn noeffects(&self) -> bool;
    fn noeffects_mut(&mut self) -> &mut bool;
    #[inline]
    fn noeffects_on(&mut self) {
        *self.noeffects_mut() = true;
    }
    #[inline]
    fn noeffects_off(&mut self) {
        *self.noeffects_mut() = false;
    }

    fn backwards(&self) -> bool;

    fn turnaround(&mut self);

    #[inline]
    fn head_to_left(&mut self) {
        if !self.backwards() {
            self.turnaround();
        }
    }
    #[inline]
    fn head_to_right(&mut self) {
        if self.backwards() {
            self.turnaround();
        }
    }
    #[inline]
    fn head_to_pos(&mut self, pos: usize) {
        match self.pos().cmp(&pos) {
            Ordering::Greater => self.head_to_left(),
            Ordering::Equal => {}
            Ordering::Less => self.head_to_right(),
        }
    }

    fn pos(&self) -> usize;
    fn pos_as_bytes(&self) -> usize;
    fn char_start_pos(&self) -> usize;

    fn extras(&self) -> &E;

    /// cloning `saved().extras` to `self.extras()`.
    #[inline]
    fn to_range_extras(&self) -> Range<E> {
        self.saved().extras.clone()..self.extras().clone()
    }

    fn reset(&mut self);
    fn save(&mut self);
    fn saved(&self) -> &StrCursorInfo<E>;
    fn load(&mut self);

    // ------ BYTES ------

    fn as_bytes(&self) -> &'s [u8];

    /// saved pos to current pos.
    #[inline]
    fn as_bytes_loaded(&self) -> &'s [u8] {
        let slice = self.as_bytes();
        let pos = self.pos_as_bytes();
        let saved_pos = self.saved().inner.pos;
        match pos.cmp(&saved_pos) {
            Ordering::Greater => &slice[saved_pos..pos.saturating_add(1)],
            Ordering::Equal => &slice[pos..1],
            Ordering::Less => &slice[pos..saved_pos.saturating_add(1)],
        }
    }

    /// saved pos to current char start pos.
    #[inline]
    fn as_char_bytes_loaded(&self) -> &'s [u8] {
        let slice = self.as_bytes();
        let pos = self.char_start_pos();
        let saved_pos = self.saved().inner.pos;
        match pos.cmp(&saved_pos) {
            Ordering::Greater => &slice[saved_pos..pos.saturating_add(1)],
            Ordering::Equal => &slice[pos..1],
            Ordering::Less => &slice[pos..saved_pos.saturating_add(1)],
        }
    }

    #[inline]
    fn as_preserved_bytes(&self) -> &'s [u8] {
        if self.backwards() {
            self.as_right_side_bytes()
        } else {
            self.as_left_side_bytes()
        }
    }

    #[inline]
    fn as_preserved_bytes_include_current(&self) -> &'s [u8] {
        if self.backwards() {
            self.as_right_side_bytes_include_current()
        } else {
            self.as_left_side_bytes_include_current()
        }
    }

    #[inline]
    fn current_byte(&self) -> u8 {
        self.as_bytes()[self.pos_as_bytes()]
    }

    #[inline]
    fn as_remaining_bytes(&self) -> &'s [u8] {
        if self.backwards() {
            self.as_left_side_bytes()
        } else {
            self.as_right_side_bytes()
        }
    }

    #[inline]
    fn as_remaining_bytes_include_current(&self) -> &'s [u8] {
        if self.backwards() {
            self.as_left_side_bytes_include_current()
        } else {
            self.as_right_side_bytes_include_current()
        }
    }

    #[inline]
    fn as_left_side_bytes(&self) -> &'s [u8] {
        &self.as_bytes()[..self.pos_as_bytes()]
    }

    #[inline]
    fn as_right_side_bytes(&self) -> &'s [u8] {
        &self.as_bytes()[self.pos_as_bytes().saturating_add(1)..]
    }

    #[inline]
    fn as_left_side_bytes_include_current(&self) -> &'s [u8] {
        &self.as_bytes()[..self.pos_as_bytes().saturating_add(1)]
    }

    #[inline]
    fn as_right_side_bytes_include_current(&self) -> &'s [u8] {
        &self.as_bytes()[self.pos_as_bytes()..]
    }

    // ------ STR ------

    #[inline]
    fn as_str(&self) -> &'s str {
        utf::from_utf8_unchecked(self.as_bytes())
    }

    /// saved pos to current pos.
    #[inline]
    fn as_str_loaded(&self) -> &'s str {
        let saved_backwards = self.saved().inner.backwards;
        let curr_backwards = self.backwards();
        let saved_pos = self.saved().inner.pos;
        let curr_pos = self.pos_as_bytes();
        let saved_char_start_pos = self.saved().char_start_pos;
        let curr_char_start_pos = self.char_start_pos();
        match curr_pos.cmp(&saved_pos) {
            Ordering::Greater => {
                let saved_pos = if saved_backwards {
                    saved_pos
                } else {
                    saved_char_start_pos
                };
                let curr_pos = if curr_backwards {
                    curr_char_start_pos
                } else {
                    curr_pos
                };
                utf::from_utf8_unchecked(&self.as_bytes()[saved_pos..curr_pos + 1])
            }
            Ordering::Equal => utf::from_utf8_unchecked(if curr_backwards {
                &self.as_bytes()[curr_pos..curr_char_start_pos + 1]
            } else {
                &self.as_bytes()[curr_char_start_pos..curr_pos + 1]
            }),
            Ordering::Less => {
                let saved_pos = if saved_backwards {
                    saved_char_start_pos
                } else {
                    saved_pos
                };
                let curr_pos = if curr_backwards {
                    curr_pos
                } else {
                    curr_char_start_pos
                };
                utf::from_utf8_unchecked(&self.as_bytes()[curr_pos..saved_pos + 1])
            }
        }
    }

    #[inline]
    fn as_preserved_str(&self) -> &'s str {
        if self.backwards() {
            self.as_right_side_str()
        } else {
            self.as_left_side_str()
        }
    }

    fn current(&self) -> char;

    #[inline]
    fn as_remaining_str(&self) -> &'s str {
        if self.backwards() {
            self.as_left_side_str()
        } else {
            self.as_right_side_str()
        }
    }

    #[inline]
    fn as_left_side_str(&self) -> &'s str {
        let curr_pos = self.pos_as_bytes();
        if let Some(n) = (&self.as_bytes()[..curr_pos.saturating_add(1)])
            .iter()
            .rev()
            .position(|&byte| !utf::utf8_is_cont_byte(byte))
        {
            utf::from_utf8_unchecked(&self.as_bytes()[..curr_pos.saturating_sub(n)])
        } else {
            ""
        }
    }

    #[inline]
    fn as_right_side_str(&self) -> &'s str {
        let curr_pos = self.pos_as_bytes();
        if let Some(n) = (&self.as_bytes()[self.pos_as_bytes().saturating_add(1)..])
            .iter()
            .position(|&byte| !utf::utf8_is_cont_byte(byte))
        {
            utf::from_utf8_unchecked(&self.as_bytes()[curr_pos.saturating_add(n + 1)..])
        } else {
            ""
        }
    }

    // ------------ JUMP ------------

    fn jump(&mut self, pos: usize) -> Option<char>;

    /// if abs == 1, initial coordinate is -1 or just 0
    #[inline]
    fn jump_to_offset(&mut self, offset: isize) -> Option<char> {
        if offset == 0 {
            return Some(self.current());
        }
        let (abs, sig) = isize_to_usize(offset);
        match (abs, sig) {
            (1, 1) => {
                if self.backwards() {
                    self.turnaround();
                    let ch = self.next();
                    self.turnaround();
                    ch
                } else {
                    self.next()
                }
            }
            (_, 1) => self.jump(self.pos().checked_add(abs)?),
            (1, -1) => {
                if self.backwards() {
                    self.next()
                } else {
                    self.turnaround();
                    let ch = self.next();
                    self.turnaround();
                    ch
                }
            }
            (_, -1) => self.jump(self.pos().checked_sub(abs)?),
            _ => unreachable!(),
        }
    }
    #[inline]
    fn jump_to_first(&mut self) -> char {
        self.jump(0).unwrap()
    }
    fn jump_to_last(&mut self) -> char;
    /// same with
    /// ```ignore
    /// {
    ///     self.jump_to_first();
    ///     self.next_to_last();
    /// }
    /// ```
    #[inline]
    fn first_to_last(&mut self) {
        self.jump_to_first();
        self.next_to_last();
    }
    /// jump to the saved pos.
    #[inline]
    fn jump_to_load(&mut self) -> char {
        self.jump(self.saved().pos).unwrap()
    }
    /// - **WARNING: isize**
    /// - if you've never tried it before,
    /// - this method will create Chars Iterator and then
    /// - consume it to count number of chars.
    #[inline]
    fn jump_cycle(&mut self, pos: isize) -> char {
        let len = self.len() as isize;
        if pos < 0 {
            self.jump((len.saturating_sub(pos)) as usize).unwrap()
        } else if len <= pos {
            self.jump((pos.saturating_sub(len).saturating_sub(1)) as usize)
                .unwrap()
        } else {
            self.jump(pos as usize).unwrap()
        }
    }
    /// - **WARNING: isize**
    /// - if you've never tried it before,
    /// - this method will create Chars Iterator and then
    /// - consume it to count number of chars.
    #[inline]
    fn jump_to_offset_cycle(&mut self, offset: isize) -> char {
        let dst = (self.pos() as isize) + offset;
        let len = self.len() as isize;
        if dst == 0 {
            self.current()
        } else if dst < 0 {
            self.jump((len.saturating_sub(dst)) as usize).unwrap()
        } else if len <= dst {
            self.jump((dst.saturating_sub(len).saturating_sub(1)) as usize)
                .unwrap()
        } else {
            self.jump_to_offset(offset).unwrap()
        }
    }

    // ------------ NEXT ------------

    #[inline]
    fn next_to_pos(&mut self, pos: usize) -> Option<char> {
        if self.is_len() && pos >= self.len() {
            return None;
        }

        let (diff, is_pos) = self.pos().detailed_diff(pos);
        match is_pos {
            Ordering::Greater => self.head_to_right(),
            Ordering::Equal => {}
            Ordering::Less => self.head_to_left(),
        }

        for _ in 1..diff {
            self.next()?;
        }
        self.next()
    }

    #[inline]
    fn next_to_offset(&mut self, offset: isize) -> Option<char> {
        if self.is_len() && self.pos().checked_move_offset(offset)? >= self.len() {
            return None;
        }

        let (abs, sig) = isize_to_usize(offset);
        match sig {
            1 => self.head_to_right(),
            0 => {}
            -1 => self.head_to_left(),
            _ => unreachable!(),
        }

        for _ in 1..abs {
            self.next()?;
        }
        self.next()
    }
    #[inline]
    fn next_to_first(&mut self) -> char {
        self.head_to_left();
        while self.next().is_some() {}
        self.current()
    }
    #[inline]
    fn next_to_last(&mut self) -> char {
        self.head_to_right();
        while self.next().is_some() {}
        self.current()
    }
    #[inline]
    fn next_to_left(&mut self) -> Option<char> {
        self.head_to_left();
        self.next()
    }
    #[inline]
    fn next_to_right(&mut self) -> Option<char> {
        self.head_to_right();
        self.next()
    }
    /// bump until meets f() = `true`.
    #[inline]
    fn next_to_until(&mut self, f: fn(char) -> bool) -> char {
        #[allow(clippy::while_let_on_iterator)]
        while let Some(ch) = self.next() {
            if f(ch) {
                break;
            }
        }
        self.current()
    }
    /// bump while f() = `true`.
    #[inline]
    fn next_to_while(&mut self, f: fn(char) -> bool) -> char {
        #[allow(clippy::while_let_on_iterator)]
        while let Some(ch) = self.next() {
            if !f(ch) {
                break;
            }
        }
        self.current()
    }
    /// bump until meets saved pos.
    #[inline]
    fn next_to_load(&mut self) -> char {
        self.next_to_pos(self.saved().pos).unwrap()
    }
    /// bump until meets saved pos.
    #[inline]
    fn next_cycle(&mut self) -> char {
        if let Some(v) = self.next() {
            v
        } else {
            self.turnaround();
            if self.backwards() {
                self.jump_to_last()
            } else {
                self.jump_to_first()
            }
        }
    }
    /// bump until meets saved pos.
    #[inline]
    fn next_to_offset_cycle(&mut self, offset: isize) -> char {
        let (abs, sig) = isize_to_usize(offset);
        match sig {
            1 => self.head_to_right(),
            0 => {}
            -1 => self.head_to_left(),
            _ => unreachable!(),
        }

        for _ in 1..abs {
            self.next_cycle();
        }
        self.next_cycle()
    }
}
