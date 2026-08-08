use std::fmt::Alignment::Left;

use crate::{
    ast::{
        BinaryOp, Expr,
        Literal::{self},
        UnaryOp::{self},
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

        Expr::Unary { op, expr } => format!("({} {})", op.dump(), print_expr(expr)),

        Expr::Binary {
            op,
            lhs_expr,
            rhs_expr,
        } => format!(
            "({} {} {})",
            op.dump(),
            print_expr(lhs_expr),
            print_expr(rhs_expr)
        ),

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

    // primary → NUMBER | STRING | "true" | "false" | "nil"
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

            TokenKind::Number => {
                let token = self.advance();
                match &token.literal {
                    Some(LiteralType::Number(n)) => Ok(Expr::Literal {
                        value: Literal::Number(*n),
                    }),
                    _ => Err("Expected Number Literal while parsing tokens".to_owned()),
                }
            }

            TokenKind::String => {
                let token = self.advance();
                match &token.literal {
                    Some(LiteralType::String(s)) => Ok(Expr::Literal {
                        value: Literal::String(s.to_owned()),
                    }),
                    _ => Err("Expected String Literal while parsing tokens".to_owned()),
                }
            }

            _ => Err("Not a Primary token while parsing primary".to_owned()),
        }
    }

    // unary → ( "!" | "-" ) unary | primary
    pub(crate) fn unary(&mut self) -> Result<Expr, String> {
        match self.tokens[self.current].kind {
            TokenKind::Bang => {
                self.advance();

                // recursive unary after we find the bang and take the value only if it is Ok else
                // if it returns error do not take it and it will return whatever the error is .
                let inner = self.unary()?;
                Ok(Expr::Unary {
                    op: UnaryOp::Bang,
                    expr: Box::new(inner),
                })
            }
            TokenKind::Minus => {
                self.advance();

                // same as bang
                let inner = self.unary()?;
                Ok(Expr::Unary {
                    op: UnaryOp::Minus,
                    expr: Box::new(inner),
                })
            }

            // unary → ( "!" | "-" ) unary | primary ----> only alternative is the primary
            _ => self.primary(),
        }
    }

    pub(crate) fn factor(&mut self) -> Result<Expr, String> {
        let mut running_expression = self.unary()?;

        while self.tokens[self.current].kind == TokenKind::Slash
            || self.tokens[self.current].kind == TokenKind::Star
        {
            match self.tokens[self.current].kind {
                TokenKind::Star => {
                    self.advance();
                    let unary = self.unary()?;
                    running_expression = Expr::Binary {
                        op: BinaryOp::Star,
                        lhs_expr: Box::new(running_expression),
                        rhs_expr: Box::new(unary),
                    };
                }

                TokenKind::Slash => {
                    self.advance();
                    let unary = self.unary()?;
                    running_expression = Expr::Binary {
                        op: BinaryOp::Slash,
                        lhs_expr: Box::new(running_expression),
                        rhs_expr: Box::new(unary),
                    };
                }
                _ => return Err("not a slash or star in factor".to_owned()),
            }
        }
        Ok(running_expression)
    }

    pub(crate) fn term(&mut self) -> Result<Expr, String> {
        let mut running_expression = self.factor()?;

        while self.tokens[self.current].kind == TokenKind::Minus
            || self.tokens[self.current].kind == TokenKind::Plus
        {
            match self.tokens[self.current].kind {
                TokenKind::Minus => {
                    self.advance();
                    let factor = self.factor()?;
                    running_expression = Expr::Binary {
                        op: BinaryOp::Minus,
                        lhs_expr: Box::new(running_expression),
                        rhs_expr: Box::new(factor),
                    };
                }

                TokenKind::Plus => {
                    self.advance();
                    let factor = self.factor()?;
                    running_expression = Expr::Binary {
                        op: BinaryOp::Plus,
                        lhs_expr: Box::new(running_expression),
                        rhs_expr: Box::new(factor),
                    };
                }
                _ => return Err("not a minus or Plus in term".to_owned()),
            }
        }
        Ok(running_expression)
    }

    pub(crate) fn comparison(&mut self) -> Result<Expr, String> {
        let mut running_expression = self.term()?;

        while self.tokens[self.current].kind == TokenKind::Greater
            || self.tokens[self.current].kind == TokenKind::Less
            || self.tokens[self.current].kind == TokenKind::GreaterEqual
            || self.tokens[self.current].kind == TokenKind::LessEqual
        {
            match self.tokens[self.current].kind {
                TokenKind::Greater => {
                    self.advance();
                    let term = self.term()?;
                    running_expression = Expr::Binary {
                        op: BinaryOp::Greater,
                        lhs_expr: Box::new(running_expression),
                        rhs_expr: Box::new(term),
                    };
                }

                TokenKind::GreaterEqual => {
                    self.advance();
                    let term = self.term()?;
                    running_expression = Expr::Binary {
                        op: BinaryOp::GreaterEqual,
                        lhs_expr: Box::new(running_expression),
                        rhs_expr: Box::new(term),
                    };
                }

                TokenKind::Less => {
                    self.advance();
                    let term = self.term()?;
                    running_expression = Expr::Binary {
                        op: BinaryOp::Less,
                        lhs_expr: Box::new(running_expression),
                        rhs_expr: Box::new(term),
                    };
                }

                TokenKind::LessEqual => {
                    self.advance();
                    let term = self.term()?;
                    running_expression = Expr::Binary {
                        op: BinaryOp::LessEqual,
                        lhs_expr: Box::new(running_expression),
                        rhs_expr: Box::new(term),
                    };
                }
                _ => {
                    return Err(
                        "not a less, greater, greater_equal, less_equal in comparison".to_owned(),
                    );
                }
            }
        }
        Ok(running_expression)
    }
}
