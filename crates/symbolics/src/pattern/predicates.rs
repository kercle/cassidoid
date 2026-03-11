use numbers::Number;

use crate::{
    atom::Atom,
    builtin::{ADD_HEAD, MUL_HEAD},
    expr::{Expr, ExprKind, NormExpr},
};

pub fn is_number<S>(expr: &Expr<S>) -> bool {
    expr.is_number()
}

pub fn is_symbol<S>(expr: &Expr<S>) -> bool {
    expr.is_symbol()
}

pub fn is_integer<S>(expr: &Expr<S>) -> bool {
    matches!(
        expr.kind(),
        ExprKind::Atom {
            entry: Atom::Number(Number::Integer(_))
        }
    )
}

pub fn is_rational<S>(expr: &Expr<S>) -> bool {
    matches!(
        expr.kind(),
        ExprKind::Atom {
            entry: Atom::Number(Number::Rational(_))
        }
    )
}

pub fn is_positive<S>(expr: &Expr<S>) -> bool {
    expr.is_number_positive()
}

pub fn is_negative<S>(expr: &Expr<S>) -> bool {
    expr.is_number_negative()
}
