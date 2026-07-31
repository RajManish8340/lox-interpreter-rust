use crate::{
    ast::{
        Expr,
        Literal::{self},
    },
    token::{LiteralType, Token, TokenKind},
};

pub(crate) fn print_expr(expr: &Expr) -> String {
    match expr {
        Expr::Literal { value } => match value {
            Literal::Bool(b) => b.to_string(),
            Literal::String(s) => s.to_string(),
            Literal::Number(n) => {
                if n.fract() == 0.0 {
                    format!("{:.1}", n)
                } else {
                    format!("{}", n)
                }
            }
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

    pub(crate) fn return_prev_than_advance(&mut self) -> &Token {
        let prev = self.current;
        self.current += 1;
        &self.tokens[prev]
    }

    pub(crate) fn primary(&mut self) -> Result<Expr, String> {
        let kind = &self.tokens[self.current].kind;

        match kind {
            TokenKind::True => {
                self.return_prev_than_advance();
                return Ok(Expr::Literal {
                    value: Literal::Bool(true),
                });
            }

            TokenKind::False => {
                self.return_prev_than_advance();
                return Ok(Expr::Literal {
                    value: Literal::Bool(false),
                });
            }

            TokenKind::Nil => {
                self.return_prev_than_advance();
                return Ok(Expr::Literal {
                    value: Literal::Nil,
                });
            }

            TokenKind::Number => {
                let token = self.return_prev_than_advance();
                match &token.literal {
                    Some(LiteralType::Number(n)) => Ok(Expr::Literal {
                        value: Literal::Number(*n),
                    }),
                    _ => Err("Expected Number Literal while parsing tokens".to_owned()),
                }
            }

            TokenKind::String => {
                let token = self.return_prev_than_advance();
                match &token.literal {
                    Some(LiteralType::String(s)) => Ok(Expr::Literal {
                        value: Literal::String(s.to_owned()),
                    }),
                    _ => Err("Expected String Literal while parsing tokens".to_owned()),
                }
            }
            _ => Err("Unexpected token while parsing primary".to_owned()),
        }
    }
}
