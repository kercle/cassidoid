use std::sync::LazyLock;
use std::{fmt::Debug, ops};

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

#[derive(Clone, PartialEq)]
pub struct UnivariatePolynomial {
    coeff: Vec<Number>,
    symbol: String,
}

#[derive(Debug, Clone)]
pub enum LongDivisionError {
    SymbolMismatch,
    DivisionByZero,
}

impl UnivariatePolynomial {
    pub fn from_expr(expr: &NormExpr, sym: impl AsRef<str>) -> Option<Self> {
        let mut coeff;

        if builtins::Add::is_application(expr) {
            coeff = Vec::new();

            for t in expr.args()? {
                let (c, n) = Self::monimial_from_expr(t, &sym)?;

                while n > coeff.len() {
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

    pub fn coeff_higest_monomial(&self) -> &Number {
        if self.degree() == 0 {
            return &ZERO;
        }

        self.coeff(self.degree())
    }

    pub fn normalize(&mut self) {
        while self.coeff_higest_monomial().is_zero() && self.coeff.len() > 1 {
            self.coeff.pop();
        }
    }

    pub fn shifted(self, n: usize) -> Self {
        let mut coeff = vec![ZERO.clone(); n];
        coeff.extend(self.coeff);

        UnivariatePolynomial {
            coeff,
            symbol: self.symbol,
        }
    }

    pub fn long_division(&self, other: &Self) -> Result<(Self, Self), LongDivisionError> {
        ensure!(
            self.symbol == other.symbol,
            LongDivisionError::SymbolMismatch
        );

        if other.degree() > self.degree() {
            return Ok((UnivariatePolynomial::zero(&self.symbol), other.clone()));
        }

        let mut quotient_coeffs = vec![ZERO.clone(); (self.degree() - other.degree()) + 1];
        let mut current_step = self.clone();

        while current_step.degree() >= other.degree() {
            let exp_diff = current_step.degree() - other.degree();

            let coeff = (current_step.coeff_higest_monomial() / other.coeff_higest_monomial())
                .ok_or(LongDivisionError::DivisionByZero)?;

            let q = (other * &coeff).shifted(exp_diff);

            quotient_coeffs[exp_diff] = coeff;
            current_step = &current_step - &q;
        }

        let r = current_step;
        let mut q = UnivariatePolynomial {
            coeff: quotient_coeffs,
            symbol: self.symbol.clone(),
        };

        q.normalize();

        Ok((q, r))
    }
}

impl ops::Sub<&UnivariatePolynomial> for &UnivariatePolynomial {
    type Output = UnivariatePolynomial;

    fn sub(self, rhs: &UnivariatePolynomial) -> Self::Output {
        // TODO: better solution for checking equality of symbol in
        // polynomial operations.
        assert_eq!(&self.symbol, &rhs.symbol);

        let new_degree = self.degree().max(rhs.degree());
        let mut coeff_new = Vec::with_capacity(new_degree + 1);

        for i in 0..=new_degree {
            coeff_new.push(self.coeff(i) - rhs.coeff(i));
        }

        let mut res = UnivariatePolynomial {
            coeff: coeff_new,
            symbol: self.symbol.clone(),
        };

        res.normalize();
        res
    }
}

impl ops::Mul<&Number> for &UnivariatePolynomial {
    type Output = UnivariatePolynomial;

    fn mul(self, rhs: &Number) -> Self::Output {
        if rhs.is_zero() {
            return UnivariatePolynomial::zero(&self.symbol);
        }

        let new_coeffs = self.coeff.iter().map(|c| c * rhs).collect();

        UnivariatePolynomial {
            coeff: new_coeffs,
            symbol: self.symbol.clone(),
        }
    }
}

impl Debug for UnivariatePolynomial {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let x = self.symbol.as_str();
        for (n, c) in self.coeff.iter().enumerate().rev() {
            write!(f, "{c:?}*{x}^{n}")?;

            if n > 0 {
                write!(f, " + ")?;
            }
        }
        Ok(())
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

    #[test]
    fn test_poly_long_division() {
        let a = UnivariatePolynomial::from_expr(
            &norm_expr!(
                4 x^7 - 2 x^5 + 9 x^2 + x - 17
            ),
            "x",
        )
        .unwrap();
        let b = UnivariatePolynomial::from_expr(
            &norm_expr!(
                x^3 - 7 x + 2
            ),
            "x",
        )
        .unwrap();

        let q_expected = UnivariatePolynomial::from_expr(
            &norm_expr!(
                182 - 8 x + 26 x^2 + 4 x^4
            ),
            "x",
        )
        .unwrap();

        let r_expected = UnivariatePolynomial::from_expr(
            &norm_expr!(
                -381 + 1291 x - 99 x^2
            ),
            "x",
        )
        .unwrap();

        let (q, r) = a
            .long_division(&b)
            .expect("Ex1: Long division should succeed");

        assert_eq!(q, q_expected);
        assert_eq!(r, r_expected);
    }
}
