use crate::{Number, integer::BigInteger, rational::BigRational};
use std::ops;

impl ops::Add for &Number {
    type Output = Number;

    fn add(self, other: Self) -> Self::Output {
        use Number::*;
        match (self, other) {
            (Integer(a), Integer(b)) => Integer(a + b),
            (Rational(a), Rational(b)) => {
                let ret = a + b;
                if ret.is_integer() {
                    Integer(ret.take_numerator())
                } else {
                    Rational(ret)
                }
            }
            (Integer(a), Rational(b)) => Rational(BigRational::from_big_integer(a.clone()) + b),
            (Rational(a), Integer(b)) => Rational(a + BigRational::from_big_integer(b.clone())),
        }
    }
}

impl ops::Add for Number {
    type Output = Number;

    fn add(self, other: Self) -> Self::Output {
        &self + &other
    }
}

impl ops::Add<&Number> for Number {
    type Output = Number;

    fn add(self, other: &Self) -> Self::Output {
        &self + other
    }
}

impl ops::Add<Number> for &Number {
    type Output = Number;

    fn add(self, other: Number) -> Self::Output {
        self + &other
    }
}

impl ops::AddAssign for Number {
    fn add_assign(&mut self, other: Self) {
        let new_value = self.clone() + other;
        *self = new_value;
    }
}

impl ops::Add<u64> for Number {
    type Output = Number;

    fn add(self, other: u64) -> Self::Output {
        match self {
            Number::Integer(a) => Number::Integer(a + BigInteger::from_u64(other)),
            Number::Rational(_a) => {
                todo!("Implement addition of u64 to Rational")
            }
        }
    }
}

impl ops::Sub for &Number {
    type Output = Number;

    fn sub(self, other: Self) -> Self::Output {
        match (self, other) {
            (Number::Integer(a), Number::Integer(b)) => Number::Integer(a - b),
            (Number::Rational(_a), Number::Rational(_b)) => {
                todo!("Implement subtraction for Rational")
            }
            _ => todo!("Implement subtraction for mixed"),
        }
    }
}

impl ops::Sub for Number {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        &self - &other
    }
}

impl ops::Mul for &Number {
    type Output = Number;

    fn mul(self, other: Self) -> Self::Output {
        use Number::*;

        match (self, other) {
            (Integer(a), Integer(b)) => Integer(a * b),
            (Rational(a), Rational(b)) => {
                let ret = a * b;
                if ret.is_integer() {
                    Integer(ret.take_numerator())
                } else {
                    Rational(ret)
                }
            }
            (Integer(a), Rational(b)) => Rational(BigRational::from_big_integer(a.clone()) * b),
            (Rational(a), Integer(b)) => Rational(a * BigRational::from_big_integer(b.clone())),
        }
    }
}

impl ops::Mul for Number {
    type Output = Number;

    fn mul(self, other: Self) -> Self::Output {
        &self * &other
    }
}

impl ops::Mul<&Number> for Number {
    type Output = Number;

    fn mul(self, other: &Self) -> Self::Output {
        &self * other
    }
}

impl ops::Mul<Number> for &Number {
    type Output = Number;

    fn mul(self, other: Number) -> Self::Output {
        self * &other
    }
}

impl ops::MulAssign for Number {
    fn mul_assign(&mut self, other: Number) {
        let new_value = self.clone() * other;
        *self = new_value;
    }
}

impl ops::Div<&Number> for &Number {
    type Output = Option<Number>;

    fn div(self, other: &Number) -> Self::Output {
        use Number::*;

        let rets = match (self, other) {
            (Integer(a), Integer(b)) => {
                if let Some(r) = a / b {
                    Integer(r)
                } else {
                    Rational(BigRational::new(a.clone(), b.clone()).ok()?)
                }
            }
            (Rational(a), Rational(b)) => {
                if let Some(r) = a / b {
                    if r.is_integer() {
                        Integer(r.take_numerator())
                    } else {
                        Rational(r)
                    }
                } else {
                    return None;
                }
            }
            (Integer(a), Rational(_)) => {
                // TODO: Rework numbers API with less cloning
                (&Rational(BigRational::from_big_integer(a.clone())) / other)?
            }
            (Rational(_), Integer(b)) => {
                // TODO: Rework numbers API with less cloning
                (self / &Rational(BigRational::from_big_integer(b.clone())))?
            }
        };

        Some(rets)
    }
}

impl ops::Neg for &Number {
    type Output = Number;

    fn neg(self) -> Self::Output {
        match self {
            Number::Integer(a) => Number::Integer(-a),
            Number::Rational(_a) => todo!("Implement negation for Rational"),
        }
    }
}

impl ops::Neg for Number {
    type Output = Self;

    fn neg(self) -> Self::Output {
        -&self
    }
}
