use crate::{
    builtins::{
        BuiltInCategory,
        traits::{BuiltIn, BuiltInDoc, PatternDoc},
    },
    raw_expr,
};

#[derive(Default)]
pub struct Condition;

impl Condition {
    pub const HEAD: &'static str = "Condition";
}

impl BuiltIn for Condition {
    #[inline(always)]
    fn head() -> &'static str {
        Self::HEAD
    }

    fn head_dyn(&self) -> &'static str {
        Self::head()
    }

    fn doc(&self) -> BuiltInDoc {
        BuiltInDoc {
            category: BuiltInCategory::Patterns,
            title: Self::head(),
            summary: "Tests the given condition if the pattern matches.",
            pattern_doc: vec![PatternDoc::new(
                raw_expr!(Condition[p_, t_]),
                "If the pattern is $p$ matches a given subject, the test $t$ is executated. The entire pattern matches iff $t$ evaluates to True.",
            )],
            examples: vec![],
            related: vec![],
        }
    }
}
