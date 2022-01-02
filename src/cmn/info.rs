// Copyright 2021 Hwakyeom Kim(=just-do-halee)

use super::*;

#[derive(PartialEq, Eq)]
pub struct CursorInfo<T, E: Extras<T> = NoneExtras<T>> {
    pub init: bool,
    pub backwards: bool,
    pub pos: usize,
    pub extras: E,
    pub noeffects: bool,
    _marker: PhantomData<T>,
}
impl<T, E: Extras<T>> Default for CursorInfo<T, E> {
    #[inline]
    fn default() -> Self {
        CursorInfo {
            init: false,
            backwards: false,
            pos: 0,
            extras: Extras::new(),
            noeffects: false,
            _marker: PhantomData,
        }
    }
}
impl<T, E: Extras<T>> Clone for CursorInfo<T, E> {
    #[inline]
    fn clone(&self) -> Self {
        CursorInfo {
            init: self.init,
            backwards: self.backwards,
            pos: self.pos,
            extras: self.extras.clone(),
            noeffects: false,
            _marker: PhantomData,
        }
    }
}

impl<T, E: Extras<T>> CursorInfo<T, E> {
    #[inline]
    pub fn new() -> Self {
        CursorInfo::default()
    }
    #[inline]
    pub fn reset(&mut self) {
        self.init = false;
        self.backwards = false;
        self.pos = 0;
        self.extras.reset();
        self.noeffects = false;
    }
}

// ------ extensions ------

#[derive(PartialEq, Eq)]
pub struct StrCursorInfo<E: Extras<char> = NoneExtras<char>> {
    pub inner: CursorInfo<u8, NoneExtras<u8>>,
    pub pos: usize,
    pub extras: E,
    pub char_start_pos: usize,
    pub current: char,
    pub noeffects: bool,
}
impl<E: Extras<char>> Default for StrCursorInfo<E> {
    #[inline]
    fn default() -> Self {
        StrCursorInfo {
            inner: CursorInfo::default(),
            pos: 0,
            extras: Extras::new(),
            char_start_pos: 0,
            current: EOF_CHAR,
            noeffects: false,
        }
    }
}
impl<E: Extras<char>> Clone for StrCursorInfo<E> {
    #[inline]
    fn clone(&self) -> Self {
        StrCursorInfo {
            inner: self.inner.clone(),
            pos: self.pos,
            extras: self.extras.clone(),
            char_start_pos: self.char_start_pos,
            current: self.current,
            noeffects: self.noeffects,
        }
    }
}

impl<E: Extras<char>> StrCursorInfo<E> {
    #[inline]
    pub fn new() -> Self {
        StrCursorInfo::default()
    }
    #[inline]
    pub fn reset(&mut self) {
        self.inner.reset();
        self.pos = 0;
        self.extras.reset();
        self.char_start_pos = 0;
        self.current = EOF_CHAR;
        self.noeffects = false;
    }
}
