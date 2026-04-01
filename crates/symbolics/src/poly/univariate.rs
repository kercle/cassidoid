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
pub enum PolynomialDivisionError {
    SymbolMismatch,
    DivisionByZero,
}

impl UnivariatePolynomial {
    fn new(mut coeff: Vec<Number>, symbol: impl AsRef<str>) -> Self {
        while coeff.len() > 1 && coeff.last().is_some_and(|c| c.is_zero()) {
            coeff.pop();
        }
        if coeff.is_empty() {
            coeff.push(Number::zero());
        }
        Self {
            coeff,
            symbol: symbol.as_ref().to_string(),
        }
    }

    pub fn from_expr(expr: &NormExpr, sym: impl AsRef<str>) -> Option<Self> {
        let mut coeff;

        if builtins::Add::is_application(expr) {
            coeff = Vec::new();

            for t in expr.args()? {
                let (c, n) = Self::monomial_from_expr(t, &sym)?;

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
            let (c, n) = Self::monomial_from_expr(expr, &sym)?;

            coeff = vec![ZERO.clone(); n + 1];
            coeff[n] = c.clone();
        }

        Some(UnivariatePolynomial::new(coeff, sym))
    }

    fn monomial_from_expr<'a>(
        expr: &'a NormExpr,
        sym: &impl AsRef<str>,
    ) -> Option<(&'a Number, usize)> {
        let mut runtime = MONOMIAL_PROG.run(expr);
        let env = runtime.first_match()?;

        let n = if let Some(expr_sym) = env.get_one("x").and_then(|e| e.get_symbol()) {
            ensure!(expr_sym == sym.as_ref());

            env.get_one("n")
                .and_then(|e| e.get_number())
                .unwrap_or(&ONE)
        } else {
            &ZERO
        };

        let n = n.clone().to_integer().ok()?.to_u64()? as usize;

        let c = env
            .get_one("c")
            .and_then(|e| e.get_number())
            .unwrap_or(&ONE);

        Some((c, n))
    }

    pub fn zero(symbol: impl AsRef<str>) -> Self {
        Self {
            coeff: vec![Number::zero()],
            symbol: symbol.as_ref().to_string(),
        }
    }

    pub fn one(symbol: impl AsRef<str>) -> Self {
        Self {
            coeff: vec![Number::one()],
            symbol: symbol.as_ref().to_string(),
        }
    }

    pub fn is_zero(&self) -> bool {
        self.degree() == 0 && self.coeff[0].is_zero()
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

    pub fn leading_coeff(&self) -> &Number {
        self.coeff(self.degree())
    }

    pub fn normalize(&mut self) {
        while self.leading_coeff().is_zero() && self.coeff.len() > 1 {
            self.coeff.pop();
        }
    }

    pub fn shifted(self, n: usize) -> Self {
        let mut coeff = vec![ZERO.clone(); n];
        coeff.extend(self.coeff);

        UnivariatePolynomial::new(coeff, self.symbol)
    }

    pub fn long_division(&self, other: &Self) -> Result<(Self, Self), PolynomialDivisionError> {
        ensure!(
            self.symbol == other.symbol,
            PolynomialDivisionError::SymbolMismatch
        );

        if other.degree() > self.degree() {
            return Ok((UnivariatePolynomial::zero(&self.symbol), self.clone()));
        } else if other.degree() == 0 {
            return Ok((
                (self / other.leading_coeff())?,
                UnivariatePolynomial::zero(&self.symbol),
            ));
        }

        let mut quotient_coeffs = vec![ZERO.clone(); (self.degree() - other.degree()) + 1];
        let mut current_step = self.clone();

        while current_step.degree() >= other.degree() {
            let exp_diff = current_step.degree() - other.degree();

            let coeff = (current_step.leading_coeff() / other.leading_coeff())
                .ok_or(PolynomialDivisionError::DivisionByZero)?;

            let q = (other * &coeff).shifted(exp_diff);

            quotient_coeffs[exp_diff] = coeff;
            current_step = &current_step - &q;
        }

        let r = current_step;
        let mut q = UnivariatePolynomial::new(quotient_coeffs, &self.symbol);

        q.normalize();

        Ok((q, r))
    }
}

pub fn extended_gcd(
    a: &UnivariatePolynomial,
    b: &UnivariatePolynomial,
) -> Result<
    (
        UnivariatePolynomial,
        (UnivariatePolynomial, UnivariatePolynomial),
    ),
    PolynomialDivisionError,
> {
    ensure!(
        a.symbol == b.symbol,
        PolynomialDivisionError::SymbolMismatch
    );

    let sym = a.symbol.as_str();

    let (mut old_r, mut r) = (a.clone(), b.clone());
    let (mut old_s, mut s) = (
        UnivariatePolynomial::one(sym),
        UnivariatePolynomial::zero(sym),
    );
    let (mut old_t, mut t) = (
        UnivariatePolynomial::zero(sym),
        UnivariatePolynomial::one(sym),
    );

    while !r.is_zero() {
        let (q, next_r) = old_r.long_division(&r)?;

        (old_r, r) = (r, next_r);
        let q_times_s = &q * &s;
        (old_s, s) = (s, old_s - q_times_s);
        let q_times_t = &q * &t;
        (old_t, t) = (t, old_t - q_times_t);
    }

    let leading_coeff = old_r.leading_coeff();
    if leading_coeff.is_zero() {
        return Ok((old_r, (old_s, old_t)));
    };

    let ggt = (&old_r / leading_coeff)?;
    let s = (&old_s / leading_coeff)?;
    let t = (&old_t / leading_coeff)?;

    Ok((ggt, (s, t)))
}

impl ops::Add<&UnivariatePolynomial> for &UnivariatePolynomial {
    type Output = UnivariatePolynomial;

