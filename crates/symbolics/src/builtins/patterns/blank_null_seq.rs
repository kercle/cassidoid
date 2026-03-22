use crate::{
    builtins::{
        BuiltInCategory,
        traits::{BuiltIn, BuiltInDoc, PatternDoc},
    },
    raw_expr,
};

#[derive(Default)]
pub struct BlankNullSeq;

impl BlankNullSeq {
    pub const HEAD: &'static str = "BlankNullSeq";
}

impl BuiltIn for BlankNullSeq {
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
            summary: "Matches any sub-sequence of subset in node arguments. The sub-sequences or subsets may be empty.",
            pattern_doc: vec![
                PatternDoc::new(
                    raw_expr!(BlankNullSeq[]),
                    "Matches any sub-sequence of subset in node arguments.",
                ),
                PatternDoc::new(
                    raw_expr!(BlankNullSeq[h_?IsSymbol]),
                    "Matches any sub-sequence of subset in node arguments with head $h$.",
                ),
            ],
            examples: vec![],
            related: vec![],
        }
    }
}
