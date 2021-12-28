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
    example10();
    println!();
}

#[inline]
fn example1() {
    let cursor = Cursor::new(&[1u8; 100]);
    for i in cursor {
        print!("{} ", i);
    }
}

#[inline]
fn example2() {
    let mut cursor = Cursor::new(&[1u8; 100]);
    while let Some(i) = cursor.next() {
        print!("{} ", i);
    }
}

#[inline]
fn example3() {
    let mut cursor = Cursor::new(&[1u8; 100]);
    for _ in cursor.range() {
        let i = cursor.unwrapped_next();
        print!("{} ", i);
    }
}

#[inline]
fn example4() {
    let mut cursor = Cursor::new(&[1u8; 100]);
    for _ in cursor.range() {
        cursor += 1; // this is jump
        print!("{} ", cursor.current());
    }
}

#[inline]
fn example5() {
    let mut cursor = Cursor::new(&[1u8; 100]);
    for _ in cursor.range() {
        let ch = &mut cursor + 1; // this is jump
        print!("{} ", ch.unwrap());
    }
}

#[inline]
fn example6() {
    let mut cursor = Cursor::new(&[1u8; 100]);
    // this is jump
    while let Some(ch) = &mut cursor + 1 {
        print!("{} ", ch);
    }
}

#[inline]
fn example7() -> Option<u8> {
    let mut cursor = Cursor::new(&[1u8; 100]);
    cursor.jump(5)?;
    cursor.jump_to_offset(-5);

    cursor.turnaround();
    cursor.next()?;
    cursor.next()?;
    cursor.turnaround();
    cursor.next()?;

    cursor.next().copied()
}

#[inline]
fn example8() -> u8 {
    let mut cursor = Cursor::new(&[1u8; 100]);
    cursor += 5;
    cursor -= 2;
    cursor += 1;

    cursor.current_deref()
}

#[inline]
fn example9() -> u8 {
    let mut cursor = Cursor::new(&[1u8; 100]);
    cursor += 10;

    let _ = &mut cursor + (5 - 2 + 1);
    let _ = &mut cursor - 1;

    *(cursor - 1).unwrap()
}

#[inline]
fn example10() {
    let any_source = (1..1000).collect::<Vec<usize>>();

    let mut cursor = Cursor::new(any_source.as_slice());

    let mut tough_rate = 0f64;

    for _ in cursor.range() {
        cursor += 1;

        let n = cursor.current_deref();

        // odd numb
        if n % 2 != 0 {
            let mut tough = 0;
            // previous numb list
            for &m in cursor.as_preserved_slice() {
                if n % m != 0 {
                    tough += 1;
                }
            }
            tough_rate = (tough as f64) / (n as f64);
        }
    }

    println!("{:?}", tough_rate);
}
