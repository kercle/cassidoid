use crate::{
    builtins::{
        BuiltInCategory,
        traits::{BuiltIn, BuiltInDoc, PatternDoc},
    },
    raw_expr,
};

#[derive(Default)]
pub struct Neg;

impl Neg {
    pub const HEAD: &'static str = "Neg";
}

impl BuiltIn for Neg {
    #[inline(always)]
    fn head() -> &'static str {
        Self::HEAD
    }

    fn head_dyn(&self) -> &'static str {
        Self::head()
    }

    fn doc(&self) -> BuiltInDoc {
        BuiltInDoc {
            category: BuiltInCategory::ElementaryArithmetic,
            title: Self::head(),
            summary: "Internal representation of the negation.",
            pattern_doc: vec![PatternDoc::new(
                raw_expr!(Neg[t_]),
                "This expression represents $-t$.",
            )],
            examples: vec![],
            related: vec!["Sub", "Mul"],
        }
    }
}
