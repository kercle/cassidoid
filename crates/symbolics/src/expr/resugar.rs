use crate::{
    atom::Atom,
    builtin::*,
    expr::{Expr, ExprKind, NormExpr, RawExpr, normalize::split_coefficient},
};
use numbers::Number;

impl NormExpr {
    pub fn resugar(self) -> RawExpr {
        Self::resugar_inner(self)
    }

    fn resugar_number(num: Number) -> RawExpr {
        match num {
            Number::Integer(_) => Atom::number(num).into(),
            Number::Rational(value) => {
                let numerator = Number::Integer(value.numerator().clone());
                let denominator = Number::Integer(value.denominator().clone());

                if denominator.is_one() {
                    Atom::number(numerator).into()
                } else {
                    RawExpr::new_binary_node(
                        DIV_HEAD,
                        Atom::number(numerator).into(),
                        Atom::number(denominator).into(),
                    )
                }
            }
        }
    }

    fn resugar_inner(expr: NormExpr) -> RawExpr {
        match expr.kind {
            ExprKind::Atom {
                entry: Atom::Number(num),
            } => Self::resugar_number(num),
            ExprKind::Node { head, args } if head.matches_symbol(ADD_HEAD) => {
                Self::resugar_add(args)
            }
            ExprKind::Node { head, args } if head.matches_symbol(MUL_HEAD) => {
                Self::resugar_mul(args)
            }
            ExprKind::Node { head, args } if head.matches_symbol(POW_HEAD) && args.len() == 2 => {
                let [lhs, rhs]: [NormExpr; 2] = args.try_into().unwrap();
                Self::resugar_pow(lhs, rhs)
            }
            ExprKind::Node { head, args } => {
                let args: Vec<super::Expr<super::Raw>> =
                    args.into_iter().map(Self::resugar_inner).collect();
                RawExpr::new_node((*head).into_raw(), args)
            }
            _ => expr.into_raw(),
        }
    }

    fn resugar_add(args: Vec<Self>) -> RawExpr {
        assert!(!args.is_empty());

        let mut positives = Vec::new();
        let mut negatives = Vec::new();

        for arg in args {
            let (coeff, term) = split_coefficient(arg);
            let term = term.map(Self::resugar_inner);

            if let Some(term) = term {
                if coeff.is_one() {
                    positives.push(term);
                } else if coeff.is_minus_one() {
                    negatives.push(term);
                } else if coeff.is_positive() {
                    positives.push(RawExpr::new_binary_node(MUL_HEAD, coeff.into(), term));
                } else {
                    negatives.push(RawExpr::new_binary_node(MUL_HEAD, coeff.abs().into(), term));
                }
            } else {
                if coeff.is_positive() {
                    positives.push(coeff.into());
                } else {
                    negatives.push(coeff.abs().into());
                }
            }
        }

        if positives.is_empty() {
            RawExpr::new_unary_node(NEG_HEAD, RawExpr::collapse_add(negatives))
        } else if negatives.is_empty() {
            RawExpr::collapse_add(positives)
        } else {
            RawExpr::new_binary_node(
                SUB_HEAD,
                RawExpr::collapse_add(positives),
                RawExpr::collapse_add(negatives),
            )
        }
    }
    fn resugar_mul(args: Vec<Self>) -> RawExpr {
        let mut has_sign = false;
        let mut numerator = Vec::with_capacity(args.len());
        let mut denominator = Vec::with_capacity(args.len());

        for arg in args {
            if arg.is_number_negative() {
                let num = arg.get_number().unwrap();
                has_sign = !has_sign;

                match num.abs() {
                    num @ Number::Integer(_) => {
                        if !num.is_one() && !num.is_minus_one() {
                            numerator.push(Expr::new_number(num))
                        }
                    }
                    Number::Rational(r) => {
                        let num_n = Number::Integer(r.numerator().clone());
                        let num_d = Number::Integer(r.denominator().clone());

                        numerator.push(Expr::new_number(num_n));
                        denominator.push(Expr::new_number(num_d));
                    }
                }

                continue;
            }

            let [base, exp]: [NormExpr; 2] = if arg.is_application_of(POW_HEAD, 2) {
                // If it's a power, we take a closer look at arguments.

                let ExprKind::Node { args, .. } = arg.kind else {
                    // if its a power, it's a node.
                    unreachable!()
                };
                args.try_into().unwrap()
            } else {
                // otherwise, just add to numerator.
                numerator.push(Self::resugar_inner(arg));
                continue;
            };

            let (coeff, exp_rest) = split_coefficient(exp);
            let coeff_abs = coeff.abs();

            let new_pow_expr = if let Some(exp_rest) = exp_rest
                && coeff_abs.is_one()
            {
                // Since we are working with a normalized node, we don't have
                // to worry about all the edge cased here (e.g. coeff cannot
                // be 0, etc.)

                let rhs = if coeff_abs.is_one() {
                    Self::resugar_inner(exp_rest)
                } else {
                    RawExpr::new_node(
                        MUL_HEAD,
                        vec![
                            Self::resugar_number(coeff_abs),
                            Self::resugar_inner(exp_rest),
                        ],
                    )
                };
                RawExpr::new_binary_node(POW_HEAD, Self::resugar_inner(base), rhs)
            } else if coeff_abs.is_one() {
                base.into_raw()
            } else {
                RawExpr::new_binary_node(POW_HEAD, base.into_raw(), Self::resugar_number(coeff_abs))
            };

            if coeff.is_negative() {
                denominator.push(new_pow_expr);
            } else {
                numerator.push(new_pow_expr);
            }
        }

        let ret_unsigned = match (numerator.is_empty(), denominator.is_empty()) {
            (_, true) => RawExpr::collapse_mul(numerator),
            (true, _) => RawExpr::new_binary_node(
                DIV_HEAD,
                RawExpr::new_number(Number::one()),
                RawExpr::collapse_mul(denominator),
            ),
            (false, false) => RawExpr::new_binary_node(
                DIV_HEAD,
                RawExpr::collapse_mul(numerator),
                RawExpr::collapse_mul(denominator),
            ),
        };

        if has_sign {
            RawExpr::new_unary_node(NEG_HEAD, ret_unsigned)
        } else {
            ret_unsigned
        }
    }

    fn resugar_pow(lhs: NormExpr, rhs: NormExpr) -> RawExpr {
        let Some(rhs_num) = rhs.get_number() else {
            return RawExpr::new_binary_node(
                POW_HEAD,
                Self::resugar_inner(lhs),
                Self::resugar_inner(rhs),
            );
        };

        let is_neg_exp = rhs_num.is_negative();
        let rhs_num = rhs_num.abs();

        let one_half = Number::new_rational_from_i64(1, 2).unwrap();
        let res = if rhs_num.is_one() {
            Self::resugar_inner(lhs)
        } else if rhs_num == one_half {
            RawExpr::new_unary_node(CANNONICAL_HEAD_SQRT, Self::resugar_inner(lhs))
        } else {
            RawExpr::new_binary_node(
                POW_HEAD,
                Self::resugar_inner(lhs),
                RawExpr::new_number(rhs_num),
            )
        };

        if is_neg_exp {
            RawExpr::new_binary_node(DIV_HEAD, Expr::from_i64(1), res)
        } else {
            res
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{norm_expr, raw_expr};

    #[test]
    fn test_resugar() {
        let expr = norm_expr! { x - y };

        assert_eq!(expr.resugar(), raw_expr! { Sub[x, y] });
    }
}
