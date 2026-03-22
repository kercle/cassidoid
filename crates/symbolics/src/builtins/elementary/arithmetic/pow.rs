use crate::{
    builtins::{
        BuiltInCategory,
        traits::{BuiltIn, BuiltInDoc, PatternDoc},
    },
    raw_expr,
};

#[derive(Default)]
pub struct Pow;

impl BuiltIn for Pow {
    #[inline(always)]
    fn head() -> &'static str {
        "Pow"
    }

    fn head_dyn(&self) -> &'static str {
        Self::head()
    }

    fn doc(&self) -> BuiltInDoc {
        BuiltInDoc {
            category: BuiltInCategory::ElementaryArithmetic,
            title: Self::head(),
            summary: "Internal representation of the power of two terms. Contrary to Mathematica, Pow is only meaningful when having arity two.",
            pattern_doc: vec![
                PatternDoc::new(raw_expr!(Pow[x, Absent]), "Reduces to x."),
                PatternDoc::new(
                    raw_expr!(Pow[Absent, x]),
                    "Reduces to Pow[x], which loses its meaning as power.",
                ),
                PatternDoc::new(raw_expr!(Power[b_, e_]), "Represents $b^e$."),
            ],
            examples: vec![("Pow[x, Absent]", "x")],
            related: vec!["Add", "Sub", "Mul", "Div"],
        }
    }
}
