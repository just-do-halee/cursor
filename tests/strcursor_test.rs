// Copyright 2021 Hwakyeom Kim(=just-do-halee)

use cursor::*;

const STRING: &str = "this is test. 안녕하세요. 이것은 #&*@( 테스트입니다. ^^ thanks.";

#[derive(Debug, Default)]
struct SpaceCounter(pub usize);

impl Extras<char> for SpaceCounter {
    fn new() -> Self {
        SpaceCounter::default()
    }
    fn clone(&self) -> Self {
        SpaceCounter(self.0)
    }
    fn reset(&mut self) {
        self.0 = 0;
    }
    fn change(&mut self, input: &char, _pos: usize) {
        if *input == ' ' {
            self.0 += 1;
        }
    }
}

#[test]
fn it_works() {
    let chars = STRING.chars().collect::<Vec<char>>();
    let cursor = StrCursor::new(STRING);
    for (i, ch) in cursor.enumerate() {
        assert_eq!(ch, chars[i]);
    }
}

#[test]
fn it_works2() {
    let chars = STRING.chars().collect::<Vec<char>>();
    let mut cursor = StrCursor::new(STRING);
    while let Some(ch) = cursor.next() {
        assert_eq!(ch, chars[cursor.pos()]);
    }
}

#[test]
fn current_works() {
    let chars = STRING.chars().collect::<Vec<char>>();
    let mut cursor = StrCursor::new(STRING);
    let mut i = 0;
    loop {
        let res = cursor.next();
        if res.is_none() {
            break;
        }
        let ch = res.unwrap();

        assert_eq!(ch, cursor.current());
        assert_eq!(cursor.current(), chars[i]);
        assert_eq!(cursor.current(), chars[i]);

        assert_eq!(ch, chars[i]);
        i += 1;
    }
}

#[test]
fn turnaround_works() {
    let mut cursor = StrCursor::new(STRING);

    for _ in 0..15 {
        cursor.next();
    }

    assert_eq!(Some('녕'), cursor.next());
    assert_eq!(Some('하'), cursor.next());
    assert_eq!(Some('세'), cursor.next());
    cursor.turnaround();
    assert_eq!(Some('하'), cursor.next());
    assert_eq!(Some('녕'), cursor.next());

    assert_eq!(Some('안'), cursor.next());
    cursor.turnaround();
    assert_eq!(Some('녕'), cursor.next());
    assert_eq!(Some('하'), cursor.next());
    assert_eq!(Some('세'), cursor.next());

    assert_eq!(Some('요'), cursor.next());
    assert_eq!(Some('.'), cursor.next());
    cursor.turnaround();
    assert_eq!(Some('요'), cursor.next());
    assert_eq!(Some('세'), cursor.next());
    assert_eq!(Some('하'), cursor.next());
    assert_eq!(Some('녕'), cursor.next());
    assert_eq!(Some('안'), cursor.next());
}

#[test]
fn shift_works() {
    let chars = STRING.chars().collect::<Vec<char>>();

    let mut cursor = StrCursor::new(STRING);
    cursor.next();
    let ch = cursor.next_to_offset(5).unwrap();
    assert_eq!(ch, chars[5]);
    let ch = cursor.next_to_offset(-5).unwrap();
    assert_eq!(ch, chars[0]);

    assert!(cursor.backwards());
    cursor.jump_to_last();
    assert!(cursor.backwards());

    let ch = cursor.current();
    assert_eq!(ch, chars[chars.len() - 1]);

    let ch = cursor.next();
    assert_eq!(ch, Some(chars[chars.len() - 2]));
}

#[test]
fn str_works() {
    let mut cursor = StrCursor::new(STRING);
    cursor.next_to_offset(16);
    assert_eq!(
        format!(
            "{}  {}  {}",
            cursor.as_preserved_str(),
            cursor.current(),
            cursor.as_remaining_str()
        ),
        "this is test. 안  녕  하세요. 이것은 #&*@( 테스트입니다. ^^ thanks."
    );
}

#[test]
fn assign_works() {
    let mut cursor = StrCursor::new(STRING);
    cursor += 4;
    assert_eq!(cursor.current(), ' ');

    let i = &mut cursor - 4;
    assert_eq!(i.unwrap(), 't');

    let j = cursor + 3;
    assert_eq!(j.unwrap(), 's');
}

#[test]
fn save_load_works() {
    {
        let mut cursor = StrCursor::new(STRING);
        cursor.save();
        assert_eq!(cursor.saved().pos, 0);

        cursor += 4;
        assert_eq!(cursor.current(), ' ');
        assert_eq!(cursor.pos(), 4);

        assert_eq!(cursor.as_str_loaded(), "this ");

        cursor.save();
        assert_eq!(cursor.saved().pos, 4);

        cursor -= 2;
        assert_eq!(cursor.current(), 'i');

        assert_eq!(cursor.as_str_loaded(), "is ");
    }
    {
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
}

#[test]
fn extras_works() {
    let mut cursor = StrCursor::new_with_extras::<SpaceCounter>(STRING);
    cursor.next_to_last();
    assert_eq!(cursor.into_extras().0, 8);
}
