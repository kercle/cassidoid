use crate::{
    builtins::{
        BuiltInCategory,
        traits::{BuiltIn, BuiltInDoc, PatternDoc},
    },
    raw_expr,
};

#[derive(Default)]
pub struct Sub;

impl BuiltIn for Sub {
    #[inline(always)]
    fn head() -> &'static str {
        "Sub"
    }

    fn head_dyn(&self) -> &'static str {
        Self::head()
    }

    fn doc(&self) -> BuiltInDoc {
        BuiltInDoc {
            category: BuiltInCategory::ElementaryArithmetic,
            title: Self::head(),
            summary: "Internal representation of the difference of two terms.",
            pattern_doc: vec![PatternDoc::new(
                raw_expr!(Sub[a_, b_]),
                "Syntactic sugar for Add[a, Mul[-1, b]].",
            )],
            examples: vec![],
            related: vec!["Add", "Mul", "Div"],
        }
    }
}
