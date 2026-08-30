use crate::ast::{Expr, Literal};

pub(crate) fn evaluate(expr: &Expr) -> Literal {
    match expr {
        Expr::Literal { value } => value.clone(),
        Expr::Unary { .. } => todo!(),
        Expr::Binary { .. } => todo!(),
        Expr::Group { .. } => todo!(),
    }
}
