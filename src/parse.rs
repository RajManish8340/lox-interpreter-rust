use core::fmt;

use crate::token::TokenKind::{self};

#[derive(Debug)]
pub(crate) struct AstParser {
    source: Box<Vec<TokenKind>>,
    parsed: Vec<String>,
}

impl fmt::Display for AstParser {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for item in &self.parsed {
            write!(f, "{} ", item)?;
        }
        Ok(())
    }
}

impl AstParser {
    pub(crate) fn new(tokens: Box<Vec<TokenKind>>, parsed: Vec<String>) -> Self {
        Self {
            source: tokens,
            parsed: parsed,
        }
    }

    pub(crate) fn parse(&mut self) -> Vec<String> {
        for item in &*self.source {
            match item {
                TokenKind::Nil => self.parsed.push(String::from("nil")),
                TokenKind::False => self.parsed.push(String::from("false")),
                TokenKind::True => self.parsed.push(String::from("true")),
                _ => continue,
            };
        }
        self.parsed.clone()
    }

    pub(crate) fn print(&self) {
        println!("{}", self)
    }
}
