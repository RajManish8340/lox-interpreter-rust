use std::fmt::format;

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

#[derive(Clone)]
pub(crate) struct Token {
    pub(crate) kind: TokenKind,
    pub(crate) lexeme: String,
    pub(crate) literal: String,
    pub(crate) line: usize,
}

impl Token {
    fn new(kind: TokenKind, lexeme: String, line: usize) -> Self {
        Self {
            kind,
            lexeme,
            literal: String::from("null"),
            line,
        }
    }
}

#[derive(Clone)]
pub(crate) struct LexicalError {
    pub(crate) character: String,
    pub(crate) line: usize,
}

impl LexicalError {
    pub(crate) fn new(character: String, line: usize) -> Self {
        Self { character, line }
    }
}

pub(crate) struct Scanner {
    source: Vec<char>,
    tokens: Vec<Token>,
    errors: Vec<LexicalError>,
    start: usize,
    current: usize,
    line: usize,
}

impl Scanner {
    pub(crate) fn new(source: &str) -> Self {
        Self {
            source: source.chars().collect(),
            tokens: vec![],
            errors: vec![],
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

    pub(crate) fn scan_token(&mut self) -> (Vec<Token>, Vec<LexicalError>) {
        while self.current < self.source.len() {
            self.start = self.current;
            let c = self.advance();

            match c {
                '\n' => self.line = self.line + 1,
                ' ' => {}
                '\r' => {}
                '\t' => {}
                '(' => {
                    self.tokens
                        .push(Token::new(TokenKind::LEFT_PAREN, c.to_string(), self.line))
                }
                ')' => {
                    self.tokens
                        .push(Token::new(TokenKind::RIGHT_PAREN, c.to_string(), self.line))
                }
                '{' => {
                    self.tokens
                        .push(Token::new(TokenKind::LEFT_BRACE, c.to_string(), self.line))
                }
                '}' => {
                    self.tokens
                        .push(Token::new(TokenKind::RIGHT_BRACE, c.to_string(), self.line))
                }
                ',' => self
                    .tokens
                    .push(Token::new(TokenKind::COMMA, c.to_string(), self.line)),
                '.' => self
                    .tokens
                    .push(Token::new(TokenKind::DOT, c.to_string(), self.line)),
                '-' => self
                    .tokens
                    .push(Token::new(TokenKind::MINUS, c.to_string(), self.line)),
                '+' => self
                    .tokens
                    .push(Token::new(TokenKind::PLUS, c.to_string(), self.line)),
                ';' => self
                    .tokens
                    .push(Token::new(TokenKind::SEMICOLON, c.to_string(), self.line)),
                '/' => self
                    .tokens
                    .push(Token::new(TokenKind::SLASH, c.to_string(), self.line)),
                '*' => self
                    .tokens
                    .push(Token::new(TokenKind::STAR, c.to_string(), self.line)),
                '!' => {
                    if self.peak() == Some('=') {
                        self.tokens.push(Token::new(
                            TokenKind::BANG_EQUAL,
                            format!("{}=", c),
                            self.line,
                        ));
                        self.advance();
                    } else {
                        self.tokens
                            .push(Token::new(TokenKind::BANG, c.to_string(), self.line));
                    }
                }
                '=' => {
                    if self.peak() == Some('=') {
                        self.tokens.push(Token::new(
                            TokenKind::EQUAL_EQUAL,
                            format!("{}=", c),
                            self.line,
                        ));
                        self.advance();
                    } else {
                        self.tokens
                            .push(Token::new(TokenKind::EQUAL, c.to_string(), self.line));
                    }
                }
                '<' => {
                    if self.peak() == Some('=') {
                        self.tokens.push(Token::new(
                            TokenKind::LESS_EQUAL,
                            format!("{}=", c),
                            self.line,
                        ));
                        self.advance();
                    } else {
                        self.tokens
                            .push(Token::new(TokenKind::LESS, c.to_string(), self.line));
                    }
                }
                '>' => {
                    if self.peak() == Some('=') {
                        self.tokens.push(Token::new(
                            TokenKind::GREATER_EQUAL,
                            format!("{}=", c),
                            self.line,
                        ));
                        self.advance();
                    } else {
                        self.tokens
                            .push(Token::new(TokenKind::GREATER, c.to_string(), self.line));
                    }
                }
                other => self
                    .errors
                    .push(LexicalError::new(other.to_string(), self.line)),
            }
        }

        self.tokens
            .push(Token::new(TokenKind::EOF, ' '.to_string(), self.line));
        (self.tokens.clone(), self.errors.clone())
    }
}
