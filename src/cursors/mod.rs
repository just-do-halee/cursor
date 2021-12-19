// Copyright 2021 Hwakyeom Kim(=just-do-halee)

use super::*;

mod string;
pub use string::*;

#[derive(PartialEq, Eq, Clone)]
pub struct Cursor<'s, T: 's, E: Extras<T> = NoneExtras<T>> {
    slice: &'s [T],
    len: usize,
    pos: usize,
    init: bool,
    backwards: bool,
    saved_pos: usize,
    pub extras: E,
}

impl<T: fmt::Debug, E: Extras<T>> fmt::Debug for Cursor<'_, T, E> {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("Cursor")
            .field(&self.as_preserved_slice())
            .field(&self.current())
            .field(&self.as_remaining_slice())
            .finish()
    }
}

impl<T, E: Extras<T>> ToExtras<E> for Cursor<'_, T, E> {
    type Input = T;
    #[inline]
    fn to_extras(&self) -> E {
        self.extras.clone()
    }
}

/// this would reset the newer cursor
impl<T, E: Extras<T>> ToCursor<T, E> for Cursor<'_, T, E> {}

impl<T, E: Extras<T>> AsRef<[T]> for Cursor<'_, T, E> {
    #[inline]
    fn as_ref(&self) -> &[T] {
        self.as_slice()
    }
}

impl<'s, T: 's> Cursor<'s, T, NoneExtras<T>> {
    #[inline]
    pub fn new(slice: &'s [T]) -> Self {
        Cursor {
            slice,
            len: slice.len(),
            pos: 0,
            init: false,
            backwards: false,
            saved_pos: 0,
            extras: Extras::new(),
        }
    }
}

impl<T> Cursor<'_, T, NoneExtras<T>> {
    #[inline]
    pub fn new_with_extras<EXTRAS: Extras<T>>(slice: &[T]) -> Cursor<T, EXTRAS> {
        Cursor {
            slice,
            len: slice.len(),
            pos: 0,
            init: false,
            backwards: false,
            saved_pos: 0,
            extras: Extras::new(),
        }
    }
}

impl<'s, T: 's, E: Extras<T>> Cursor<'s, T, E> {
    #[inline]
    fn pos_checked_add(&self, n: usize) -> Option<usize> {
        let pos = self.pos.checked_add(n)?;
        if pos < self.len {
            Some(pos)
        } else {
            None
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
            self.extras.change(self.current());
        }
    }
    #[inline]
    pub fn unwrapped_next(&mut self) -> T
    where
        T: Copy,
    {
        *self.next().unwrap()
    }
}

impl<'s, T: 's, E: Extras<T>> CursorTrait<'s, T> for Cursor<'s, T, E> {
    #[inline]
    fn len(&self) -> usize {
        self.len
    }
    #[inline]
    fn pos(&self) -> usize {
        self.pos
    }
    #[inline]
    fn saved_pos(&self) -> usize {
        self.saved_pos
    }
    #[inline]
    fn current(&self) -> &'s T {
        &self.slice[self.pos]
    }
    #[inline]
    fn backwards(&self) -> bool {
        self.backwards
    }
    #[inline]
    fn reset(&mut self) {
        self.pos = 0;
        self.init = false;
        self.backwards = false;
        self.saved_pos = 0;
        self.extras.reset();
    }
    #[inline]
    fn save(&mut self) {
        self.saved_pos = self.pos;
    }
    #[inline]
    fn load_slice(&self) -> &'s [T] {
        if self.saved_pos < self.pos {
            &self.slice[self.saved_pos..self.pos.saturating_add(1)]
        } else {
            &self.slice[self.pos..self.saved_pos.saturating_add(1)]
        }
    }
    #[inline]
    fn as_slice(&self) -> &'s [T] {
        self.slice
    }
    #[inline]
    fn as_remaining_slice(&self) -> &'s [T] {
        if self.backwards {
            &self.slice[..self.pos]
        } else {
            &self.slice[self.pos.saturating_add(1)..]
        }
    }
    #[inline]
    fn as_preserved_slice(&self) -> &'s [T] {
        if self.backwards {
            &self.slice[self.pos.saturating_add(1)..]
        } else {
            &self.slice[..self.pos]
        }
    }
    #[inline]
    fn turnaround(&mut self) {
        self.backwards = !self.backwards;
    }
    #[inline]
    fn shift_first(&mut self) {
        self.set_pos(0);
        self.set_to_left();
    }
    #[inline]
    fn shift_last(&mut self) {
        self.set_pos(self.len - 1);
        self.set_to_right();
    }
    #[inline]
    fn left_shift(&mut self, n: usize) -> Option<&'s T> {
        let mut n = n;
        if !self.init {
            self.init = true;
            if n == 1 {
                n = 0;
                self.extras.change(self.current());
            }
        }
        self.set_pos(self.pos.checked_sub(n)?);
        self.set_to_left();
        Some(self.current())
    }
    #[inline]
    fn right_shift(&mut self, n: usize) -> Option<&'s T> {
        let mut n = n;
        if !self.init {
            self.init = true;
            if n == 1 {
                n = 0;
                self.extras.change(self.current());
            }
        }
        self.set_pos(self.pos_checked_add(n)?);
        self.set_to_right();
        Some(self.current())
    }
}

impl<'s, T, E: Extras<T>> AddAssign<usize> for Cursor<'s, T, E> {
    #[inline]
    fn add_assign(&mut self, rhs: usize) {
        self.right_shift(rhs);
    }
}

impl<'s, T, E: Extras<T>> Add<usize> for Cursor<'s, T, E> {
    type Output = Option<&'s T>;
    #[inline]
    fn add(mut self, rhs: usize) -> Self::Output {
        self.right_shift(rhs)
    }
}

impl<'s, T, E: Extras<T>> Add<usize> for &mut Cursor<'s, T, E> {
    type Output = Option<&'s T>;
    #[inline]
    fn add(self, rhs: usize) -> Self::Output {
        (*self).right_shift(rhs)
    }
}

impl<'s, T, E: Extras<T>> SubAssign<usize> for Cursor<'s, T, E> {
    #[inline]
    fn sub_assign(&mut self, rhs: usize) {
        self.left_shift(rhs);
    }
}

impl<'s, T, E: Extras<T>> Sub<usize> for Cursor<'s, T, E> {
    type Output = Option<&'s T>;
    #[inline]
    fn sub(mut self, rhs: usize) -> Self::Output {
        self.left_shift(rhs)
    }
}

impl<'s, T, E: Extras<T>> Sub<usize> for &mut Cursor<'s, T, E> {
    type Output = Option<&'s T>;
    #[inline]
    fn sub(self, rhs: usize) -> Self::Output {
        (*self).left_shift(rhs)
    }
}

impl<'s, T, E: Extras<T>> AsRef<Self> for Cursor<'s, T, E> {
    #[inline]
    fn as_ref(&self) -> &Self {
        self
    }
}

impl<'s, T, E: Extras<T>> Iterator for Cursor<'s, T, E> {
    type Item = &'s T;
    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        match self.backwards {
            false => self.right_shift(1),
            true => self.left_shift(1),
        }
    }
}
