use crate::parser::ast::AstNode;

mod latex;

pub trait MathDisplay {
    fn to_latex(&self) -> String;
}

impl<A: Clone + PartialEq> MathDisplay for AstNode<A> {
    fn to_latex(&self) -> String {
        latex::ast_to_latex(self, None)
    }
}
