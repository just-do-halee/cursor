// Copyright 2021 Hwakyeom Kim(=just-do-halee)

use cursor::*;

const SLICE: &[u8] = &[1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

#[derive(Debug, Default)]
struct EvenCounter(pub usize);

impl Extras<u8> for EvenCounter {
    fn new() -> Self {
        EvenCounter::default()
    }
    fn clone(&self) -> Self {
        EvenCounter(self.0)
    }
    fn reset(&mut self) {
        self.0 = 0;
    }
    fn change(&mut self, input: &u8, _pos: usize) {
        if input % 2 == 0 {
            self.0 += 1;
        }
    }
}

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
fn jump_works() {
    let mut cursor = Cursor::new(SLICE);
    let mut ch;

    ch = *cursor.jump_to_offset(6).unwrap();
    assert_eq!(ch, SLICE[6]);
    ch = *cursor.jump_to_offset(-6).unwrap();
    assert_eq!(ch, SLICE[0]);

    ch = *cursor.jump_to_last();

    assert_eq!(ch, SLICE[SLICE.len() - 1]);
    assert_eq!(ch, 10);

    assert!(!cursor.backwards());
    cursor.head_to_left();
    assert!(cursor.backwards());

    ch = cursor.unwrapped_next();
    assert_eq!(ch, SLICE[SLICE.len() - 2]);
    assert_eq!(ch, 9);
}

#[test]
fn slice_works() {
    let mut cursor = Cursor::new(SLICE);
    cursor.jump(4);
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

    let i = &mut cursor - 4;
    assert_eq!(*i.unwrap(), 2);

    let j = cursor + 5;
    assert_eq!(*j.unwrap(), 7);
}

#[test]
fn save_load_works() {
    let mut cursor = Cursor::new(SLICE);
    cursor.save();
    assert_eq!(cursor.saved().pos, 0);

    cursor += 4;
    assert_eq!(*cursor.current(), 5);

    assert_eq!(cursor.as_slice_loaded(), &[1, 2, 3, 4, 5]);

    cursor.save();
    assert_eq!(cursor.saved().pos, 4);

    cursor -= 3;
    assert_eq!(*cursor.current(), 2);

    assert_eq!(cursor.as_slice_loaded(), &[2, 3, 4, 5]);
}

#[test]
fn extras_works() {
    let mut cursor = Cursor::new_with_extras::<EvenCounter>(SLICE);
    assert_eq!(cursor.to_extras().0, 0);
    assert_eq!(cursor.saved().extras.0, 0);

    assert_eq!(*cursor.next_to_last(), SLICE[SLICE.len() - 1]);
    assert_eq!(cursor.to_extras().0, 5);
    assert_eq!(cursor.saved().extras.0, 0);

    cursor.save();
    assert_eq!(cursor.saved().extras.0, 5);

    cursor.reset();
    assert_eq!(cursor.to_extras().0, 0);
    assert_eq!(cursor.saved().extras.0, 5);

    cursor.next_to_offset(3);
    assert_eq!(cursor.to_extras().0, 1);
    assert_eq!(cursor.saved().extras.0, 5);

    cursor.save();
    cursor.next_to_last();

    assert_eq!(cursor.to_extras().0, 5);
    assert_eq!(cursor.saved().extras.0, 1);
}
