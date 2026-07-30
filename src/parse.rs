use crate::{
    ast::{
        Expr,
        Literal::{self},
    },
    token::{Token, TokenKind},
};

pub(crate) fn print_expr(expr: &Expr) -> String {
    match expr {
        Expr::Literal { value } => match value {
            Literal::Bool(b) => b.to_string(),
            Literal::String(s) => s.to_string(),
            Literal::Number(n) => n.to_string(),
            Literal::Nil => "nil".to_string(),
        },
        //TODO: rest of the match arms
        _ => "".to_owned(),
    }
}
#[derive(Debug, Clone)]
pub(crate) struct AstParser {
    tokens: Vec<Token>,
    current: usize,
}

impl AstParser {
    pub(crate) fn new(tokens: Vec<Token>, current: usize) -> Self {
        Self { tokens, current }
    }
    pub(crate) fn is_at_end(&self) -> bool {
        if self.tokens[self.current].kind == TokenKind::Eof {
            return true;
        }
        false
    }

    pub(crate) fn peak(&self) -> &Token {
        &self.tokens[self.current]
    }

    pub(crate) fn advance(&mut self) -> &Token {
        let prev = self.current;
        self.current += 1;
        &self.tokens[prev]
    }

    pub(crate) fn primary(&mut self) -> Result<Expr, String> {
        let kind = &self.tokens[self.current].kind;
        match kind {
            TokenKind::True => {
                self.advance();
                return Ok(Expr::Literal {
                    value: Literal::Bool(true),
                });
            }
            TokenKind::False => {
                self.advance();
                return Ok(Expr::Literal {
                    value: Literal::Bool(false),
                });
            }
            TokenKind::Nil => {
                self.advance();
                return Ok(Expr::Literal {
                    value: Literal::Nil,
                });
            }
            _ => Err("Unexpected token while parsing primary".to_owned()),
        }
    }
}
