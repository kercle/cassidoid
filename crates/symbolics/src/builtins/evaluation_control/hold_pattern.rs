use crate::{
    builtins::{
        BuiltInCategory,
        traits::{BuiltIn, BuiltInDoc, PatternDoc},
    },
    raw_expr,
};

#[derive(Default)]
pub struct HoldPattern;

impl HoldPattern {
    pub const HEAD: &'static str = "HoldPattern";
}

impl BuiltIn for HoldPattern {
    #[inline(always)]
    fn head() -> &'static str {
        Self::HEAD
    }

    fn head_dyn(&self) -> &'static str {
        Self::head()
    }

    fn doc(&self) -> BuiltInDoc {
        BuiltInDoc {
            category: BuiltInCategory::EvaluationControl,
            title: Self::head(),
            summary: "Holding patterns without normalizing them.",
            pattern_doc: vec![PatternDoc::new(
                raw_expr!(HoldPattern[p_]),
                "The pattern $p$ is not evaluated until the pattern is being used.",
            )],
            examples: vec![],
            related: vec![],
        }
    }
}
