use std::fmt::Display;

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

        Expr::Group { expr } => format!("(group {})", print_expr(expr)),
    }
}

pub(crate) struct ParsingError {
    token: Token,
    message: String,
}

impl Display for ParsingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.token.lexeme == "" {
            write!(
                f,
                "[line{}] {}, Found Token 'end of file'",
                self.token.line, self.message
            )
        } else {
            write!(
                f,
                "[line{}] {}, Found Token '{}'",
                self.token.line, self.message, self.token.lexeme,
            )
        }
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

    pub(crate) fn check(&self, kind: TokenKind) -> bool {
        if self.is_at_end() {
            return false;
        }
        return self.peak().kind == kind;
    }

    pub(crate) fn consume(
        &mut self,
        kind: TokenKind,
        message: String,
    ) -> Result<Token, ParsingError> {
        if self.check(kind) {
            return Ok(self.advance().clone());
        }
        return Err(ParsingError {
            token: self.peak().clone(),
            message,
        });
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
        let curr = self.current; // current token 
        self.current += 1; // advace
        &self.tokens[curr] // return the current token and advance
    }

    pub(crate) fn previous(&self) -> &Token {
        &self.tokens[self.current - 1]
    }

    // for later
    pub(crate) fn synchronize(&mut self) {
        self.advance();

        while !self.is_at_end() {
            if self.previous().kind == TokenKind::Semicolon {
                return;
            }

            match self.peak().kind {
                TokenKind::Class
                | TokenKind::Fun
                | TokenKind::Var
                | TokenKind::For
                | TokenKind::If
                | TokenKind::While
                | TokenKind::Print
                | TokenKind::Return => return,
                _ => {
                    self.advance();
                }
            }
        }
    }

    // primary → NUMBER | STRING | "true" | "false" | "nil"
    pub(crate) fn primary(&mut self) -> Result<Expr, ParsingError> {
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

            //TODO: Just to highlight that the code should not reach the error case in any of the
            // primary tokens only at the end when no primary token is found
            TokenKind::Number => {
                let token = self.advance();
                match &token.literal {
                    Some(LiteralType::Number(n)) => Ok(Expr::Literal {
                        value: Literal::Number(*n),
                    }),
                    _ => Err(ParsingError {
                        token: self.previous().clone(),
                        message: "Expected Number Literal while parsing tokens".to_owned(),
                    }),
                }
            }

            TokenKind::String => {
                let token = self.advance();
                match &token.literal {
                    Some(LiteralType::String(s)) => Ok(Expr::Literal {
                        value: Literal::String(s.to_owned()),
                    }),
                    _ => Err(ParsingError {
                        token: self.previous().clone(),
                        message: "Expected string Literal while parsing tokens".to_owned(),
                    }),
                }
            }

            TokenKind::LeftParen => {
                self.advance(); // advances after seeing (
                let inner = self.expression()?;
                // consume ) if it exists else returns next token
                self.consume(
                    TokenKind::RightParen,
                    "Expect ) after expression".to_owned(),
                )?;
                Ok(Expr::Group {
                    expr: Box::new(inner),
                })
            }

            _ => {
                return Err(ParsingError {
                    token: self.tokens[self.current].clone(),
                    message: "not a primary token while parsing primary".to_owned(),
                });
            }
        }
    }

    // unary → ( "!" | "-" ) unary | primary
    pub(crate) fn unary(&mut self) -> Result<Expr, ParsingError> {
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

    pub(crate) fn factor(&mut self) -> Result<Expr, ParsingError> {
        let mut running_expression = self.unary()?;

        while self.tokens[self.current].kind == TokenKind::Star
            || self.tokens[self.current].kind == TokenKind::Slash
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
                _ => {
                    panic!("code should not reach here");
                }
            }
        }
        Ok(running_expression)
    }

    pub(crate) fn term(&mut self) -> Result<Expr, ParsingError> {
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
                _ => {
                    panic!("code should not reach here in term")
                }
            }
        }
        Ok(running_expression)
    }

    pub(crate) fn comparison(&mut self) -> Result<Expr, ParsingError> {
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
                    panic!("code should not reach here in comparision")
                }
            }
        }
        Ok(running_expression)
    }

    pub(crate) fn equality(&mut self) -> Result<Expr, ParsingError> {
        let mut running_expression = self.comparison()?;

        while self.tokens[self.current].kind == TokenKind::BangEqual
            || self.tokens[self.current].kind == TokenKind::EqualEqual
        {
            match self.tokens[self.current].kind {
                TokenKind::BangEqual => {
                    self.advance();
                    let comparison = self.comparison()?;
                    running_expression = Expr::Binary {
                        op: BinaryOp::BangEqual,
                        lhs_expr: Box::new(running_expression),
                        rhs_expr: Box::new(comparison),
                    };
                }

                TokenKind::EqualEqual => {
                    self.advance();
                    let comparison = self.comparison()?;
                    running_expression = Expr::Binary {
                        op: BinaryOp::EqualEqual,
                        lhs_expr: Box::new(running_expression),
                        rhs_expr: Box::new(comparison),
                    };
                }
                _ => {
                    panic!("code should not reach here in equality")
                }
            }
        }
        Ok(running_expression)
    }

    pub(crate) fn expression(&mut self) -> Result<Expr, ParsingError> {
        self.equality()
    }
}
