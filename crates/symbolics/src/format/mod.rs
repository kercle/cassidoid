use crate::parser::ast::AstNode;

mod latex;
mod yasc;

pub trait MathDisplay {
    fn to_latex(&self) -> String;
    fn to_yasc(&self) -> String;
}

impl MathDisplay for AstNode {
    fn to_latex(&self) -> String {
        latex::ast_to_latex(self, None)
    }

    fn to_yasc(&self) -> String {
        yasc::ast_to_yasc(self, None)
    }
}
