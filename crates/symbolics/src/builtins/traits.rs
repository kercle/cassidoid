use crate::{
    builtins::BuiltInCategory,
    expr::{NormExpr, RawExpr},
};

#[derive(Clone, Debug)]
pub struct BuiltInDoc {
    pub category: BuiltInCategory,
    pub title: &'static str,
    pub summary: &'static str,
    pub pattern_doc: Vec<PatternDoc>,
    pub examples: Vec<(&'static str, &'static str)>,
    pub related: Vec<&'static str>,
}

#[derive(Clone, Debug)]
pub struct PatternDoc {
    pub pattern: RawExpr,
    pub summary: String,
}

impl PatternDoc {
    pub fn new<T: ToString>(pattern: RawExpr, summary: T) -> Self {
        Self {
            pattern,
            summary: summary.to_string(),
        }
    }
}

pub trait BuiltIn {
    fn doc(&self) -> BuiltInDoc;

    fn apply_all(&self, expr: NormExpr) -> NormExpr {
        expr
    }

    fn head() -> &'static str
    where
        Self: Sized;

    fn head_dyn(&self) -> &'static str;
}
