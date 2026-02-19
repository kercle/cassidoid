use crate::parser::ast::ParserAst;

mod latex;

pub trait MathDisplay {
    fn to_latex(&self) -> String;
}

impl<A: Clone + PartialEq> MathDisplay for ParserAst<A> {
    fn to_latex(&self) -> String {
        latex::ast_to_latex(self, None)
    }
}
