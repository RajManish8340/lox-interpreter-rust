use core::fmt;

#[allow(non_camel_case_types)]
#[derive(Debug, Clone)]
pub(crate) enum TokenKind {
    LEFT_PAREN,
    RIGHT_PAREN,
    LEFT_BRACE,
    RIGHT_BRACE,
    DOT,
    COMMA,
    MINUS,
    PLUS,
    SEMICOLON,
    SLASH,
    STAR,
    BANG,
    BANG_EQUAL,
    EQUAL,
    EQUAL_EQUAL,
    GREATER,
    GREATER_EQUAL,
    LESS,
    LESS_EQUAL,
    IDENTIFIER,
    STRING,
    NUMBER,
    AND,
    CLASS,
    ELSE,
    FALSE,
    FUN,
    FOR,
    IF,
    NIL,
    OR,
    PRINT,
    RETURN,
    SUPER,
    THIS,
    TRUE,
    VAR,
    WHILE,
    EOF,
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
