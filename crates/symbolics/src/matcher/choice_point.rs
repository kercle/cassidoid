use crate::{expr::Expr, matcher::pattern_span::PatSpan};

pub struct ChoicePointOrderedSplit<'a, A> {
    pub todo_len: usize,
    pub undo_len: usize,

    pub seq_name: Option<&'a str>,

    pub k_min: usize,
    pub k_next: usize,

    pub rest_pats: PatSpan<'a, A>,
    pub rest_exprs: &'a [Expr<A>],
}

impl<'a, A> ChoicePointUnorderedSplit<'a, A> {
    pub fn to_choice_point(self) -> ChoicePoint<'a, A> {
        ChoicePoint::UnorderedSplit(self)
    }
}

pub struct ChoicePointUnorderedSplit<'a, A> {
    pub todo_len: usize,
    pub undo_len: usize,

    pub seq_name: Option<&'a str>,

    pub k_min: usize,
    pub k_next: usize,

    pub rest_pats: PatSpan<'a, A>,
    pub rest_exprs: &'a [Expr<A>],
}

impl<'a, A> ChoicePointOrderedSplit<'a, A> {
    pub fn to_choice_point(self) -> ChoicePoint<'a, A> {
        ChoicePoint::OrderedSplit(self)
    }
}

pub enum ChoicePoint<'a, A> {
    OrderedSplit(ChoicePointOrderedSplit<'a, A>),
    UnorderedSplit(ChoicePointUnorderedSplit<'a, A>),
}
