use std::fmt::Debug;

use crate::{
    expr::{Expr, NormalizedExpr},
    matcher::{CommutativePredicate, Matcher, context::MatchContext},
};

pub type RuleTransformer<A> = Box<dyn Fn(&mut MatchContext<'_, A>) -> Expr<A> + Send + Sync>;

pub struct Rule<A>
where
    A: Clone + PartialEq,
{
    pub matcher: Matcher<A>,
    pub transform: RuleTransformer<A>,
}

#[derive(Default)]
pub struct Rewriter<A>
where
    A: Clone + PartialEq,
{
    rules: Vec<Rule<A>>,
    is_commutative: Option<CommutativePredicate<A>>,
}

impl<A> Rewriter<A>
where
    A: Clone + PartialEq + Default,
{
    pub fn new() -> Self {
        Self::default()
    }
}

impl<A> Rewriter<A>
where
    A: Clone + PartialEq + Default + Debug,
{
    pub fn commutative_if<F>(mut self, f: F) -> Self
    where
        F: Fn(&Expr<A>) -> bool + 'static,
    {
        debug_assert!(
            self.rules.is_empty(),
            "call commutative_if() before adding rules"
        );
        self.is_commutative = Some(CommutativePredicate::new(f));
        self
    }

    pub fn with_rule<F>(mut self, pattern: NormalizedExpr<A>, transform: F) -> Self
    where
        F: Fn(&mut MatchContext<'_, A>) -> Expr<A> + Send + Sync + 'static,
    {
        let matcher = Matcher::new(pattern.take_expr())
            .with_commutative_predicate(self.is_commutative.clone());

        self.rules.push(Rule {
            matcher,
            transform: Box::new(transform),
        });
        self
    }

    pub fn with_rules<I, F>(mut self, rules: I) -> Self
    where
        I: IntoIterator<Item = (NormalizedExpr<A>, F)>,
        F: Fn(&mut MatchContext<'_, A>) -> Expr<A> + Send + Sync + 'static,
    {
        for (p, t) in rules {
            self = self.with_rule(p, t);
        }
        self
    }

    pub fn apply_first_match(&self, expr: Expr<A>) -> Expr<A> {
        expr.map_bottom_up(&|expr| {
            let mut res = expr;

            for rule in &self.rules {
                if let Some(mut ctx) = rule.matcher.first_match(&res) {
                    res = (rule.transform)(&mut ctx);
                    break; // "first match" semantics
                }
            }

            res
        })
    }
}
