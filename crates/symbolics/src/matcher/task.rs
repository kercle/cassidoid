use crate::{expr::Expr, matcher::pattern_span::PatSpan, pattern::Pattern};

pub enum Task<'a, A> {
    MatchOne {
        pattern: Pattern<'a, A>,
        expr: &'a Expr<A>,
    },
    MatchSeq {
        patterns: PatSpan<'a, A>,
        exprs: &'a [Expr<A>],
    },
    ResumeOrderedSplit {
        seq_name: Option<&'a str>,
        k_min: usize,
        k: usize,
        rest_pats: PatSpan<'a, A>,
        rest_exprs: &'a [Expr<A>],
    },
}

pub struct ChoicePoint<'a, A> {
    pub todo_len: usize,
    pub undo_len: usize,
    pub resume: Task<'a, A>,
}
