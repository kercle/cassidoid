use numbers::Number;

use crate::{expr::NormExpr, simplify::Simplifier};

#[derive(Clone)]
pub struct UnivariatePolynomial(Vec<Number>);

impl UnivariatePolynomial {
    pub fn from_expr(expr: &NormExpr) -> Option<Self> {
        let expr = Simplifier::new(expr.clone()).with_expansion();
        todo!()
    }

    pub fn zero() -> Self {
        Self(vec![Number::zero()])
    }

    pub fn degree(&self) -> usize {
        self.0.len() - 1
    }

    pub fn coeff(&self, i: usize) -> &Number {
        self.0.get(i).unwrap_or(&numbers::ZERO)
    }

    pub fn long_division(&self, other: &Self) -> (Self, Self) {
        if other.degree() > self.degree() {
            return (UnivariatePolynomial::zero(), other.clone());
        }
        todo!()
    }
}
