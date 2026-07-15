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
    IDENTIFIERS,
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
    fn with_literal(kind: TokenKind, lexeme: String, literal: String, line: usize) -> Self {
        Self {
            kind,
            lexeme,
            literal,
            line,
        }
    }
}

#[derive(Clone)]
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
    fn is_at_end(&self) -> bool {
        if self.current >= self.source.len() {
            true
        } else {
            false
        }
    }

    fn string(&mut self) {
        while self.peak() != Some('"') && !self.is_at_end() {
            if self.peak() == Some('\n') {
                self.line += 1;
            }
            self.advance();
        }

        if self.is_at_end() {
            self.errors.push(LexicalError::new(
                String::from(r#"""#),
                self.line,
                String::from("Unterminated String"),
            ));
            return;
        }

        self.advance();
        let literal: String = self.source[self.start + 1..self.current - 1]
            .iter()
            .collect();

        let lexeme: String = self.source[self.start..self.current].iter().collect();

        self.tokens.push(Token::with_literal(
            TokenKind::STRING,
            lexeme,
            literal,
            self.line,
        ));
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
                '/' => {
                    if self.peak() == Some('/') {
                        while self.peak() != Some('\n') && !self.is_at_end() {
                            self.advance();
                        }
                        self.advance();
                    } else {
                        self.tokens
                            .push(Token::new(TokenKind::SLASH, c.to_string(), self.line));
                    }
                }
                '"' => {
                    self.string();
                }
                other => self.errors.push(LexicalError::new(
                    other.to_string(),
                    self.line,
                    String::from("Unexpected character"),
                )),
            }
        }

        self.tokens
            .push(Token::new(TokenKind::EOF, ' '.to_string(), self.line));
        (self.tokens.clone(), self.errors.clone())
    }
}
