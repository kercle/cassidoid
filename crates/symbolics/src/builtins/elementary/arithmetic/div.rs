use crate::{
    builtins::{
        BuiltInCategory,
        traits::{BuiltIn, BuiltInDoc, PatternDoc},
    },
    raw_expr,
};

#[derive(Default)]
pub struct Div;

impl BuiltIn for Div {
    #[inline(always)]
    fn head() -> &'static str {
        "Div"
    }

    fn head_dyn(&self) -> &'static str {
        Self::head()
    }

    fn doc(&self) -> BuiltInDoc {
        BuiltInDoc {
            category: BuiltInCategory::ElementaryArithmetic,
            title: "Div",
            summary: "Internal representation of the quotient of two terms.",
            pattern_doc: vec![PatternDoc::new(
                raw_expr!(Div[a_, b_]),
                "Syntactic sugar for Mul[a, Pow[b, -1]].",
            )],
            examples: vec![],
            related: vec!["Div", "Mul", "Div"],
        }
    }
}
