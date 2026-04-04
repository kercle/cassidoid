use crate::{
    builtins::{
        BuiltInCategory,
        traits::{ApplicationError, BuiltIn, BuiltInDoc, PatternDoc},
    },
    ensure,
    expr::{Expr, NormExpr},
    norm_expr, raw_expr,
    rewrite::Rewriter,
};

pub struct Integrate {
    pattern_doc: Vec<PatternDoc>,
    rewriter: Rewriter,
}

impl Integrate {
    pub fn new() -> Self {
        Self {
            pattern_doc: vec![PatternDoc::new(
                raw_expr!( Integrate[f_, x_?IsSymbol] ),
                "gives the indefinite integral (anti-derivative) $\\int f(x)\\!{\\rm d}x$",
            )],
            rewriter: build_rewriter(),
        }
    }
}

impl Default for Integrate {
    fn default() -> Self {
        Self::new()
    }
}

impl BuiltIn for Integrate {
    #[inline(always)]
    fn head() -> &'static str {
        "Integrate"
    }

    fn head_dyn(&self) -> &'static str {
        Self::head()
    }

    fn doc(&self) -> BuiltInDoc {
        BuiltInDoc {
            category: BuiltInCategory::Calculus,
            title: Self::head(),
            summary: "Symbolically determine integrals of expressions.",
            pattern_doc: self.pattern_doc.clone(),
            examples: vec![
                ("Integrate[x,x]", "x^2/2"),
                ("Integrate[2 * Sqrt[x] + x^6, x]", "(4/3)*x^(3/2) + x^7/7"),
                ("Integrate[Cos[x+f[y]],x]", "Sin[x+f[y]]"),
            ],
            related: vec!["Diff"],
        }
    }

    fn apply_all(&self, expr: NormExpr) -> NormExpr {
        expr.rewrite_all(&self.rewriter, 1000)
    }

    fn validate_application_of<S>(
        head: &Expr<S>,
        children: &[Expr<S>],
    ) -> Result<(), ApplicationError> {
        ensure!(children.len() == 2, ApplicationError::ArityMismatch);
        ensure!(
            children.get(1).is_some_and(|a| a.is_symbol()),
            ApplicationError::ExpectedSymbolAt(1)
        );
        ensure!(
            head.matches_symbol(Self::head()),
            ApplicationError::HeadMismatch
        );
        Ok(())
    }
}

