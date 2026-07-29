use crate::{
    ast::{
        Expr,
        Literal::{self},
    },
    token::{Token, TokenKind},
};

#[derive(Debug, Clone)]
pub(crate) struct AstParser {
    tokens: Vec<Token>,
    current: usize,
}

impl AstParser {
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
    pub(crate) fn primary(&mut self) -> Option<Expr> {
        let kind = &self.tokens[self.current].kind;
        match kind {
            TokenKind::True => {
                self.advance();
                return Some(Expr::Literal {
                    value: Literal::Bool(true),
                });
            }
            TokenKind::False => {
                self.advance();
                return Some(Expr::Literal {
                    value: Literal::Bool(false),
                });
            }
            TokenKind::Nil => {
                self.advance();
                return Some(Expr::Literal {
                    value: Literal::Nil,
                });
            }
            _ => None,
        }
    }
}
