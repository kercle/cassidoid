use std::ops;

use numbers::{RealScalar, integer::BigInteger};

use crate::parser::ast::AstNode;

impl ops::Add for AstNode {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        AstNode::new_add(self, other)
    }
}

impl ops::Add<u64> for AstNode {
    type Output = Self;

    fn add(self, other: u64) -> Self {
        AstNode::new_add(
            self,
            AstNode::new_constant(RealScalar::Integer(BigInteger::from_u64(other))),
        )
    }
}

impl ops::Add<AstNode> for u64 {
    type Output = AstNode;

    fn add(self, other: AstNode) -> AstNode {
        AstNode::new_add(
            AstNode::new_constant(RealScalar::Integer(BigInteger::from_u64(self))),
            other,
        )
    }
}

impl ops::Sub for AstNode {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        AstNode::new_sub(self, other)
    }
}

impl ops::Mul for AstNode {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        AstNode::new_mul(self, other)
    }
}

impl ops::Div for AstNode {
    type Output = Self;

    fn div(self, other: Self) -> Self {
        AstNode::new_div(self, other)
    }
}

impl ops::Mul<u64> for AstNode {
    type Output = Self;

    fn mul(self, other: u64) -> Self {
        AstNode::new_mul(
            self,
            AstNode::new_constant(RealScalar::Integer(BigInteger::from_u64(other))),
        )
    }
}

impl ops::Mul<AstNode> for u64 {
    type Output = AstNode;

    fn mul(self, other: AstNode) -> AstNode {
        AstNode::new_mul(
            AstNode::new_constant(RealScalar::Integer(BigInteger::from_u64(self))),
            other,
        )
    }
}
