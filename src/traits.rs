// Copyright 2021 Hwakyeom Kim(=just-do-halee)

use super::*;

pub trait Extras<Input> {
    fn new() -> Self;
    fn clone(&self) -> Self;
    fn change(&mut self, input: &Input);
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
    Self: Iterator<Item = &'s T>,
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

    fn reset(&mut self);

    fn pos(&self) -> usize;
    fn backwards(&self) -> bool;

    fn saved_pos(&self) -> usize;
    fn save(&mut self);
    fn load_slice(&self) -> &'s [T];

    fn as_slice(&self) -> &'s [T];
    fn as_remaining_slice(&self) -> &'s [T];
    fn as_preserved_slice(&self) -> &'s [T];

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

    fn turnaround(&mut self);
    fn shift_first(&mut self);
    fn shift_last(&mut self);
    #[inline]
    fn first_to_last(&mut self) {
        self.shift_first();
        self.next_to_last();
    }
    #[inline]
    fn next_to_last(&mut self) {
        while self.right_shift(1).is_some() {}
    }
    fn left_shift(&mut self, n: usize) -> Option<&'s T>;
    fn right_shift(&mut self, n: usize) -> Option<&'s T>;
}