    fn add(self, rhs: &UnivariatePolynomial) -> Self::Output {
        // TODO: better solution for checking equality of symbol in
        // polynomial operations.
        assert_eq!(&self.symbol, &rhs.symbol);

        let new_degree = self.degree().max(rhs.degree());
        let mut coeff_new = Vec::with_capacity(new_degree + 1);

        for i in 0..=new_degree {
            coeff_new.push(self.coeff(i) + rhs.coeff(i));
        }

        UnivariatePolynomial::new(coeff_new, &self.symbol)
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

        let mut res = UnivariatePolynomial::new(coeff_new, self.symbol.clone());

        res.normalize();
        res
    }
}

impl ops::Sub for UnivariatePolynomial {
    type Output = UnivariatePolynomial;

    fn sub(self, rhs: UnivariatePolynomial) -> Self::Output {
        &self - &rhs
    }
}

impl ops::Mul<&Number> for &UnivariatePolynomial {
    type Output = UnivariatePolynomial;

    fn mul(self, rhs: &Number) -> Self::Output {
        if rhs.is_zero() {
            return UnivariatePolynomial::zero(&self.symbol);
        }

        let new_coeffs = self.coeff.iter().map(|c| c * rhs).collect();

        UnivariatePolynomial::new(new_coeffs, self.symbol.clone())
    }
}

impl ops::Mul<&UnivariatePolynomial> for &UnivariatePolynomial {
    type Output = UnivariatePolynomial;

    fn mul(self, rhs: &UnivariatePolynomial) -> Self::Output {
        if self.is_zero() || rhs.is_zero() {
            return UnivariatePolynomial::zero(&self.symbol);
        }

        let mut new_coeffs = vec![ZERO.clone(); self.degree() + rhs.degree() + 1];

        for (k, item) in new_coeffs.iter_mut().enumerate() {
            for l in 0..=k {
                *item += self.coeff(l) * rhs.coeff(k - l);
            }
        }

        UnivariatePolynomial::new(new_coeffs, self.symbol.clone())
    }
}

impl ops::Div<&Number> for &UnivariatePolynomial {
    type Output = Result<UnivariatePolynomial, PolynomialDivisionError>;

