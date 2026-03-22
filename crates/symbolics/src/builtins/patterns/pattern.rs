use crate::{
    builtins::{
        BuiltInCategory,
        traits::{BuiltIn, BuiltInDoc, PatternDoc},
    },
    raw_expr,
};

#[derive(Default)]
pub struct Pattern;

impl Pattern {
    pub const HEAD: &'static str = "Pattern";
}

impl BuiltIn for Pattern {
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
            summary: "Binds a matched subject to a given symbol.",
            pattern_doc: vec![PatternDoc::new(
                raw_expr!(Pattern[s_?IsSymbol, p_]),
                "If the pattern is $p$ matches a given subject, the subject is bound to the symbol $s$.",
            )],
            examples: vec![],
            related: vec![],
        }
    }
}
