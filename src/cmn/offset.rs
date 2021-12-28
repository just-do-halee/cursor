// Copyright 2021 Hwakyeom Kim(=just-do-halee)

use super::*;

pub trait MoveOffset: Sized {
    fn checked_move_offset(self, offset: isize) -> Option<Self>;
    fn detailed_diff(self, other: Self) -> (Self, Ordering);
    fn plain_diff(self, other: Self) -> Self;
}

impl MoveOffset for usize {
    #[inline]
    fn plain_diff(self, other: Self) -> Self {
        match self.cmp(&other) {
            Ordering::Greater => self - other,
            Ordering::Equal => self,
            Ordering::Less => other - self,
        }
    }
    /// returns (`distance`, `is_pos`)
    #[inline]
    fn detailed_diff(self, other: Self) -> (Self, Ordering) {
        match self.cmp(&other) {
            Ordering::Greater => (self - other, Ordering::Less),
            Ordering::Equal => (self, Ordering::Equal),
            Ordering::Less => (other - self, Ordering::Greater),
        }
    }
    #[inline]
    fn checked_move_offset(self, offset: isize) -> Option<Self> {
        let (abs, sig) = isize_to_usize(offset);
        match sig {
            1 => self.checked_add(abs),
            0 => Some(self),
            -1 => self.checked_sub(abs),
            _ => unreachable!(),
        }
    }
}
