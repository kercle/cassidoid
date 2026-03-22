use crate::{
    builtins::{
        BuiltInCategory,
        traits::{BuiltIn, BuiltInDoc, PatternDoc},
    },
    raw_expr,
};

#[derive(Default)]
pub struct Hold;

impl Hold {
    pub const HEAD: &'static str = "Hold";
}

impl BuiltIn for Hold {
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
            summary: "Holding expressions without normalizing them.",
            pattern_doc: vec![PatternDoc::new(
                raw_expr!(Hold[t_]),
                "The expression $t$ is not evaluated until it is released.",
            )],
            examples: vec![],
            related: vec![],
        }
    }
}
