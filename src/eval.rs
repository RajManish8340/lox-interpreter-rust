use crate::ast::{
    Expr,
    Literal::{self},
    UnaryOp,
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
        Expr::Binary { .. } => todo!(),
        Expr::Group { expr } => evaluate(expr),
    }
}
