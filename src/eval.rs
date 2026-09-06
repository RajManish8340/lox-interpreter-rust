use crate::{
    ast::{
        BinaryOp, Expr,
        Literal::{self},
        UnaryOp,
    },
    token::LiteralType,
};

pub(crate) fn print_literal(lit: &Literal) -> String {
    match lit {
        Literal::Bool(b) => b.to_string(),
        Literal::String(s) => s.to_string(),
        Literal::Number(n) => {
            format!("{}", n)
        }
        Literal::Nil => "nil".to_string(),
    }
}

pub(crate) fn evaluate(expr: &Expr) -> Literal {
    match expr {
        Expr::Literal { value } => value.clone(),

        Expr::Unary { op, expr } => {
            let lit: Literal = evaluate(expr);
            if op == &UnaryOp::Bang {
                match lit {
                    Literal::Bool(b) => Literal::Bool(!b),
                    Literal::Number(..) => Literal::Bool(false),
                    _ => unreachable!(),
                }
            } else {
                match lit {
                    Literal::Number(n) => Literal::Number(-n),
                    _ => unreachable!(),
                }
            }
        }

        Expr::Binary {
            op,
            lhs_expr,
            rhs_expr,
        } => {
            let lhs_literal = evaluate(lhs_expr);
            let rhs_literal = evaluate(rhs_expr);

            match op {
                BinaryOp::Slash => match (lhs_literal, rhs_literal) {
                    (Literal::Number(l), Literal::Number(r)) => Literal::Number(l / r),
                    _ => unreachable!(),
                },

                BinaryOp::Star => match (lhs_literal, rhs_literal) {
                    (Literal::Number(l), Literal::Number(r)) => Literal::Number(l * r),
                    _ => unreachable!(),
                },
                BinaryOp::Plus => match (lhs_literal, rhs_literal) {
                    (Literal::Number(l), Literal::Number(r)) => Literal::Number(l + r),
                    (Literal::String(l), Literal::String(r)) => {
                        let concat = format!("{}{}", l, r);
                        Literal::String(concat)
                    }
                    _ => unreachable!(),
                },
                BinaryOp::Minus => match (lhs_literal, rhs_literal) {
                    (Literal::Number(l), Literal::Number(r)) => Literal::Number(l - r),
                    _ => unreachable!(),
                },
                _ => unreachable!(),
            }
        }

        Expr::Group { expr } => evaluate(expr),
    }
}
