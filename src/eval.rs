use crate::ast::{Expr, Literal};

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
        Expr::Unary { .. } => todo!(),
        Expr::Binary { .. } => todo!(),
        Expr::Group { .. } => todo!(),
    }
}
