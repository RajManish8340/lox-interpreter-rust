use std::char;

use crate::token::{LexicalError, LiteralType, Token, TokenKind};

#[derive(Debug, Clone)]
pub(crate) struct Scanner {
    source: Vec<char>,
    pub(crate) tokens: Vec<Token>,
    errors: Vec<LexicalError>,
    start: usize,
    current: usize,
    line: usize,
}

fn is_digit(c: char) -> bool {
    match c {
        '0'..='9' => return true,
        _ => return false,
    }
}

fn is_alpha(c: char) -> bool {
    return matches!(c, 'a'..='z' | 'A'..='Z' | '_');
}

fn is_alpha_numeric(c: char) -> bool {
    return is_digit(c) | is_alpha(c);
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

    fn peak(&self) -> char {
        if self.current >= self.source.len() {
            return '\0';
        }
        self.source[self.current]
    }

    fn peak_next(&self) -> char {
        if self.current + 1 >= self.source.len() {
            return '\0';
        }
        self.source[self.current + 1]
    }

    fn is_at_end(&self) -> bool {
        if self.current >= self.source.len() {
            true
        } else {
            false
        }
    }

    fn string(&mut self) {
        while self.peak() != '"' && !self.is_at_end() {
            if self.peak() == '\n' {
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

        self.tokens.push(Token::new(
            TokenKind::String,
            lexeme,
            Some(LiteralType::String(literal)),
            self.line,
        ));
    }

    fn number(&mut self) {
        while is_digit(self.peak()) {
            self.advance();
        }
        if self.peak() == '.' && is_digit(self.peak_next()) {
            self.advance();
            while is_digit(self.peak()) {
                self.advance();
            }
        }
        let lexeme_non_parsed: String = self.source[self.start..self.current].iter().collect();
        let lexeme = lexeme_non_parsed
            .parse::<f64>()
            .unwrap_or_else(|_| 0.0000000000);
        let literal = lexeme;
        self.tokens.push(Token::new(
            TokenKind::Number,
            lexeme.to_string(),
            Some(LiteralType::Number(literal)),
            self.line,
        ));
    }

    fn identifier(&mut self) {
        while is_alpha_numeric(self.peak()) {
            self.advance();
        }
        let lexeme: String = self.source[self.start..self.current].iter().collect();

        let kind: TokenKind = match lexeme.as_str() {
            "and" => TokenKind::And,
            "class" => TokenKind::Class,
            "else" => TokenKind::Else,
            "false" => TokenKind::False,
            "for" => TokenKind::For,
            "fun" => TokenKind::Fun,
            "if" => TokenKind::If,
            "nil" => TokenKind::Nil,
            "or" => TokenKind::Or,
            "print" => TokenKind::Print,
            "return" => TokenKind::Return,
            "super" => TokenKind::Super,
            "this" => TokenKind::This,
            "true" => TokenKind::True,
            "var" => TokenKind::Var,
            "while" => TokenKind::While,
            _ => TokenKind::Identifier,
        };

        self.tokens.push(Token::new(kind, lexeme, None, self.line));
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
                '(' => self.tokens.push(Token::new(
                    TokenKind::LeftParen,
                    c.to_string(),
                    None,
                    self.line,
                )),
                ')' => self.tokens.push(Token::new(
                    TokenKind::RightParen,
                    c.to_string(),
                    None,
                    self.line,
                )),
                '{' => self.tokens.push(Token::new(
                    TokenKind::LeftBrace,
                    c.to_string(),
                    None,
                    self.line,
                )),
                '}' => self.tokens.push(Token::new(
                    TokenKind::RightBrace,
                    c.to_string(),
                    None,
                    self.line,
                )),

                ',' => {
                    self.tokens
                        .push(Token::new(TokenKind::Comma, c.to_string(), None, self.line))
                }
                '.' => self
                    .tokens
                    .push(Token::new(TokenKind::Dot, c.to_string(), None, self.line)),
                '-' => {
                    self.tokens
                        .push(Token::new(TokenKind::Minus, c.to_string(), None, self.line))
                }
                '+' => {
                    self.tokens
                        .push(Token::new(TokenKind::Plus, c.to_string(), None, self.line))
                }
                ';' => self.tokens.push(Token::new(
                    TokenKind::Semicolon,
                    c.to_string(),
                    None,
                    self.line,
                )),
                '*' => {
                    self.tokens
                        .push(Token::new(TokenKind::Star, c.to_string(), None, self.line))
                }

                '!' => {
                    if self.peak() == '=' {
                        self.tokens.push(Token::new(
                            TokenKind::BangEqual,
                            format!("{}=", c),
                            None,
                            self.line,
                        ));
                        self.advance();
                    } else {
                        self.tokens.push(Token::new(
                            TokenKind::Bang,
                            c.to_string(),
                            None,
                            self.line,
                        ));
                    }
                }

                '=' => {
                    if self.peak() == '=' {
                        self.tokens.push(Token::new(
                            TokenKind::EqualEqual,
                            format!("{}=", c),
                            None,
                            self.line,
                        ));
                        self.advance();
                    } else {
                        self.tokens.push(Token::new(
                            TokenKind::EqualEqual,
                            c.to_string(),
                            None,
                            self.line,
                        ));
                    }
                }

                '<' => {
                    if self.peak() == '=' {
                        self.tokens.push(Token::new(
                            TokenKind::LessEqual,
                            format!("{}=", c),
                            None,
                            self.line,
                        ));
                        self.advance();
                    } else {
                        self.tokens.push(Token::new(
                            TokenKind::Less,
                            c.to_string(),
                            None,
                            self.line,
                        ));
                    }
                }

                '>' => {
                    if self.peak() == '=' {
                        self.tokens.push(Token::new(
                            TokenKind::GreaterEqual,
                            format!("{}=", c),
                            None,
                            self.line,
                        ));
                        self.advance();
                    } else {
                        self.tokens.push(Token::new(
                            TokenKind::Greater,
                            c.to_string(),
                            None,
                            self.line,
                        ));
                    }
                }

                '/' => {
                    if self.peak() == '/' {
                        while self.peak() != '\n' && !self.is_at_end() {
                            self.advance();
                        }
                        if self.peak() == '\n' {
                            self.line += 1;
                        }
                        if !self.is_at_end() {
                            self.advance();
                        }
                    } else if self.peak() == '*' {
                        self.advance();
                        while !(self.peak() == '*' && self.peak_next() == '/') && !self.is_at_end()
                        {
                            if self.peak() == '\n' {
                                self.line += 1;
                            }
                            self.advance();
                        }
                        if self.is_at_end() {
                            self.errors.push(LexicalError::new(
                                "*/".to_string(),
                                self.line - 1,
                                String::from("block comment not closed"),
                            ));
                            continue;
                        }
                        self.advance();
                        self.advance();
                    } else {
                        self.tokens.push(Token::new(
                            TokenKind::Slash,
                            c.to_string(),
                            None,
                            self.line,
                        ));
                    }
                }

                '"' => {
                    self.string();
                }

                '0'..='9' => {
                    self.number();
                }
                'a'..='z' | 'A'..='Z' | '_' => {
                    self.identifier();
                }
                other => self.errors.push(LexicalError::new(
                    other.to_string(),
                    self.line,
                    String::from("Unexpected character"),
                )),
            }
        }

        self.tokens
            .push(Token::new(TokenKind::Eof, ' '.to_string(), None, self.line));
        (self.tokens.clone(), self.errors.clone())
    }
}
