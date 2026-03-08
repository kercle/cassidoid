use crate::{
    builtin::*,
    expr::{Expr, ExprKind, NormExpr, RawExpr},
};
use numbers::Number;

impl NormExpr {
    pub fn resugar(self) -> RawExpr {
        Self::resugar_inner(self.into_raw())
    }

    fn resugar_inner(expr: RawExpr) -> RawExpr {
        match expr.kind {
            ExprKind::Node { head, args } if head.matches_symbol(ADD_HEAD) => {
                Self::resugar_add(args)
            }
            ExprKind::Node { head, args } if head.matches_symbol(MUL_HEAD) => {
                Self::resugar_mul(args)
            }
            // ExprKind::Node { head, args } if head.matches_symbol(POW_HEAD) && args.len() == 2 => {
            //     let [lhs, rhs]: [RawExpr; 2] =
            //         args.into_iter().map(|e| e.into_raw()).try_into().unwrap();
            //     Self::resugar_pow(lhs, rhs)
            // }
            ExprKind::Node { head, args } => {
                let args: Vec<super::Expr<super::Raw>> =
                    args.into_iter().map(Self::resugar_inner).collect();
                RawExpr::new_node(*head, args)
            }
            _ => expr,
        }
    }

    fn resugar_add(args: Vec<RawExpr>) -> RawExpr {
        assert!(!args.is_empty());

        let mut positives = Vec::new();
        let mut negatives = Vec::new();

        for arg in args {
            let (coeff, term) = NormExpr::split_coefficient(arg);

            if coeff.is_negative() {
                let rhs = if coeff.is_minus_one() {
                    term
                } else {
                    RawExpr::new_binary_node(MUL_HEAD, coeff.abs().into(), term)
                };
                negatives.push(rhs);
            } else if coeff.is_one() {
                positives.push(term);
            } else {
                positives.push(RawExpr::new_binary_node(MUL_HEAD, coeff.into(), term));
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
    fn resugar_mul(args: Vec<RawExpr>) -> RawExpr {
        let mut has_sign = false;
        let mut numerator = Vec::with_capacity(args.len());
        let mut denominator = Vec::with_capacity(args.len());

        for a in args {
            if a.is_number_negative() {
                let num = a.get_number().unwrap();
                has_sign = !has_sign;

                match num.abs() {
                    num @ Number::Integer(_) => numerator.push(Expr::new_number(num)),
                    Number::Rational(r) => {
                        let num_n = Number::Integer(r.numerator().clone());
                        let num_d = Number::Integer(r.denominator().clone());

                        numerator.push(Expr::new_number(num_n));
                        denominator.push(Expr::new_number(num_d));
                    }
                }

                continue;
            }

            if !a.matches_head(POW_HEAD) || a.args_len() != 2 {
                numerator.push(a);
                continue;
            }

            let base = a.get_arg(0).unwrap().clone();
            let exp = a.get_arg(1).unwrap().clone();

            let (coeff, exp_rest) = NormExpr::split_coefficient(exp);

            if coeff.is_negative() {
                let denom_exp = RawExpr::collapse_mul(vec![coeff.abs().into(), exp_rest]);
                denominator.push(RawExpr::new_binary_node(POW_HEAD, base, denom_exp));
            } else {
                // coeff is positive, put back as-is
                numerator.push(RawExpr::new_binary_node(
                    POW_HEAD,
                    base,
                    RawExpr::collapse_mul(vec![coeff.into(), exp_rest]),
                ));
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

    // fn resugar_pow(lhs: RawExpr, rhs: RawExpr) -> RawExpr {
    //     let one_half = Number::new_rational_from_i64(1, 2).unwrap();
    //     if rhs == one_half {
    //         return RawExpr::new_node(
    //             CANNONICAL_HEAD_SQRT,
    //             vec![args.first().unwrap().clone().into_raw()],
    //         );
    //     }

    //     let args = args.into_iter().map(|e| e.resugar()).collect();

    //     Self::resugar_mul(vec![RawExpr::new_node(head.into_raw(), args)])
    // }
}
