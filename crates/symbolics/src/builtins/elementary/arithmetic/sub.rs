use crate::builtins::{
    BuiltInCategory,
    traits::{BuiltIn, BuiltInDoc, PatternDoc},
};

#[derive(Default)]
pub struct Sub;

impl BuiltIn for Sub {
    fn doc(&self) -> BuiltInDoc {
        BuiltInDoc {
            category: BuiltInCategory::ElementaryArithmetic,
            title: "Sub",
            summary: "Internal representation of the difference of two terms.",
            pattern_doc: vec![PatternDoc::new(
                "Sub[a_, b_]",
                "Syntactic sugar for Add[a, Mul[-1, b]].",
            )],
            examples: vec![],
            related: vec!["Sub", "Mul", "Div"],
        }
    }

    fn head_symbol(&self) -> &'static str {
        "Sub"
    }
}