fn build_rewriter() -> Rewriter {
    Rewriter::new().with_rules(&norm_expr!({
        // =============== Linearity ===============

        Integrate[f_ + r__, x_?IsSymbol] :> Integrate[f, x] + Integrate[Add[r],x];
        Integrate[c_ * r__, x_?IsSymbol] /; FreeOf[c, x] :> c * Integrate[Mul[r],x];

        // =============== Basic ===============

         Integrate[c_, x_?IsSymbol] /; FreeOf[c, x] :> c * x;
         Integrate[x_, x_?IsSymbol] :>  x ^ 2 / 2;
         Integrate[x_ * f__, x_?IsSymbol] :>  x * Integrate[Mul[f], x] - Integrate[Integrate[Mul[f], x], x];
         Integrate[x_^n_?IsPositiveInteger * f__, x_?IsSymbol] :>  x^n * Integrate[Mul[f], x] - n * Integrate[x^(n-1) * Integrate[Mul[f], x], x];

        // =============== Powers ===============

        Integrate[1 / (a_. + b_. * x_), x_?IsSymbol] /; FreeOf[(a, b), x] :> Log[Abs[a + b * x]] / b;
        Integrate[x_ ^ k_?IsNumber, x_?IsSymbol] :> x ^ (k + 1) / (k + 1);

        // =============== Exponentials ===============

        Integrate[Exp[a_. + b_. * x_], x_?IsSymbol] /; FreeOf[(a, b), x] :> Exp[a + b * x] / b;

        // =============== Logarithms ===============

        Integrate[Log[a_. + b_. * x_], x_?IsSymbol] /; FreeOf[(a, b), x] :> (a / b + x) * Log[a + b * x] - x;

        // =============== Trigonometric functions ===============

        Integrate[Sin[a_. + b_. * x_], x_?IsSymbol] /; FreeOf[(a, b), x] :> -Cos[a + b * x] / b;
        Integrate[Cos[a_. + b_. * x_], x_?IsSymbol] /; FreeOf[(a, b), x] :> Sin[a + b * x] / b;

        // =============== RUBI ===================================
        // ========================================================

        Int[f_ + r__, x_?IsSymbol] :> Int[f, x] + Int[Add[r],x];
        Int[c_ * r__, x_?IsSymbol]
            /; FreeOf[c, x]
            :> c * Int[Mul[r],x];

        // === 1 Algebraic functions ==============================
        // ===== 1.1 Binomial products ============================
        // ======= 1.1.1 Linear ===================================

        // --- 1.1.1.1 (a+b x)^m.m
        Int[1/x_, x_?IsSymbol]
            :> Log[Abs[x]];

        Int[x_^m_, x_?IsSymbol]
            /; FreeOf[m, x] && m != -1
            :> x^(m + 1)/(m + 1);

        Int[1/(a_ + b_.*x_), x_?IsSymbol]
            /; FreeOf[{a, b}, x]
            :> Log[Abs[a + b*x]]/b;

        Int[(a_. + b_.*x_)^m_, x_?IsSymbol]
            /; FreeOf[(a, b, m), x] && m != -1
            :> (a + b*x)^(m + 1)/(b*(m + 1));

        // TODO: Needs LinearQ equivalend and Subst
        // Int[(a_. + b_.*u_)^m_, x_?IsSymbol]
        //     /; FreeOf[{a, b, m}, x] && LinearQ[u, x] && u != x
        //     := 1/Coefficient[u, x, 1]*Subst[Int[(a + b*x)^m, x], x, u] 

        // --- 1.1.1.2 (a+b x)^m (c+d x)^n

        Int[(a_ + b_.*x_)^m_.*(c_ + d_.*x_), x_?IsSymbol]
            /; FreeOf[(a, b, c, d, m), x] && a*d - b*c*(m + 2) == 0
            :> d*x*(a + b*x)^(m + 1)/(b*(m + 2));

        Int[1/((a_ + b_.*x_)*(c_ + d_.*x_)), x_?IsSymbol]
            /; FreeOf[(a, b, c, d), x] && b*c + a*d == 0
            :> Int[1/(a*c + b*d*x^2), x];

        Int[1/((a_. + b_.*x_)*(c_. + d_.*x_)), x_?IsSymbol]
            /; FreeOf[(a, b, c, d), x] && b*c - a*d != 0
            :> b/(b*c - a*d)*Int[1/(a + b*x), x] - d/(b*c - a*d)*Int[1/(c + d*x), x];

        Int[(a_. + b_.*x_)^m_.*(c_. + d_.*x_)^n_, x_?IsSymbol]
            /; FreeOf[(a, b, c, d, m, n), x] && b*c - a*d != 0 && m + n + 2 == 0 && m != -1
            :> (a + b*x)^(m + 1)*(c + d*x)^(n + 1)/((b*c - a*d)*(m + 1));

        // TODO: Look into IGtQ
        // Int[(a_ + b_.*x_)^m_*(c_ + d_.*x_)^m_, x_?IsSymbol]
        //     /; FreeOf[(a, b, c, d), x] && b*c + a*d == 0 && IGtQ[m + 1/2, 0]
        //     :> x*(a + b*x)^m*(c + d*x)^m/(2*m + 1) + 2*a*c*m/(2*m + 1)*Int[(a + b*x)^(m - 1)*(c + d*x)^(m - 1), x]

        Int[1/((a_ + b_.*x_)^(3/2)*(c_ + d_.*x_)^(3/2)), x_?IsSymbol]
            /; FreeOf[(a, b, c, d), x] && b*c + a*d == 0
            :> x/(a*c*Sqrt[a + b*x]*Sqrt[c + d*x]);

        // TODO: Look into ILtQ
        // Int[(a_ + b_.*x_)^m_*(c_ + d_.*x_)^m_, x_?IsSymbol] 
        //     /; FreeOf[(a, b, c, d), x] && EqQ[b*c + a*d, 0] && ILtQ[m + 3/2, 0]
        //     :> -x*(a + b*x)^(m + 1)*(c + d*x)^(m + 1)/(2*a*c*(m + 1)) + (2*m + 3)/(2*a*c*(m + 1))* Int[(a + b*x)^(m + 1)*(c + d*x)^(m + 1), x] 

        // TODO: Needs GtQ
        // Int[(a_ + b_.*x_)^m_.*(c_ + d_.*x_)^m_., x_?IsSymbol]
        //     /; FreeQ[{a, b, c, d, m}, x] && EqQ[b*c + a*d, 0] && (IntegerQ[m] || GtQ[a, 0] && GtQ[c, 0])
        //     :> Int[(a*c + b*d*x^2)^m, x]

        // TODO: Needs FracPart
        // Int[(a_ + b_.*x_)^m_*(c_ + d_.*x_)^m_, x_?IsSymbol]
        //     /; FreeQ[{a, b, c, d, m}, x] && EqQ[b*c + a*d, 0] && Not[IntegerQ[2*m]]        
        //     :> (a + b*x)^ FracPart[m]*(c + d*x)^FracPart[m]/(a*c + b*d*x^2)^FracPart[m]* Int[(a*c + b*d*x^2)^m, x]

        // TODO: Needs GtQ and ILtQ
        // Int[(a_. + b_.*x_)^m_*(c_. + d_.*x_)^n_, x_?IsSymbol]
        //     /; FreeQ[{a, b, c, d, n}, x] && NeQ[b*c - a*d, 0] && ILtQ[m, -1] && Not[IntegerQ[n]] && GtQ[n, 0]
        //     :> (a + b*x)^(m + 1)*(c + d*x)^n/(b*(m + 1)) - d*n/(b*(m + 1))*Int[(a + b*x)^(m + 1)*(c + d*x)^(n - 1), x]

        // TODO: Needs ILtQ
        // Int[(a_. + b_.*x_)^m_*(c_. + d_.*x_)^n_, x_?IsSymbol]
        //     /; FreeQ[{a, b, c, d, n}, x] && NeQ[b*c - a*d, 0] && ILtQ[m, -1] && Not[IntegerQ[n]] && LtQ[n, 0]
        //     :> (a + b*x)^(m + 1)*(c + d*x)^(n + 1)/((b*c - a*d)*(m + 1)) - d*(m + n + 2)/((b*c - a*d)*(m + 1))* Int[(a + b*x)^(m + 1)*(c + d*x)^n, x]

        //Int[(a_. + b_.*x_)^m_.*(c_. + d_.*x_)^n_., x_?IsSymbol] := Int[ExpandIntegrand[(a + b*x)^m*(c + d*x)^n, x], x] /; FreeQ[{a, b, c, d, n}, x] && NeQ[b*c - a*d, 0] && IGtQ[m, 0] && (Not[IntegerQ[n]] || EqQ[c, 0] && LeQ[7*m + 4*n + 4, 0] || LtQ[9*m + 5*(n + 1), 0] || GtQ[m + n + 2, 0])

        // Int[(a_ + b_.*x_)^m_*(c_. + d_.*x_)^n_., x_?IsSymbol] := Int[ExpandIntegrand[(a + b*x)^m*(c + d*x)^n, x], x] /; FreeQ[{a, b, c, d}, x] && NeQ[b*c - a*d, 0] && ILtQ[m, 0] && IntegerQ[n] && Not[IGtQ[n, 0] && LtQ[m + n + 2, 0]]

        // Int[(a_. + b_.*x_)^m_*(c_. + d_.*x_)^n_, x_?IsSymbol] := (a + b*x)^(m + 1)*(c + d*x)^(n + 1)/((b*c - a*d)*(m + 1)) - d*Simplify[m + n + 2]/((b*c - a*d)*(m + 1))* Int[(a + b*x)^Simplify[m + 1]*(c + d*x)^n, x] /; FreeQ[{a, b, c, d, m, n}, x] && NeQ[b*c - a*d, 0] && ILtQ[Simplify[m + n + 2], 0] && NeQ[m, -1] && Not[ LtQ[m, -1] && LtQ[n, -1] && (EqQ[a, 0] || NeQ[c, 0] && LtQ[m - n, 0] && IntegerQ[n])] && (SumSimplerQ[m, 1] || Not[SumSimplerQ[n, 1]])

        //Int[1/((a_ + b_.*x_)^(9/4)*(c_ + d_.*x_)^(1/4)), x_?IsSymbol] := -4/(5*b*(a + b*x)^(5/4)*(c + d*x)^(1/4)) - d/(5*b)*Int[1/((a + b*x)^(5/4)*(c + d*x)^(5/4)), x] /; FreeQ[{a, b, c, d}, x] && EqQ[b*c + a*d, 0] && NegQ[a^2*b^2]

        //Int[(a_. + b_.*x_)^m_*(c_. + d_.*x_)^n_, x_?IsSymbol] := (a + b*x)^(m + 1)*(c + d*x)^n/(b*(m + 1)) - d*n/(b*(m + 1))*Int[(a + b*x)^(m + 1)*(c + d*x)^(n - 1), x] /; FreeQ[{a, b, c, d}, x] && NeQ[b*c - a*d, 0] && GtQ[n, 0] && LtQ[m, -1] && Not[IntegerQ[n] && Not[IntegerQ[m]]] && Not[ ILeQ[m + n + 2, 0] && (FractionQ[m] || GeQ[2*n + m + 1, 0])] && IntLinearQ[a, b, c, d, m, n, x]

        //Int[1/((a_ + b_.*x_)^(5/4)*(c_ + d_.*x_)^(1/4)), x_?IsSymbol] := -2/(b*(a + b*x)^(1/4)*(c + d*x)^(1/4)) + c*Int[1/((a + b*x)^(5/4)*(c + d*x)^(5/4)), x] /; FreeQ[{a, b, c, d}, x] && EqQ[b*c + a*d, 0] && NegQ[a^2*b^2]

        //Int[(a_ + b_.*x_)^m_*(c_ + d_.*x_)^n_, x_?IsSymbol] := (a + b*x)^(m + 1)*(c + d*x)^n/(b*(m + n + 1)) + 2*c*n/(m + n + 1)*Int[(a + b*x)^m*(c + d*x)^(n - 1), x] /; FreeQ[{a, b, c, d}, x] && EqQ[b*c + a*d, 0] && IGtQ[m + 1/2, 0] && IGtQ[n + 1/2, 0] && LtQ[m, n]

        //Int[(a_. + b_.*x_)^m_*(c_. + d_.*x_)^n_, x_?IsSymbol] := (a + b*x)^(m + 1)*(c + d*x)^n/(b*(m + n + 1)) + n*(b*c - a*d)/(b*(m + n + 1))* Int[(a + b*x)^m*(c + d*x)^(n - 1), x] /; FreeQ[{a, b, c, d}, x] && NeQ[b*c - a*d, 0] && GtQ[n, 0] && NeQ[m + n + 1, 0] && Not[ IGtQ[m, 0] && (Not[IntegerQ[n]] || GtQ[m, 0] && LtQ[m - n, 0])] && Not[ILtQ[m + n + 2, 0]] && IntLinearQ[a, b, c, d, m, n, x]

        //Int[(a_. + b_.*x_)^m_*(c_. + d_.*x_)^n_, x_?IsSymbol] := (a + b*x)^(m + 1)*(c + d*x)^(n + 1)/((b*c - a*d)*(m + 1)) - d*(m + n + 2)/((b*c - a*d)*(m + 1))* Int[(a + b*x)^(m + 1)*(c + d*x)^n, x] /; FreeQ[{a, b, c, d, n}, x] && NeQ[b*c - a*d, 0] && LtQ[m, -1] && Not[ LtQ[n, -1] && (EqQ[a, 0] || NeQ[c, 0] && LtQ[m - n, 0] && IntegerQ[n])] && IntLinearQ[a, b, c, d, m, n, x]

        //Int[1/(Sqrt[a_ + b_.*x_]*Sqrt[c_ + d_.*x_]), x_?IsSymbol] := ArcCosh[b*x/a]/b /; FreeQ[{a, b, c, d}, x] && EqQ[a + c, 0] && EqQ[b - d, 0] && GtQ[a, 0]

        //Int[1/(Sqrt[a_ + b_.*x_]*Sqrt[c_. + d_.*x_]), x_?IsSymbol] := Int[1/Sqrt[a*c - b*(a - c)*x - b^2*x^2], x] /; FreeQ[{a, b, c, d}, x] && EqQ[b + d, 0] && GtQ[a + c, 0]

        //Int[1/(Sqrt[a_. + b_.*x_]*Sqrt[c_. + d_.*x_]), x_?IsSymbol] := 2/Sqrt[b]* Subst[Int[1/Sqrt[b*c - a*d + d*x^2], x], x, Sqrt[a + b*x]] /; FreeQ[{a, b, c, d}, x] && GtQ[b*c - a*d, 0] && GtQ[b, 0]

        //Int[1/((a_. + b_.*x_)*(c_. + d_.*x_)^(1/3)), x_?IsSymbol] := With[{q = Rt[(b*c - a*d)/b, 3]}, -Log[RemoveContent[a + b*x, x]]/(2*b*q) - 3/(2*b*q)*Subst[Int[1/(q - x), x], x, (c + d*x)^(1/3)] + 3/(2*b)* Subst[Int[1/(q^2 + q*x + x^2), x], x, (c + d*x)^(1/3)]] /; FreeQ[{a, b, c, d}, x] && PosQ[(b*c - a*d)/b]

        //Int[1/((a_. + b_.*x_)*(c_. + d_.*x_)^(1/3)), x_?IsSymbol] := With[{q = Rt[-(b*c - a*d)/b, 3]}, Log[RemoveContent[a + b*x, x]]/(2*b*q) - 3/(2*b*q)*Subst[Int[1/(q + x), x], x, (c + d*x)^(1/3)] + 3/(2*b)* Subst[Int[1/(q^2 - q*x + x^2), x], x, (c + d*x)^(1/3)]] /; FreeQ[{a, b, c, d}, x] && NegQ[(b*c - a*d)/b]

        //Int[1/((a_. + b_.*x_)*(c_. + d_.*x_)^(2/3)), x_?IsSymbol] := With[{q = Rt[(b*c - a*d)/b, 3]}, -Log[RemoveContent[a + b*x, x]]/(2*b*q^2) - 3/(2*b*q^2)*Subst[Int[1/(q - x), x], x, (c + d*x)^(1/3)] - 3/(2*b*q)* Subst[Int[1/(q^2 + q*x + x^2), x], x, (c + d*x)^(1/3)]] /; FreeQ[{a, b, c, d}, x] && PosQ[(b*c - a*d)/b]

        //Int[1/((a_. + b_.*x_)*(c_. + d_.*x_)^(2/3)), x_?IsSymbol] := With[{q = Rt[-(b*c - a*d)/b, 3]}, -Log[RemoveContent[a + b*x, x]]/(2*b*q^2) + 3/(2*b*q^2)*Subst[Int[1/(q + x), x], x, (c + d*x)^(1/3)] + 3/(2*b*q)* Subst[Int[1/(q^2 - q*x + x^2), x], x, (c + d*x)^(1/3)]] /; FreeQ[{a, b, c, d}, x] && NegQ[(b*c - a*d)/b]

        //Int[1/((a_. + b_.*x_)^(1/3)*(c_. + d_.*x_)^(2/3)), x_?IsSymbol] := With[{q = Rt[d/b, 3]}, -Sqrt[3]*q/d* ArcTan[2*q*(a + b*x)^(1/3)/(Sqrt[3]*(c + d*x)^(1/3)) + 1/Sqrt[3]] - q/(2*d)*Log[c + d*x] - 3*q/(2*d)*Log[q*(a + b*x)^(1/3)/(c + d*x)^(1/3) - 1]] /; FreeQ[{a, b, c, d}, x] && NeQ[b*c - a*d, 0] && PosQ[d/b]

        //Int[1/((a_. + b_.*x_)^(1/3)*(c_. + d_.*x_)^(2/3)), x_?IsSymbol] := With[{q = Rt[-d/b, 3]}, Sqrt[3]*q/d* ArcTan[1/Sqrt[3] - 2*q*(a + b*x)^(1/3)/(Sqrt[3]*(c + d*x)^(1/3))] + q/(2*d)*Log[c + d*x] + 3*q/(2*d)*Log[q*(a + b*x)^(1/3)/(c + d*x)^(1/3) + 1]] /; FreeQ[{a, b, c, d}, x] && NeQ[b*c - a*d, 0] && NegQ[d/b]

        //Int[(a_. + b_.*x_)^m_*(c_ + d_.*x_)^m_, x_?IsSymbol] := (a + b*x)^m*(c + d*x)^m/(a*c + (b*c + a*d)*x + b*d*x^2)^m* Int[(a*c + (b*c + a*d)*x + b*d*x^2)^m, x] /; FreeQ[{a, b, c, d}, x] && NeQ[b*c - a*d, 0] && LtQ[-1, m, 0] && LeQ[3, Denominator[m], 4] && AtomQ[b*c + a*d]

        //Int[(a_. + b_.*x_)^m_*(c_ + d_.*x_)^m_, x_?IsSymbol] := (a + b*x)^m*(c + d*x)^m/((a + b*x)*(c + d*x))^m* Int[(a*c + (b*c + a*d)*x + b*d*x^2)^m, x] /; FreeQ[{a, b, c, d}, x] && NeQ[b*c - a*d, 0] && LtQ[-1, m, 0] && LeQ[3, Denominator[m], 4]

        //Int[(a_. + b_.*x_)^m_*(c_. + d_.*x_)^n_, x_?IsSymbol] := With[{p = Denominator[m]}, p/b* Subst[Int[x^(p*(m + 1) - 1)*(c - a*d/b + d*x^p/b)^n, x], x, (a + b*x)^(1/p)]] /; FreeQ[{a, b, c, d}, x] && NeQ[b*c - a*d, 0] && LtQ[-1, m, 0] && LeQ[-1, n, 0] && LeQ[Denominator[n], Denominator[m]] && IntLinearQ[a, b, c, d, m, n, x]

        //Int[(b_.*x_)^m_*(c_ + d_.*x_)^n_, x_?IsSymbol] := c^n*(b*x)^(m + 1)/(b*(m + 1))* Hypergeometric2F1[-n, m + 1, m + 2, -d*x/c] /; FreeQ[{b, c, d, m, n}, x] && Not[IntegerQ[m]] && (IntegerQ[n] || GtQ[c, 0] && Not[EqQ[n, -1/2] && EqQ[c^2 - d^2, 0] && GtQ[-d/(b*c), 0]])

        //Int[(b_.*x_)^m_*(c_ + d_.*x_)^n_, x_?IsSymbol] := (c + d*x)^(n + 1)/(d*(n + 1)*(-d/(b*c))^m)* Hypergeometric2F1[-m, n + 1, n + 2, 1 + d*x/c] /; FreeQ[{b, c, d, m, n}, x] && Not[IntegerQ[n]] && (IntegerQ[m] || GtQ[-d/(b*c), 0])

        //Int[(b_.*x_)^m_*(c_ + d_.*x_)^n_, x_?IsSymbol] := c^IntPart[n]*(c + d*x)^FracPart[n]/(1 + d*x/c)^FracPart[n]* Int[(b*x)^m*(1 + d*x/c)^n, x] /; FreeQ[{b, c, d, m, n}, x] && Not[IntegerQ[m]] && Not[IntegerQ[n]] && Not[GtQ[c, 0]] && Not[GtQ[-d/(b*c), 0]] && (RationalQ[m] && Not[EqQ[n, -1/2] && EqQ[c^2 - d^2, 0]] || Not[RationalQ[n]])

        //Int[(b_.*x_)^m_*(c_ + d_.*x_)^n_, x_?IsSymbol] := (-b*c/d)^IntPart[m]*(b*x)^FracPart[m]/(-d*x/c)^FracPart[m]* Int[(-d*x/c)^m*(c + d*x)^n, x] /; FreeQ[{b, c, d, m, n}, x] && Not[IntegerQ[m]] && Not[IntegerQ[n]] && Not[GtQ[c, 0]] && Not[GtQ[-d/(b*c), 0]]

        // TODO: needs Not
        // Int[(a_ + b_.*x_)^m_*(c_ + d_.*x_)^n_?IsInteger, x_?IsSymbol]
        //     /; FreeQ[{a, b, c, d, m}, x] && NeQ[b*c - a*d, 0] && Not[IntegerQ[m]]
        //     :> (b*c - a*d)^n*(a + b*x)^(m + 1)/(b^(n + 1)*(m + 1))* Hypergeometric2F1[-n, m + 1, m + 2, -d*(a + b*x)/(b*c - a*d)];
    
        //Int[(a_ + b_.*x_)^m_*(c_ + d_.*x_)^n_, x_?IsSymbol] := (a + b*x)^(m + 1)/(b*(m + 1)*(b/(b*c - a*d))^n)* Hypergeometric2F1[-n, m + 1, m + 2, -d*(a + b*x)/(b*c - a*d)] /; FreeQ[{a, b, c, d, m, n}, x] && NeQ[b*c - a*d, 0] && Not[IntegerQ[m]] && Not[IntegerQ[n]] && GtQ[b/(b*c - a*d), 0] && (RationalQ[m] || Not[RationalQ[n] && GtQ[-d/(b*c - a*d), 0]])

        //Int[(a_ + b_.*x_)^m_*(c_ + d_.*x_)^n_, x_?IsSymbol] := (c + d*x)^ FracPart[ n]/((b/(b*c - a*d))^IntPart[n]*(b*(c + d*x)/(b*c - a*d))^ FracPart[n])* Int[(a + b*x)^m*Simp[b*c/(b*c - a*d) + b*d*x/(b*c - a*d), x]^n, x] /; FreeQ[{a, b, c, d, m, n}, x] && NeQ[b*c - a*d, 0] && Not[IntegerQ[m]] && Not[IntegerQ[n]] && (RationalQ[m] || Not[SimplerQ[n + 1, m + 1]])

        //Int[(a_. + b_.*u_)^m_.*(c_. + d_.*u_)^n_., x_?IsSymbol] := 1/Coefficient[u, x, 1]* Subst[Int[(a + b*x)^m*(c + d*x)^n, x], x, u] /; FreeQ[{a, b, c, d, m, n}, x] && LinearQ[u, x] && NeQ[Coefficient[u, x, 0], 0]
    }))
}
