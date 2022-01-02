// Copyright 2021 Hwakyeom Kim(=just-do-halee)

use super::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct NoneExtras<T>(PhantomData<T>);
impl<T> Extras<T> for NoneExtras<T> {
    #[inline]
    fn new() -> Self {
        Self(PhantomData)
    }
    #[inline]
    fn clone(&self) -> Self {
        NoneExtras::new()
    }
    #[inline]
    fn change(&mut self, _: &T, _: usize) {}
    #[inline]
    fn reset(&mut self) {}
}
