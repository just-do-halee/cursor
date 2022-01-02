#![allow(dead_code)]

use cursor::*;

use derive_new::*;
use std::str::FromStr;

derive_debug_partials! {

    #[derive(Default, PartialOrd, Ord, Clone, Copy, new)]
    struct Offset {
        pub pos: usize,
        pub line: usize,
        pub column: usize,
    }

    #[derive(Default, Clone, Copy, new)]
    struct Span {
        pub start: Offset,
        pub end: Offset,
    }

    #[derive(Clone, new)]
    struct SourceChunk<'s> {
        pub source: &'s str, // whole mass
        span: Span,
    }

    #[derive(Clone, Copy)]
    enum TokenKind {
        // Single-character tokens.
        LeftParen,
        RightParen,
        LeftBrace,
        RightBrace,
        Comma,
        Dot,
        Minus,
        Plus,
        Semicolon,
        Slash,
        Star,
        Ampersand,
        VerticalBar,
        Circumflex,

        // One or two character tokens.
        Bang,
        BangEqual,
        Equal,
        EqualEqual,
        Greater,
        GreaterEqual,
        Less,
        LessEqual,

        // Literals.
        Identifier,
        String,
        Number,

        // Keywords.
        And,
        Class,
        Else,
        False,
        Fun,
        For,
        If,
        Nil,
        Or,
        Print,
        Return,
        Super,
        This,
        True,
        Var,
        While,

        Eof,
    }

    #[derive(Clone)]
    enum Object {
        Identifier(String),
        String(String),
        Number(i32),
        Boolean(bool),
        Nil,
        None,
    }


}

#[derive(PartialEq, Eq, Clone, new)]
struct Token<'s> {
    pub kind: TokenKind,
    pub lexeme: SourceChunk<'s>,
    pub literal: Object,
}

type Tokens<'s> = Vec<Token<'s>>;

impl<'s> fmt::Debug for Token<'s> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Token")
            .field("kind", &self.kind)
            .field("lexeme", &self.lexeme.as_str())
            .field("literal", &self.literal)
            .finish()
    }
}

impl<'s> SourceChunk<'s> {
    pub fn as_str(&self) -> &'s str {
        let Span { start, end } = self.span;
        &self.source[start.pos..end.pos.saturating_add(1)]
    }
}

impl From<Range<LexerExtras>> for Span {
    fn from(v: Range<LexerExtras>) -> Self {
        Span {
            start: v.start.offset,
            end: v.end.offset,
        }
    }
}

impl<'s> From<&StrCursor<'s, LexerExtras>> for SourceChunk<'s> {
    fn from(cursor: &StrCursor<'s, LexerExtras>) -> Self {
        SourceChunk {
            source: cursor.as_str(),
            span: Span::from(cursor.to_range_extras()),
        }
    }
}

#[derive(Default, Debug)]
struct LexerExtras {
    prev_offset: Offset,
    offset: Offset,
}

impl Extras<char> for LexerExtras {
    fn new() -> Self {
        LexerExtras::default()
    }
    fn clone(&self) -> Self {
        LexerExtras {
            prev_offset: self.prev_offset,
            offset: self.offset,
        }
    }
    fn reset(&mut self) {
        self.prev_offset = Offset::default();
        self.offset = Offset::default();
    }
    fn change(&mut self, input: &char, pos: usize) {
        if self.offset.pos > pos {
            self.offset = self.prev_offset;
        } else {
            self.prev_offset = self.offset;
            self.offset.pos = pos;
            match *input {
                '\n' => {
                    self.offset.line += 1;
                    self.offset.column = 0;
                }
                _ => self.offset.column += 1,
            }
        }
    }
}

fn main() {
    example1();
    println!();
}

