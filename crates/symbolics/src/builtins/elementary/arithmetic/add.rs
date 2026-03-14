use crate::builtins::traits::{BuiltIn, BuiltInDoc, PatternDoc};

#[derive(Default)]
pub struct Add;

impl BuiltIn for Add {
    fn doc(&self) -> BuiltInDoc {
        BuiltInDoc {
            category: "Elementary arithmetic",
            title: "Add",
            summary: "Internal representation of the sum of terms.",
            pattern_doc: vec![
                PatternDoc::new("Add[]", "Reduces to zero."),
                PatternDoc::new(
                    "Add[t__]",
                    "If $t=[t_1,\\dots,t_n]$, this expression represents $t_1+\\cdots+t_n$.",
                ),
            ],
            examples: vec![],
            related: vec!["Sub", "Mul", "Div"],
        }
    }

    fn head_symbol(&self) -> &'static str {
        "Add"
    }
}
