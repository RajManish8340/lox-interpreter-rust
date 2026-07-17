use std::char;

use crate::tokens::{LexicalError, LiteralType, Token, TokenKind};

#[derive(Debug, Clone)]
pub(crate) struct Scanner {
    source: Vec<char>,
    tokens: Vec<Token>,
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
            TokenKind::STRING,
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
            TokenKind::NUMBER,
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

        let kind = match lexeme.as_str() {
            "and" => TokenKind::AND,
            "class" => TokenKind::CLASS,
            "else" => TokenKind::ELSE,
            "false" => TokenKind::FALSE,
            "for" => TokenKind::FOR,
            "fun" => TokenKind::FUN,
            "if" => TokenKind::IF,
            "nil" => TokenKind::NIL,
            "or" => TokenKind::OR,
            "print" => TokenKind::PRINT,
            "return" => TokenKind::RETURN,
            "super" => TokenKind::SUPER,
            "this" => TokenKind::THIS,
            "true" => TokenKind::TRUE,
            "var" => TokenKind::VAR,
            "while" => TokenKind::WHILE,
            _ => TokenKind::IDENTIFIER,
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
                    TokenKind::LEFT_PAREN,
                    c.to_string(),
                    None,
                    self.line,
                )),
                ')' => self.tokens.push(Token::new(
                    TokenKind::RIGHT_PAREN,
                    c.to_string(),
                    None,
                    self.line,
                )),
                '{' => self.tokens.push(Token::new(
                    TokenKind::LEFT_BRACE,
                    c.to_string(),
                    None,
                    self.line,
                )),
                '}' => self.tokens.push(Token::new(
                    TokenKind::RIGHT_BRACE,
                    c.to_string(),
                    None,
                    self.line,
                )),

                ',' => {
                    self.tokens
                        .push(Token::new(TokenKind::COMMA, c.to_string(), None, self.line))
                }
                '.' => self
                    .tokens
                    .push(Token::new(TokenKind::DOT, c.to_string(), None, self.line)),
                '-' => {
                    self.tokens
                        .push(Token::new(TokenKind::MINUS, c.to_string(), None, self.line))
                }
                '+' => {
                    self.tokens
                        .push(Token::new(TokenKind::PLUS, c.to_string(), None, self.line))
                }
                ';' => self.tokens.push(Token::new(
                    TokenKind::SEMICOLON,
                    c.to_string(),
                    None,
                    self.line,
                )),
                '*' => {
                    self.tokens
                        .push(Token::new(TokenKind::STAR, c.to_string(), None, self.line))
                }

                '!' => {
                    if self.peak() == '=' {
                        self.tokens.push(Token::new(
                            TokenKind::BANG_EQUAL,
                            format!("{}=", c),
                            None,
                            self.line,
                        ));
                        self.advance();
                    } else {
                        self.tokens.push(Token::new(
                            TokenKind::BANG,
                            c.to_string(),
                            None,
                            self.line,
                        ));
                    }
                }

                '=' => {
                    if self.peak() == '=' {
                        self.tokens.push(Token::new(
                            TokenKind::EQUAL_EQUAL,
                            format!("{}=", c),
                            None,
                            self.line,
                        ));
                        self.advance();
                    } else {
                        self.tokens.push(Token::new(
                            TokenKind::EQUAL,
                            c.to_string(),
                            None,
                            self.line,
                        ));
                    }
                }

                '<' => {
                    if self.peak() == '=' {
                        self.tokens.push(Token::new(
                            TokenKind::LESS_EQUAL,
                            format!("{}=", c),
                            None,
                            self.line,
                        ));
                        self.advance();
                    } else {
                        self.tokens.push(Token::new(
                            TokenKind::LESS,
                            c.to_string(),
                            None,
                            self.line,
                        ));
                    }
                }

                '>' => {
                    if self.peak() == '=' {
                        self.tokens.push(Token::new(
                            TokenKind::GREATER_EQUAL,
                            format!("{}=", c),
                            None,
                            self.line,
                        ));
                        self.advance();
                    } else {
                        self.tokens.push(Token::new(
                            TokenKind::GREATER,
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
                    } else {
                        self.tokens.push(Token::new(
                            TokenKind::SLASH,
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
            .push(Token::new(TokenKind::EOF, ' '.to_string(), None, self.line));
        (self.tokens.clone(), self.errors.clone())
    }
}