#[inline]
fn example1() {
    let mut cursor = StrCursor::new_with_extras::<LexerExtras>(
        r#"
            print 2 + 1;
            print "one";
            print true;

            if () {}
            /* U(Uj#$*()@#)(@!#&%#%NM) */ 

            if (1 != 2) {
                print "yes";
            }"#,
    );
    let mut tokens = Tokens::new();
    let undo = |cursor: &mut StrCursor<LexerExtras>| {
        cursor.next_to_left();
        cursor.head_to_right();
    };

    while let Some(c) = cursor.next() {
        match c {
            '+' => {
                cursor.save();
                tokens.push(Token::new(
                    TokenKind::Plus,
                    SourceChunk::from(&cursor),
                    Object::None,
                ))
            }
            '"' => {
                // strings
                cursor.save();
                cursor.next_to_until(|c| c == '"');
                let s = cursor.as_str_loaded();
                let literal = (&s[1..s.len().saturating_sub(1)]).to_string();
                tokens.push(Token::new(
                    TokenKind::String,
                    SourceChunk::from(&cursor),
                    Object::String(literal),
                ));
            }
            '0'..='9' => {
                // numbers
                cursor.save();
                cursor.next_to_while(|c| c.is_digit(10));
                undo(&mut cursor); // Undo
                let literal = i32::from_str(cursor.as_str_loaded()).unwrap();
                tokens.push(Token::new(
                    TokenKind::Number,
                    SourceChunk::from(&cursor),
                    Object::Number(literal),
                ));
            }
            'a'..='z' | 'A'..='Z' | '_' => {
                // ident
                cursor.save();
                cursor.next_to_while(|c| matches!(c, 'a'..='z' | 'A'..='Z' | '0'..='9' | '_'));
                undo(&mut cursor); // Undo
                let literal = cursor.as_str_loaded().to_string();
                tokens.push(Token::new(
                    TokenKind::Identifier,
                    SourceChunk::from(&cursor),
                    Object::Identifier(literal),
                ));
            }
            _ => {}
        }
    }
    let s = format!("{:#?}", tokens);
    assert_eq!(
        s,
        r#"[
    Token {
        kind: Identifier,
        lexeme: "print",
        literal: Identifier(
            "print",
        ),
    },
    Token {
        kind: Number,
        lexeme: "2",
        literal: Number(
            2,
        ),
    },
    Token {
        kind: Plus,
        lexeme: "+",
        literal: None,
    },
    Token {
        kind: Number,
        lexeme: "1",
        literal: Number(
            1,
        ),
    },
    Token {
        kind: Identifier,
        lexeme: "print",
        literal: Identifier(
            "print",
        ),
    },
    Token {
        kind: String,
        lexeme: "\"one\"",
        literal: String(
            "one",
        ),
    },
    Token {
        kind: Identifier,
        lexeme: "print",
        literal: Identifier(
            "print",
        ),
    },
    Token {
        kind: Identifier,
        lexeme: "true",
        literal: Identifier(
            "true",
        ),
    },
    Token {
        kind: Identifier,
        lexeme: "if",
        literal: Identifier(
            "if",
        ),
    },
    Token {
        kind: Identifier,
        lexeme: "U",
        literal: Identifier(
            "U",
        ),
    },
    Token {
        kind: Identifier,
        lexeme: "Uj",
        literal: Identifier(
            "Uj",
        ),
    },
    Token {
        kind: Identifier,
        lexeme: "NM",
        literal: Identifier(
            "NM",
        ),
    },
    Token {
        kind: Identifier,
        lexeme: "if",
        literal: Identifier(
            "if",
        ),
    },
    Token {
        kind: Number,
        lexeme: "1",
        literal: Number(
            1,
        ),
    },
    Token {
        kind: Number,
        lexeme: "2",
        literal: Number(
            2,
        ),
    },
    Token {
        kind: Identifier,
        lexeme: "print",
        literal: Identifier(
            "print",
        ),
    },
    Token {
        kind: String,
        lexeme: "\"yes\"",
        literal: String(
            "yes",
        ),
    },
]"#
    )
}

#[macro_export]
macro_rules! derive_debug_partials {
    (
        $(
            $i:item
        )*
    ) => {
        $(
            #[derive(Debug, PartialEq, Eq)]
            $i
        )*
    };
}
