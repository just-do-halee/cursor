#![allow(clippy::while_let_on_iterator)]

use cursor::*;

const SLICE: &[u8] = &[1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
const STRING: &str = "this is test. 안녕하세요. 이것은 #&*@( 테스트입니다. ^^ thanks.";

#[derive(Debug)]
struct Counter(pub usize);

impl Counter {
    pub fn _new() -> Self {
        Counter(0)
    }
    fn _clone(&self) -> Self {
        Counter(self.0)
    }
    fn _reset(&mut self) {
        self.0 = 0;
    }
}

impl Extras<u8> for Counter {
    fn new() -> Self {
        Counter::_new()
    }
    fn clone(&self) -> Self {
        self._clone()
    }
    fn reset(&mut self) {
        self._reset()
    }
    fn change(&mut self, input: &u8, _pos: usize) {
        if input % 2 == 0 {
            self.0 += 1;
        }
    }
}

impl Extras<char> for Counter {
    fn new() -> Self {
        Counter::_new()
    }
    fn clone(&self) -> Self {
        self._clone()
    }
    fn reset(&mut self) {
        self._reset()
    }
    fn change(&mut self, input: &char, _pos: usize) {
        if *input == ' ' {
            self.0 += 1;
        }
    }
}

fn main() {
    example1();
    example2();
    example3();
    example4();
    // TODO: more examples
    println!();
}

#[inline]
fn example1() {
    // even counter
    let mut cursor = Cursor::new_with_extras::<Counter>(SLICE);

    cursor.next_to_last();

    let extra = cursor.into_extras();

    assert_eq!(extra.0, 5);
    println!("{:?}", extra);
}

#[inline]
fn example2() {
    // space counter
    let mut cursor = StrCursor::new_with_extras::<Counter>(STRING);

    cursor.next_to_last();

    let extra = cursor.into_extras();

    assert_eq!(extra.0, 8);
    println!("{:?}", extra);
}

#[inline]
fn example3() {
    let mut cursor = StrCursor::new(STRING);
    cursor += 4;
    assert_eq!(cursor.current(), ' ');

    while let Some(ch) = &mut cursor + 1 {
        if let Some(next) = match ch {
            ' ' => &mut cursor + 1,
            '.' => &mut cursor + 2,
            _ => None,
        } {
            print!("{}", next);
        }
    }
}

#[inline]
fn example4() {
    let mut cursor = StrCursor::new(STRING);
    cursor.save();
    assert_eq!(cursor.saved().pos, 0);

    cursor += 4;
    assert_eq!(cursor.current(), ' ');

    assert_eq!(cursor.as_str_loaded(), "this ");

    cursor.save();
    assert_eq!(cursor.saved().pos, 4);

    cursor -= 2;
    assert_eq!(cursor.current(), 'i');

    assert_eq!(cursor.as_str_loaded(), "is ");

    let mut cursor = StrCursor::new("한글테스트^^");
    cursor.save();
    assert_eq!(cursor.saved().pos, 0);

    cursor += 4;
    assert_eq!(cursor.current(), '트');

    assert_eq!(cursor.as_str_loaded(), "한글테스트");

    cursor.save();
    cursor -= 2;

    assert_eq!(cursor.as_str_loaded(), "테스트");
}
