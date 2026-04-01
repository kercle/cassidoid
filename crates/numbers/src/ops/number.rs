use crate::Number;
use std::ops;

impl ops::Add for &Number {
    type Output = Number;

    fn add(self, other: Self) -> Self::Output {
        self.inner_add(other)
    }
}

impl ops::Add for Number {
    type Output = Number;

    fn add(self, other: Self) -> Self::Output {
        self.inner_add(&other)
    }
}

impl ops::Add<&Number> for Number {
    type Output = Number;

    fn add(self, other: &Number) -> Self::Output {
        self.inner_add(other)
    }
}

impl ops::Add<Number> for &Number {
    type Output = Number;

    fn add(self, other: Number) -> Self::Output {
        self.inner_add(&other)
    }
}

impl ops::AddAssign for Number {
    fn add_assign(&mut self, other: Self) {
        let new_value = self.inner_add(&other);
        *self = new_value;
    }
}

impl ops::Add<u64> for Number {
    type Output = Number;

    fn add(self, other: u64) -> Self::Output {
        self.add(&Number::from_u64(other))
    }
}

impl ops::Sub for &Number {
    type Output = Number;

    fn sub(self, other: Self) -> Self::Output {
        self.inner_sub(other)
    }
}

impl ops::Sub for Number {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        self.inner_sub(&other)
    }
}

impl ops::Mul for &Number {
    type Output = Number;

    fn mul(self, other: Self) -> Self::Output {
        self.inner_mul(other)
    }
}

impl ops::Mul for Number {
    type Output = Number;

    fn mul(self, other: Self) -> Self::Output {
        self.inner_mul(&other)
    }
}

impl ops::Mul<&Number> for Number {
    type Output = Number;

    fn mul(self, other: &Self) -> Self::Output {
        self.inner_mul(other)
    }
}

impl ops::Mul<Number> for &Number {
    type Output = Number;

    fn mul(self, other: Number) -> Self::Output {
        self.inner_mul(&other)
    }
}

impl ops::MulAssign for Number {
    fn mul_assign(&mut self, other: Number) {
        let new_value = self.inner_mul(&other);
        *self = new_value;
    }
}

impl ops::Div<&Number> for &Number {
    type Output = Option<Number>;

    fn div(self, other: &Number) -> Self::Output {
        self.inner_div(other)
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
