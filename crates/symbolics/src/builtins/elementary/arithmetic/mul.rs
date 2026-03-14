use crate::builtins::traits::{BuiltIn, BuiltInDoc, PatternDoc};

#[derive(Default)]
pub struct Mul;

impl BuiltIn for Mul {
    fn doc(&self) -> BuiltInDoc {
        BuiltInDoc {
            category: "Elementary arithmetic",
            title: "Mul",
            summary: "Internal representation of the product of terms.",
            pattern_doc: vec![
                PatternDoc::new("Mul[]", "Reduces to one."),
                PatternDoc::new(
                    "Mul[t__]",
                    "If $t=[t_1,\\dots,t_n]$, this expression represents $t_1\\times\\cdots\\times t_n$.",
                ),
            ],
            examples: vec![],
            related: vec!["Sub", "Mul", "Div"],
        }
    }

    fn head_symbol(&self) -> &'static str {
        "Mul"
    }
}
