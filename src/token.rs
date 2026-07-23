use core::fmt;

#[derive(Debug, Clone, PartialEq)] // Added PartialEq for easier token comparing
pub(crate) enum TokenKind {
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Dot,
    Comma,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,

    // One or two character tokens
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    // Literals
    Identifier,
    String,
    Number,

    // Keywords
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
#[derive(Clone, Debug)]
pub(crate) enum LiteralType {
    String(String),
    Number(f64),
}

impl fmt::Display for LiteralType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LiteralType::String(s) => write!(f, "{}", s),
            LiteralType::Number(n) => {
                if n.fract() == 0.0 {
                    write!(f, "{}.0", n)
                } else {
                    write!(f, "{}", n)
                }
            }
        }
    }
}

#[derive(Debug, Clone)]
pub(crate) struct Token {
    pub(crate) kind: TokenKind,
    pub(crate) lexeme: String,
    pub(crate) literal: Option<LiteralType>,
    pub(crate) line: usize,
}

impl Token {
    pub(crate) fn new(
        kind: TokenKind,
        lexeme: String,
        literal: Option<LiteralType>,
        line: usize,
    ) -> Self {
        Self {
            kind,
            lexeme,
            literal,
            line,
        }
    }
}

#[derive(Clone, Debug)]
pub(crate) struct LexicalError {
    pub(crate) character: String,
    pub(crate) line: usize,
    pub(crate) message: String,
}

impl LexicalError {
    pub(crate) fn new(character: String, line: usize, message: String) -> Self {
        Self {
            character,
            line,
            message,
        }
    }
}
