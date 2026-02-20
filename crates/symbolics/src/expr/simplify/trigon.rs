use expr_macro::{norm_expr, raw_expr};

use crate::expr::{Expr, NormalizedExpr, atom::Atom, matcher::MatchIter, pattern::Pattern};

pub fn replace_symbols(expr: Expr, s: &str, replacement: &Expr) -> Expr {
    expr.map_bottom_up(&|e| {
        if e.matches_symbol(s) {
            replacement.clone()
        } else {
            e
        }
    })
}

// TODO: implement proper rewrite engine
pub fn replace_all_quick_and_dirty(expr: Expr, norm_pattern_expr: NormalizedExpr, replacement: Expr) -> Expr {
    let pattern_expr = norm_pattern_expr.take_expr();
    let pattern = Pattern::from_expr(&pattern_expr);

    dbg!(&pattern_expr);
    dbg!(&expr);

    expr.map_bottom_up(&|e| {
        if let Some(ctx) = MatchIter::new(&e, &pattern).next() {
            let mut new_term = replacement.clone();
            for (k, v) in ctx.iter() {
                new_term = replace_symbols(new_term, k, v.get_expr());
            }
            new_term
        } else {
            e
        }
    })
}

pub fn simplify_trigon(expr: Expr) -> Expr {
    let expr = replace_all_quick_and_dirty(
        expr,
        norm_expr! { Sqrt[1 - Cos[Pattern[x, Blank[]]]^2] },
        raw_expr! { Sin[x]^2 },
    );

    let expr = replace_all_quick_and_dirty(
        expr,
        norm_expr! { Sqrt[1 - Sin[Pattern[x, Blank[]]]^2] },
        raw_expr! { Cos[x]^2 },
    );

    expr
}
