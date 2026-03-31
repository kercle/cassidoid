use std::sync::LazyLock;

use numbers::{Number, ONE, ZERO};

use crate::{
    builtins::{self, traits::BuiltIn},
    ensure,
    expr::NormExpr,
    norm_expr,
    pattern::program::{Compiler, Program},
};

static MONOMIAL_PROG: LazyLock<Program> = LazyLock::new(|| {
    Compiler::new().compile(&norm_expr!(
        Optional[c_?IsNumber] * Optional[(x_?IsSymbol) ^ Optional[n_?IsPositive]]
    ))
});

#[derive(Clone, Debug)]
pub struct UnivariatePolynomial {
    coeff: Vec<Number>,
    symbol: String,
}

#[derive(Debug, Clone)]
pub enum LongDivisionError {
    SymbolMismatch,
}

impl UnivariatePolynomial {
    pub fn from_expr(expr: &NormExpr, sym: impl AsRef<str>) -> Option<Self> {
        let mut coeff;

        if builtins::Add::is_application(expr) {
            coeff = Vec::new();

            for t in expr.args()? {
                let (c, n) = Self::monimial_from_expr(t, &sym)?;

                while n < coeff.len() {
                    coeff.push(ZERO.clone());
                }

                if n == coeff.len() {
                    coeff.push(c.clone());
                } else {
                    coeff[n] += c.clone();
                }
            }
        } else {
            let (c, n) = Self::monimial_from_expr(expr, &sym)?;

            coeff = vec![ZERO.clone(); n + 1];
            coeff[n] = c.clone();
        }

        Some(UnivariatePolynomial {
            coeff,
            symbol: sym.as_ref().to_string(),
        })
    }

    fn monimial_from_expr<'a>(
        expr: &'a NormExpr,
        sym: &impl AsRef<str>,
    ) -> Option<(&'a Number, usize)> {
        let mut runtime = MONOMIAL_PROG.run(expr);
        let env = runtime.first_match()?;

        let n = if let Some(expr_sym) = env.get_one("x").map(|e| e.get_symbol()).flatten() {
            ensure!(expr_sym == sym.as_ref());

            env.get_one("n")
                .map(|e| e.get_number())
                .flatten()
                .unwrap_or(&ONE)
        } else {
            &ZERO
        };

        let n = n.clone().to_integer().ok()?.to_u64()? as usize;

        let c = env
            .get_one("c")
            .map(|e| e.get_number())
            .flatten()
            .unwrap_or(&ONE);

        Some((c, n))
    }

    pub fn zero(symbol: impl AsRef<str>) -> Self {
        Self {
            coeff: vec![Number::zero()],
            symbol: symbol.as_ref().to_string(),
        }
    }

    pub fn degree(&self) -> usize {
        self.coeff.len() - 1
    }

    pub fn into_coeffs(self) -> Vec<Number> {
        self.coeff
    }

    pub fn coeff(&self, i: usize) -> &Number {
        self.coeff.get(i).unwrap_or(&numbers::ZERO)
    }

    pub fn long_division(&self, other: &Self) -> Result<(Self, Self), LongDivisionError> {
        if self.symbol != other.symbol {
            return Err(LongDivisionError::SymbolMismatch);
        }

        if other.degree() > self.degree() {
            return Ok((UnivariatePolynomial::zero(&self.symbol), other.clone()));
        }

        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_poly_from_expression() {
        let expr = norm_expr!(3 x^2 + 2 x + 9);
        let poly = UnivariatePolynomial::from_expr(&expr, "x");

        assert!(poly.is_some(), "The given polynomial is univariate.");

        let poly = poly.unwrap();
        let coeffs = poly.into_coeffs();

        assert_eq!(
            coeffs,
            vec![
                Number::from_i64(9),
                Number::from_i64(2),
                Number::from_i64(3)
            ]
        );
    }
}
