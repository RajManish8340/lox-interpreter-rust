use crate::tokens::TokenKind;

// TODO:: Equal (assignment), And/Or (logical) — added when statements/control-flow are implemented
#[derive(Debug)]
pub(crate) enum BinaryOp {
    Plus,
    Minus,
    Star,
    Slash,
    EqualEqual,
    BangEqual,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,
}

impl BinaryOp {
    pub(crate) fn from_token_kind(kind: &TokenKind) -> Option<Self> {
        match kind {
            TokenKind::Plus => Some(Self::Plus),
            TokenKind::Minus => Some(Self::Minus),
            TokenKind::Star => Some(Self::Star),
            TokenKind::Slash => Some(Self::Slash),
            TokenKind::EqualEqual => Some(Self::EqualEqual),
            TokenKind::BangEqual => Some(Self::BangEqual),
            TokenKind::Less => Some(Self::Less),
            TokenKind::LessEqual => Some(Self::LessEqual),
            TokenKind::Greater => Some(Self::Greater),
            TokenKind::GreaterEqual => Some(Self::GreaterEqual),
            _ => None,
        }
    }

    fn dump(&self) -> String {
        match self {
            Self::Plus => String::from("+"),
            Self::Minus => String::from("-"),
            Self::Star => String::from("*"),
            Self::Slash => String::from("/"),
            Self::EqualEqual => String::from("=="),
            Self::BangEqual => String::from("!="),
            Self::Less => String::from("<"),
            Self::LessEqual => String::from("<="),
            Self::Greater => String::from(">"),
            Self::GreaterEqual => String::from(">="),
        }
    }
}

#[derive(Debug)]
pub(crate) enum Literal {
    Bool(bool),
    String(String),
    Number(f64),
    Nil,
}

#[derive(Debug)]
pub(crate) enum UnaryOp {
    Bang,
    Minus,
}

impl UnaryOp {
    pub(crate) fn dump(&self) -> String {
        match self {
            Self::Bang => String::from("!"),
            Self::Minus => String::from("-"),
        }
    }
}

#[derive(Debug)]
pub(crate) enum Expr {
    Literal {
        value: Literal,
    },
    Unary {
        op: UnaryOp,
        expr: Box<Expr>,
    },
    Group {
        expr: Box<Expr>,
    },
    Binary {
        op: BinaryOp,
        lhs_expr: Box<Expr>,
        rhs_expr: Box<Expr>,
    },
}
