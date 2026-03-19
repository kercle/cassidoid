use numbers::Number;

use crate::{
    atom::Atom,
    expr::pool::{ExprHandle, ExprPool, ExprView},
};

pub fn is_number<S: Copy + 'static>(pool: &ExprPool, expr: ExprHandle<S>) -> bool {
    expr.view(pool).get_number().is_some()
}

pub fn is_symbol<S: Copy + 'static>(pool: &ExprPool, expr: ExprHandle<S>) -> bool {
    expr.view(pool).get_symbol().is_some()
}

pub fn is_integer<S: Copy + 'static>(pool: &ExprPool, expr: ExprHandle<S>) -> bool {
    matches!(
        expr.view(pool),
        ExprView::Atom(Atom::Number(Number::Integer(_)))
    )
}

pub fn is_rational<S: Copy + 'static>(pool: &ExprPool, expr: ExprHandle<S>) -> bool {
    matches!(
        expr.view(pool),
        ExprView::Atom(Atom::Number(Number::Rational(_)))
    )
}

pub fn is_positive<S: Copy + 'static>(pool: &ExprPool, expr: ExprHandle<S>) -> bool {
    expr.view(pool)
        .get_number()
        .map(|n| n.is_positive())
        .unwrap_or(false)
}

pub fn is_negative<S: Copy + 'static>(pool: &ExprPool, expr: ExprHandle<S>) -> bool {
    expr.view(pool)
        .get_number()
        .map(|n| n.is_negative())
        .unwrap_or(false)
}
