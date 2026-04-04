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
pub struct Or;

impl Or {
    pub const HEAD: &'static str = "Or";
}

impl BuiltIn for Or {
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
            summary: "Internal representation of the boolean Or operation.",
            pattern_doc: vec![
                PatternDoc::new(raw_expr!(Or[]), "Reduces to False."),
                PatternDoc::new(
                    raw_expr!(Or[b__]),
                    "If $b=[b_1,\\dots,b_n]$, this expression represents $b_1\\vee\\cdots\\vee b_n$.",
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