    fn div(self, rhs: &Number) -> Self::Output {
        if rhs.is_zero() {
            return Err(PolynomialDivisionError::DivisionByZero);
        }

        let new_coeffs: Option<Vec<_>> = self.coeff.iter().map(|c| c / rhs).collect();

        Ok(UnivariatePolynomial::new(
            new_coeffs.ok_or(PolynomialDivisionError::DivisionByZero)?,
            self.symbol.clone(),
        ))
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
    fn test_poly_long_division_example_1() {
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

    #[test]
    fn test_poly_long_division_example_2() {
        let a = UnivariatePolynomial::from_expr(
            &norm_expr!(
                x^3 + 6 x^2 + 11 x + 6
            ),
            "x",
        )
        .unwrap();
        let b = UnivariatePolynomial::from_expr(
            &norm_expr!(
                -2 x^2 - 6 x - 4
            ),
            "x",
        )
        .unwrap();

        let q_expected = UnivariatePolynomial::from_expr(&norm_expr!(-x / 2 - 3 / 2), "x").unwrap();

        let r_expected = UnivariatePolynomial::from_expr(&norm_expr!(0), "x").unwrap();

        let (q, r) = a
            .long_division(&b)
            .expect("Ex1: Long division should succeed");

        assert_eq!(q, q_expected);
        assert_eq!(r, r_expected);
    }

    #[test]
    fn test_division_by_higher_order() {
        let a = UnivariatePolynomial::from_expr(&norm_expr!(x + 1), "x").unwrap();
        let b = UnivariatePolynomial::from_expr(&norm_expr!(x ^ 5), "x").unwrap();

        let (q, r) = a.long_division(&b).expect("should succeed");

        let zero = UnivariatePolynomial::zero("x");
        let expected = UnivariatePolynomial::from_expr(&norm_expr!(x + 1), "x").unwrap();

        assert_eq!(q, zero);
        assert_eq!(r, expected);
    }

    #[test]
    fn test_long_division_constant_divisor() {
        let a = UnivariatePolynomial::from_expr(&norm_expr!(6 x^2 + 3 x + 9), "x").unwrap();
        let b = UnivariatePolynomial::from_expr(&norm_expr!(3), "x").unwrap();

        let (q, r) = a
            .long_division(&b)
            .expect("dividing by a nonzero constant must not fail");

        let q_expected = UnivariatePolynomial::from_expr(&norm_expr!(2 x^2 + x + 3), "x").unwrap();
        let r_expected = UnivariatePolynomial::zero("x");

        assert_eq!(q, q_expected);
        assert_eq!(r, r_expected);
    }

    #[test]
    fn test_long_division_leading_term_cancellation() {
        let a = UnivariatePolynomial::from_expr(&norm_expr!(x ^ 4 + x ^ 3 + x ^ 2 + x + 1), "x")
            .unwrap();
        let b = UnivariatePolynomial::from_expr(&norm_expr!(x ^ 4 + 1), "x").unwrap();

        let (q, r) = a.long_division(&b).expect("should terminate and succeed");

        let q_expected = UnivariatePolynomial::from_expr(&norm_expr!(1), "x").unwrap();
        let r_expected =
            UnivariatePolynomial::from_expr(&norm_expr!(x ^ 3 + x ^ 2 + x), "x").unwrap();

        assert_eq!(q, q_expected);
        assert_eq!(r, r_expected);
    }

    #[test]
    fn test_long_division_by_zero_for_degree_zero_polys() {
        let a = UnivariatePolynomial::from_expr(&norm_expr!(0), "x").expect("should succeed");
        let b = UnivariatePolynomial::from_expr(&norm_expr!(0), "x").expect("should succeed");

        assert!(
            a.long_division(&b).is_err(),
            "Should be division-by-zero error."
        );
    }

    #[test]
    fn test_xgcd_example_1() {
        let a = UnivariatePolynomial::from_expr(&norm_expr!(x ^ 2 - 1), "x").unwrap();

        let b = UnivariatePolynomial::from_expr(&norm_expr!(x - 1), "x").unwrap();

        let (r, (s, t)) = extended_gcd(&a, &b).expect("Should succeed.");

        assert_eq!(
            r,
            UnivariatePolynomial::from_expr(&norm_expr!(x - 1), "x",).unwrap()
        );

        assert_eq!(
            s,
            UnivariatePolynomial::from_expr(&norm_expr!(0), "x",).unwrap()
        );

        assert_eq!(
            t,
            UnivariatePolynomial::from_expr(&norm_expr!(1), "x",).unwrap()
        );
    }

    #[test]
    fn test_xgcd_example_2() {
        let a = UnivariatePolynomial::from_expr(
            &norm_expr!(
                8 + 22 x + 21 x^2 + 8 x^3 + x^4
            ),
            "x",
        )
        .unwrap();

        let b = UnivariatePolynomial::from_expr(
            &norm_expr!(
                6 + 11 x + 6 x^2 + x^3
            ),
            "x",
        )
        .unwrap();

        let (r, (s, t)) = extended_gcd(&a, &b).expect("Should succeed.");

        dbg!(&r);
        dbg!(&s);
        dbg!(&t);

        assert_eq!(
            r,
            UnivariatePolynomial::from_expr(
                &norm_expr!(
                    2 + 3 x + x^2
                ),
                "x",
            )
            .unwrap()
        );

        assert_eq!(
            s,
            UnivariatePolynomial::from_expr(&norm_expr!(-1 / 2), "x",).unwrap()
        );

        assert_eq!(
            t,
            UnivariatePolynomial::from_expr(&norm_expr!(1 + x / 2), "x",).unwrap()
        );
    }
}
