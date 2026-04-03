use crate::expr::RawExpr;

mod input;
mod latex;
mod mathml;

pub trait MathDisplay {
    fn to_latex_form(&self) -> String;
    fn to_input_form(&self) -> String;
    fn to_mathml_form(&self) -> String;
}

impl MathDisplay for RawExpr {
    fn to_latex_form(&self) -> String {
        latex::render(self)
    }

    fn to_input_form(&self) -> String {
        input::render(self)
    }

    fn to_mathml_form(&self) -> String {
        mathml::render(self)
    }
}
