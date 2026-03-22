use crate::{
    builtins::{
        BuiltInCategory,
        traits::{BuiltIn, BuiltInDoc, PatternDoc},
    },
    raw_expr,
};

#[derive(Default)]
pub struct Factorial;

impl Factorial {
    pub const HEAD: &'static str = "Factorial";
}

impl BuiltIn for Factorial {
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
            summary: "Represents the factorial.",
            pattern_doc: vec![PatternDoc::new(
                raw_expr!(Factorial[x_]),
                "Represents the factorial of the given expression.",
            )],
            examples: vec![],
            related: vec![],
        }
    }
}
