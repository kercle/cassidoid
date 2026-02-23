mod functions_known_values;
mod trigonometric_functions;

use crate::{
    calculus::{derivative::resolve_derivatives, integrate::resolve_indefinite_integrals},
    expr::{Expr, NormalizedExpr},
};

pub struct Simplifier {
    expr: NormalizedExpr,
}

impl Simplifier {
    pub fn new(expr: NormalizedExpr) -> Simplifier {
        Simplifier {
            expr: expr.collect_like_terms(),
        }
    }

    pub fn simple(self) -> NormalizedExpr {
        self.with_known_function_values()
            .with_resolved_derivatives()
            .with_resolved_indefinite_integrals()
            .with_trigonometric_identities()
            .finish_normalized()
    }

    pub fn with_resolved_derivatives(self) -> Simplifier {
        Simplifier::new(resolve_derivatives(self.expr.take_expr()))
    }

    pub fn with_resolved_indefinite_integrals(self) -> Simplifier {
        Simplifier::new(resolve_indefinite_integrals(self.expr.take_expr()))
    }

    pub fn with_trigonometric_identities(self) -> Simplifier {
        let expr = NormalizedExpr::new(trigonometric_functions::simplify_trigon(
            self.expr.take_expr(),
        ));
        Simplifier::new(expr)
    }

    pub fn with_known_function_values(self) -> Simplifier {
        let expr = NormalizedExpr::new(functions_known_values::simplify_evaluation_at_zero(
            self.expr.take_expr(),
        ));
        Simplifier::new(expr)
    }

    pub fn finish(self) -> Expr {
        self.expr.take_expr()
    }

    pub fn finish_normalized(self) -> NormalizedExpr {
        self.expr
    }
}
