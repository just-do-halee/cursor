// Copyright 2021 Hwakyeom Kim(=just-do-halee)

use cursor::StrCursor;

const STRING: &str = "this is test. 안녕하세요. 이것은 #&*@( 테스트입니다. ^^ thanks.";

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
fn const_works() {
    assert_eq!(StrCursor::EOF, '\0');
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
    let ch = cursor.right_shift(5).unwrap();
    assert_eq!(ch, chars[5]);
    let ch = cursor.left_shift(5).unwrap();
    assert_eq!(ch, chars[0]);

    cursor.shift_last();

    let ch = cursor.current();
    assert_eq!(ch, chars[chars.len() - 1]);

    cursor.turnaround();

    let ch = cursor.next().unwrap();
    assert_eq!(ch, chars[chars.len() - 2]);
}

#[test]
fn str_works() {
    let mut cursor = StrCursor::new(STRING);
    cursor.right_shift(15);
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
    let mut cursor = StrCursor::new(STRING);
    cursor.save();
    assert_eq!(cursor.saved_pos(), 0);

    cursor += 4;
    assert_eq!(cursor.current(), ' ');

    assert_eq!(cursor.load_str(), "this ");

    cursor.save();
    assert_eq!(cursor.saved_pos(), 4);

    cursor -= 2;
    assert_eq!(cursor.current(), 'i');

    assert_eq!(cursor.load_str(), "is ");

    let mut cursor = StrCursor::new("한글테스트^^");
    cursor.save();
    assert_eq!(cursor.saved_pos(), 0);

    cursor += 4;
    assert_eq!(cursor.current(), '트');

    assert_eq!(cursor.load_str(), "한글테스트");

    cursor.save();
    cursor -= 2;

    assert_eq!(cursor.load_str(), "테스트");
}