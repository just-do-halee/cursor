#![allow(clippy::while_let_on_iterator)]

use cursor::*;

fn main() {
    example1();
    example2();
    example3();
    example4();
    example5();
    example6();
    example7();
    example8();
    example9();
    println!();
}

#[inline]
fn example1() {
    let cursor = StrCursor::new("test입니다. ^^");
    for i in cursor {
        print!("{} ", i);
    }
}

#[inline]
fn example2() {
    let mut cursor = StrCursor::new("test입니다. ^^");
    while let Some(i) = cursor.next() {
        print!("{} ", i);
    }
}

#[inline]
fn example3() {
    let mut cursor = StrCursor::new("test ascii only!");
    for _ in cursor.range() {
        let i = cursor.unwrapped_next();
        print!("{} ", i);
    }
}

#[inline]
fn example4() {
    let mut cursor = StrCursor::new("test ascii only!");
    for _ in cursor.range() {
        cursor += 1;
        print!("{} ", cursor.current());
    }
}

#[inline]
fn example5() {
    let mut cursor = StrCursor::new("test ascii only!");
    while let Some(ch) = &mut cursor + 1 {
        print!("{} ", ch);
    }
}

#[inline]
fn example6() -> Option<char> {
    let mut cursor = StrCursor::new("test입니다. ^^");

    cursor.next_to_offset(5)?;
    cursor.next_to_offset(-5)?;

    cursor.turnaround();
    cursor.next()?;
    cursor.next()?;
    cursor.turnaround();
    cursor.next()?;

    cursor.next()
}

#[inline]
fn example7() -> char {
    let mut cursor = StrCursor::new("test입니다. ^^");
    cursor += 5;
    cursor -= 2;
    cursor += 1;

    cursor.current()
}

#[inline]
fn example8() -> char {
    let mut cursor = StrCursor::new("test입니다. ^^");
    cursor += 10;

    let _ = &mut cursor + (5 - 2 + 1);
    let _ = &mut cursor - 1;

    (cursor - 1).unwrap()
}

#[inline]
fn example9() {
    let mut cursor = StrCursor::new("test입니다. ^^");
    cursor += 5;

    assert_eq!(cursor.as_preserved_str(), "test입");
    assert_eq!(cursor.current(), '니');
    assert_eq!(cursor.as_remaining_str(), "다. ^^");
}
