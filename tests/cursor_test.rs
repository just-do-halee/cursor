// Copyright 2021 Hwakyeom Kim(=just-do-halee)

use cursor::{Cursor, CursorTrait};

const SLICE: &[u8] = &[1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

#[test]
fn it_works() {
    let cursor = Cursor::new(SLICE);
    for (i, &ch) in cursor.enumerate() {
        assert_eq!(ch, SLICE[i]);
    }
}

#[test]
fn it_works2() {
    let mut cursor = Cursor::new(SLICE);
    while let Some(&ch) = cursor.next() {
        assert_eq!(ch, SLICE[cursor.pos()]);
    }
}

#[test]
fn const_works() {
    let cursor = Cursor::new(SLICE);
    assert_eq!(cursor.item_size(), 1);
}

#[test]
fn current_works() {
    let mut cursor = Cursor::new(SLICE);
    let mut i = 0;
    loop {
        let res = cursor.next();
        if res.is_none() {
            break;
        }
        let ch = *res.unwrap();

        assert_eq!(ch, *cursor.current());
        assert_eq!(*cursor.current(), SLICE[i]);
        assert_eq!(*cursor.current(), SLICE[i]);

        assert_eq!(ch, SLICE[i]);
        i += 1;
    }
}

#[test]
fn turnaround_works() {
    let mut cursor = Cursor::new(SLICE);
    while let Some(&ch) = cursor.next() {
        let saved_pos = cursor.pos();

        // check
        assert_eq!(ch, SLICE[saved_pos]);

        if saved_pos > 0 {
            // check prev
            cursor.turnaround();
            if let Some(&prev_ch) = cursor.next() {
                assert_eq!(prev_ch, SLICE[saved_pos - 1]);
            }

            // get back
            cursor.turnaround();
            cursor.next();
        }
    }
}

#[test]
fn shift_works() {
    let mut cursor = Cursor::new(SLICE);
    let ch = *cursor.right_shift(6).unwrap();
    assert_eq!(ch, SLICE[6]);
    let ch = *cursor.left_shift(6).unwrap();
    assert_eq!(ch, SLICE[0]);

    cursor.shift_last();

    let ch = *cursor.current();
    assert_eq!(ch, SLICE[SLICE.len() - 1]);

    cursor.turnaround();

    let ch = *cursor.next().unwrap();
    assert_eq!(ch, SLICE[SLICE.len() - 2]);
}

#[test]
fn slice_works() {
    let mut cursor = Cursor::new(SLICE);
    cursor.right_shift(4);
    assert_eq!(
        format!(
            "{:?}  {}  {:?}",
            cursor.as_preserved_slice(),
            cursor.current(),
            cursor.as_remaining_slice()
        ),
        "[1, 2, 3, 4]  5  [6, 7, 8, 9, 10]"
    );
}

#[test]
fn assign_works() {
    let mut cursor = Cursor::new(SLICE);
    cursor += 5;
    assert_eq!(*cursor.current(), 6);

    let i = &mut cursor - 5;
    assert_eq!(*i.unwrap(), 1);

    let j = cursor + 4;
    assert_eq!(*j.unwrap(), 5);
}

#[test]
fn save_load_works() {
    let mut cursor = Cursor::new(SLICE);
    cursor.save();
    assert_eq!(cursor.saved_pos(), 0);

    cursor += 4;
    assert_eq!(*cursor.current(), 5);

    assert_eq!(cursor.load_slice(), &[1, 2, 3, 4, 5]);

    cursor.save();
    assert_eq!(cursor.saved_pos(), 4);

    cursor -= 3;
    assert_eq!(*cursor.current(), 2);

    assert_eq!(cursor.load_slice(), &[2, 3, 4, 5]);
}
