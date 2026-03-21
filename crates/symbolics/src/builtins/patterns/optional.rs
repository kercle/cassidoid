use crate::builtins::{
    BuiltInCategory,
    traits::{BuiltIn, BuiltInDoc, PatternDoc},
};

pub const OPTIONAL_HEAD: &str = "Optional";

#[derive(Default)]
pub struct Optional;

impl BuiltIn for Optional {
    fn doc(&self) -> BuiltInDoc {
        BuiltInDoc {
            category: BuiltInCategory::Patterns,
            title: self.head_symbol(),
            summary: "Holding patterns without normalizing them.",
            pattern_doc: vec![PatternDoc::new(
                "HoldPattern[p_]",
                "The pattern $p$ is not evaluated until the pattern is being used.",
            )],
            examples: vec![],
            related: vec![],
        }
    }

    fn head_symbol(&self) -> &'static str {
        OPTIONAL_HEAD
    }
}
