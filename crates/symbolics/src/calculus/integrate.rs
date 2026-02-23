use expr_macro::{expr, norm_expr};

use crate::{
    atom::Atom,
    expr::{Expr, NormalizedExpr},
    matcher::context::MatchContext,
    rewrite::Rewriter,
};

pub fn resolve_indefinite_integrals<A>(expr: Expr<A>) -> Expr
where
    A: Default + Clone + PartialEq,
{
    let rules = indefinite_integrals_rules();
    let rw = Rewriter::new().with_rules(rules.into_iter().map(|(pat, repl)| {
        (pat, move |ctx: &mut MatchContext<'_, ()>| {
            ctx.fill(repl.clone())
        })
    }));

    let mut expr = expr.drop_annotation();

    loop {
        let expr_next_iter = rw.apply_first_match(expr.clone());

        if expr != expr_next_iter {
            expr = expr_next_iter;
        } else {
            return expr;
        }
    }
}

fn indefinite_integrals_rules() -> Vec<(NormalizedExpr, Expr)> {
    vec![(
        norm_expr!(
        Integrate[
            Pattern[a, Blank[]] + Pattern[r, BlankSeq[]],
            PatternTest[Pattern[x, Blank[]], IsSymbolQ]
        ]),
        expr!(
        Integrate[a,x] + Integrate[Add[r],x]
        ),
    )]
}
