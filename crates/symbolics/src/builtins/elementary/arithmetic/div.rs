use crate::builtins::{
    BuiltInCategory,
    traits::{BuiltIn, BuiltInDoc, PatternDoc},
};

#[derive(Default)]
pub struct Div;

impl BuiltIn for Div {
    fn doc(&self) -> BuiltInDoc {
        BuiltInDoc {
            category: BuiltInCategory::ElementaryArithmetic,
            title: "Div",
            summary: "Internal representation of the quotient of two terms.",
            pattern_doc: vec![PatternDoc::new(
                "Div[a_, b_]",
                "Syntactic sugar for Mul[a, Pow[b, -1]].",
            )],
            examples: vec![],
            related: vec!["Div", "Mul", "Div"],
        }
    }

    fn head_symbol(&self) -> &'static str {
        "Div"
    }
}
