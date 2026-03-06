use expr_macro::{expr, norm_expr};

use crate::{
    atom::Atom,
    expr::{Expr, NormalizedExpr},
};

pub fn trigonometric_rules() -> Vec<(NormalizedExpr, Expr)> {
    vec![
        // =============== Sin exact values ===============
        (norm_expr!(Sin[pi / 12]), expr!(Sqrt[2] * (Sqrt[3] - 1) / 4)),
        (norm_expr!(Sin[pi / 10]), expr!((Sqrt[5] - 1) / 4)),
        (norm_expr!(Sin[pi / 8]), expr!(Sqrt[2 - Sqrt[2]] / 2)),
        (norm_expr!(Sin[pi / 6]), expr!(1 / 2)),
        (
            norm_expr!(Sin[pi / 5]),
            expr!(Sqrt[2] * Sqrt[5 - Sqrt[5]] / 4),
        ),
        (norm_expr!(Sin[pi / 4]), expr!(1 / Sqrt[2])),
        // =============== Cos exact values ===============
        (norm_expr!(Cos[pi / 12]), expr!(Sqrt[2] * (Sqrt[3] + 1) / 4)),
        (
            norm_expr!(Cos[pi / 10]),
            expr!(Sqrt[2] * Sqrt[5 + Sqrt[5]] / 4),
        ),
        (norm_expr!(Cos[pi / 8]), expr!(Sqrt[2 + Sqrt[2]] / 2)),
        (norm_expr!(Cos[pi / 6]), expr!(Sqrt[3] / 2)),
        (norm_expr!(Cos[pi / 5]), expr!((Sqrt[5] + 1) / 4)),
        (norm_expr!(Cos[pi / 4]), expr!(1 / Sqrt[2])),
        // =============== Pythagorean identity ===============
        (
            norm_expr!(Cos[Pattern[a, Blank[]]]^2 + Sin[Pattern[a, Blank[]]]^2 + Pattern[rest, BlankNullSeq[]]),
            expr!(1 + rest),
        ),
        (
            norm_expr!(Sqrt[1 - Cos[Pattern[x, Blank[]]] ^ 2]),
            expr!(Sin[x]),
        ),
        (
            norm_expr!(Sqrt[1 - Sin[Pattern[x, Blank[]]] ^ 2]),
            expr!(Cos[x]),
        ),
    ]
}
