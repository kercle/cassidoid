use crate::builtins::{
    BuiltInCategory,
    traits::{BuiltIn, BuiltInDoc, PatternDoc},
};

pub const FACTORIAL_HEAD: &str = "Factorial";

#[derive(Default)]
pub struct Factorial;

impl BuiltIn for Factorial {
    fn doc(&self) -> BuiltInDoc {
        BuiltInDoc {
            category: BuiltInCategory::Patterns,
            title: FACTORIAL_HEAD,
            summary: "Represents the factorial.",
            pattern_doc: vec![PatternDoc::new(
                "Factorial[x_]",
                "Represents the factorial of the given expression.",
            )],
            examples: vec![],
            related: vec![],
        }
    }

    fn head_symbol(&self) -> &'static str {
        FACTORIAL_HEAD
    }
}
