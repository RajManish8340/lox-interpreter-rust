#[derive(Debug)]
pub(crate) enum TokenKind {
    LEFT_PARAN,
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
    BANG,
    BANG_EQUAL,
    EQUAL,
    EQUAL_EQUAL,
    GREATER,
    GREATER_EQUAL,
    LESS,
    LESS_EQUAL,
    LITERALS,
    IDENTIFIERS,
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

struct Token {
    kind: TokenKind,
    lexeme: String,
    literal: String,
    line: u32,
}

struct Scanner {
    source: Vec<char>,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: u32,
}

impl Scanner {
    fn new(source: &str) -> Self {
        Self {
            source: source.chars().collect(),
            tokens: vec![],
            start: 0,
            current: 0,
            line: 1,
        }
    }

    fn advance(&mut self) -> char {
        let prev = self.current;
        self.current += 1;
        return self.source[prev];
    }

    fn peak(&self) -> Option<char> {
        if self.current >= self.source.len() {
            return None;
        }
        Some(self.source[self.current])
    }

    fn scan_token(&mut self) -> Vec<Token> {
        while self.current < self.source.len() {
            self.start = self.current;
            let c = self.advance();

            match c {}
        }
        vec![]
    }
}
