// Copyright 2021 Hwakyeom Kim(=just-do-halee)

use super::*;

pub trait CursorTrait<'s, T: 's>
where
    Self: Iterator<Item = &'s T>,
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
    fn left_shift(&mut self, n: usize) -> Option<&'s T>;
    fn right_shift(&mut self, n: usize) -> Option<&'s T>;
}
