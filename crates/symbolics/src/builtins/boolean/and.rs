use crate::{
    builtins::{
        BuiltInCategory,
        traits::{ApplicationError, BuiltIn, BuiltInDoc, PatternDoc},
    },
    ensure,
    expr::Expr,
    raw_expr,
};

#[derive(Default)]
pub struct And;

impl And {
    pub const HEAD: &'static str = "And";
}

impl BuiltIn for And {
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
            summary: "Internal representation of the boolean And operation.",
            pattern_doc: vec![
                PatternDoc::new(raw_expr!(And[]), "Reduces to True."),
                PatternDoc::new(
                    raw_expr!(And[b__]),
                    "If $b=[b_1,\\dots,b_n]$, this expression represents $b_1\\wedge\\cdots\\wedge b_n$.",
                ),
            ],
            examples: vec![],
            related: vec![],
        }
    }

    fn validate_application_of<S>(
        head: &Expr<S>,
        _children: &[Expr<S>],
    ) -> Result<(), ApplicationError> {
        ensure!(
            head.matches_symbol(Self::head()),
            ApplicationError::HeadMismatch
        );
        Ok(())
    }
}
