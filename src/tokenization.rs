#[allow(non_camel_case_types)]
#[derive(Debug)]
pub(crate) enum TokenType {
    // Single-character tokens.
    lEFT_PARAN,
    RIGHT_PARAN,
    LEFT_BRACE,
    RIGHT_BRACE,
    DOT,
    COMMA,
    MINUS,
    PLUS,
    SEMICOLON,
    SLASH,
    STAR,

    // One of two char tokens
    BANG,
    BANG_EQUAL,
    EQUAL,
    EQUAL_EQUAL,
    GREATER,
    GREATER_EQUAL,
    LESS,
    LESS_EQUAL,

    // literals
    LITERALS,
    IDENTIFIERS,
    NUMBER,

    // keywords
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

pub(crate) struct Token {
    token: TokenType,
    lexeme: String,
    literal: String,
    line: u8,
}

impl Token {
    fn print(&self) {
        println!("{:?} {:?} {:?}", self.token, self.lexeme, self.literal)
    }
}

pub(crate) fn scan_tokens(file_content: &str) {}
