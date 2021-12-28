// Copyright 2021 Hwakyeom Kim(=just-do-halee)

use super::*;

mod extensions;
pub use extensions::string::*;

// ---------------------------

#[derive(PartialEq, Eq, Clone)]
pub struct Cursor<'s, T: 's, E: Extras<T> = NoneExtras<T>> {
    slice: &'s [T],
    len: usize,
    info: CursorInfo<T, E>,
    saved_info: CursorInfo<T, E>,
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
        self.info.extras.clone()
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
impl<'s, T, E: Extras<T>> AsRef<Self> for Cursor<'s, T, E> {
    #[inline]
    fn as_ref(&self) -> &Self {
        self
    }
}
impl<T, E: Extras<T>> AsMut<Self> for Cursor<'_, T, E> {
    #[inline]
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

#[inline]
fn cursor_new<T, EXTRAS: Extras<T>>(slice: &[T]) -> Cursor<T, EXTRAS> {
    Cursor {
        slice,
        len: slice.len(),
        info: CursorInfo::new(),
        saved_info: CursorInfo::new(),
    }
}
impl<'s, T: 's> Cursor<'s, T, NoneExtras<T>> {
    #[inline]
    pub fn new(slice: &'s [T]) -> Self {
        cursor_new(slice)
    }
    #[inline]
    pub fn new_with_extras<EXTRAS: Extras<T>>(slice: &[T]) -> Cursor<T, EXTRAS> {
        cursor_new(slice)
    }
}

impl<'s, T: 's, E: Extras<T>> Cursor<'s, T, E> {
    // ------ private ------
    #[inline]
    fn set_init(&mut self, val: bool) {
        self.info.init = val;
    }
    /// * WARNING: directly sets position. no effects.
    #[inline]
    pub fn unsafe_set_pos(&mut self, new_pos: usize) {
        self.info.pos = new_pos;
    }
    #[inline]
    fn set_pos(&mut self, new_pos: usize) -> Option<&'s T> {
        if new_pos == self.pos() {
            return Some(self.current());
        } else if new_pos >= self.len() {
            return None;
        }

        if !self.is_init() {
            self.set_init(true);
        }

        self.info.pos = new_pos;

        self.blush_extras();
        Some(self.current())
    }
    #[inline]
    fn blush_extras(&mut self) {
        self.info.extras.change(self.current());
    }

    #[inline]
    fn jump_to_added(&mut self, rhs: usize) -> Option<&'s T> {
        self.jump_to_offset(rhs as isize)
    }
    #[inline]
    fn jump_to_subed(&mut self, rhs: usize) -> Option<&'s T> {
        self.jump_to_offset(-(rhs as isize))
    }

    // ------ public ------
    #[inline]
    pub fn unwrapped_next(&mut self) -> T
    where
        T: Copy,
    {
        *self.next().unwrap()
    }
}

impl<'s, T, E: Extras<T>> Iterator for Cursor<'s, T, E> {
    type Item = &'s T;
    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        match self.backwards() {
            _ if !self.is_init() => {
                self.set_init(true);
                Some(self.current())
            }
            false => self.set_pos(self.pos().checked_add(1)?),
            true => self.set_pos(self.pos().checked_sub(1)?),
        }
    }
}

impl<'s, T: 's, E: Extras<T>> CursorTrait<'s, T, E> for Cursor<'s, T, E> {
    #[inline]
    fn is_init(&self) -> bool {
        self.info.init
    }
    #[inline]
    fn backwards(&self) -> bool {
        self.info.backwards
    }
    #[inline]
    fn backwards_mut(&mut self) -> &mut bool {
        &mut self.info.backwards
    }
    #[inline]
    fn turnaround(&mut self) {
        self.info.backwards = !self.info.backwards;
    }
    #[inline]
    fn pos(&self) -> usize {
        self.info.pos
    }
    #[inline]
    fn len(&self) -> usize {
        self.len
    }
    #[inline]
    fn is_empty(&self) -> bool {
        self.len == 0
    }
    #[inline]
    fn as_slice(&self) -> &'s [T] {
        self.slice
    }
    #[inline]
    fn extras(&self) -> &E {
        &self.info.extras
    }

    /// excepts saved_info.
    #[inline]
    fn reset(&mut self) {
        self.info.reset();
    }
    #[inline]
    fn save(&mut self) {
        self.saved_info = self.info.clone();
    }
    #[inline]
    fn saved(&self) -> &CursorInfo<T, E> {
        &self.saved_info
    }
    #[inline]
    fn load(&mut self) {
        self.info = self.saved_info.clone();
    }

    #[inline]
    fn jump(&mut self, pos: usize) -> Option<&'s T> {
        self.set_pos(pos)
    }
}

// ------- WARNING: isize -------

impl<'s, T, E: Extras<T>> AddAssign<usize> for Cursor<'s, T, E> {
    #[inline]
    fn add_assign(&mut self, rhs: usize) {
        self.jump_to_added(rhs);
    }
}

impl<'s, T, E: Extras<T>> Add<usize> for Cursor<'s, T, E> {
    type Output = Option<&'s T>;
    #[inline]
    fn add(mut self, rhs: usize) -> Self::Output {
        self.jump_to_added(rhs)
    }
}

impl<'s, T, E: Extras<T>> Add<usize> for &mut Cursor<'s, T, E> {
    type Output = Option<&'s T>;
    #[inline]
    fn add(self, rhs: usize) -> Self::Output {
        self.jump_to_added(rhs)
    }
}

impl<'s, T, E: Extras<T>> SubAssign<usize> for Cursor<'s, T, E> {
    #[inline]
    fn sub_assign(&mut self, rhs: usize) {
        self.jump_to_subed(rhs);
    }
}

impl<'s, T, E: Extras<T>> Sub<usize> for Cursor<'s, T, E> {
    type Output = Option<&'s T>;
    #[inline]
    fn sub(mut self, rhs: usize) -> Self::Output {
        self.jump_to_subed(rhs)
    }
}

impl<'s, T, E: Extras<T>> Sub<usize> for &mut Cursor<'s, T, E> {
    type Output = Option<&'s T>;
    #[inline]
    fn sub(self, rhs: usize) -> Self::Output {
        self.jump_to_subed(rhs)
    }
}
