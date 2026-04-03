use std::fmt::Write;

use crate::atom::Atom;
use crate::builtins;
use crate::builtins::traits::BuiltIn;
use crate::expr::{ExprKind, RawExpr};
use numbers::Number;

// Determines the placement of paranthesis, depending on
// which position the term is placed at.
#[derive(Clone, Copy)]
enum Position {
    Root,
    AddOperand, // Either operand of Add
    SubLhs,     // Left-hand side of Sub
    SubRhs,     // Right-hand side of Sub; needs parens for add/sub/neg
    MulOperand, // Any operand of Mul
    DivChild,   // Numerator or denominator of Div
    FactArg,    // Factorial argument
    PowBase,    // Base of Pow
    PowExp,     // Exponent of Pow
    NegOperand, // Operand of unary Neg
    FnArg,      // Argument of a function
}

fn needs_parens(expr: &RawExpr, pos: Position) -> bool {
    match pos {
        // No parens necessary for these:
        Position::Root
        | Position::DivChild
        | Position::PowExp
        | Position::FnArg
        | Position::AddOperand
        | Position::SubLhs => false,

        Position::SubRhs => {
            // a-(b+c),  a-(b-c),  a-(-c)
            builtins::Add::is_application(expr)
                || builtins::Sub::is_application(expr)
                || builtins::Neg::is_application(expr)
        }

        Position::NegOperand => {
            // -(a+b),  -(a-b)
            builtins::Add::is_application(expr) || builtins::Sub::is_application(expr)
        }

        Position::MulOperand => {
            // (a+b)*c
            builtins::Add::is_application(expr) || builtins::Sub::is_application(expr)
        }

        Position::PowBase => {
            // (a+b)^n,  (a*b)^n,  (a/b)^n,  (-a)^n
            builtins::Add::is_application(expr)
                || builtins::Sub::is_application(expr)
                || builtins::Mul::is_application(expr)
                || builtins::Div::is_application(expr)
                || builtins::Neg::is_application(expr)
        }

        Position::FactArg => {
            // (a+b)!,  (a*b)!,  (a/b)!,  (-a)!
            builtins::Add::is_application(expr)
                || builtins::Sub::is_application(expr)
                || builtins::Mul::is_application(expr)
                || builtins::Div::is_application(expr)
                || builtins::Neg::is_application(expr)
        }
    }
}

fn greek_letter(name: &str) -> String {
    match name {
        "alpha" | "α" => "&alpha;",
        "beta" | "β" => "&beta;",
        "gamma" | "γ" => "&gamma;",
        "delta" | "δ" => "&delta;",
        "epsilon" | "ε" => "&epsilon;",
        "zeta" | "ζ" => "&zeta;",
        "eta" | "η" => "&eta;",
        "theta" | "θ" => "&theta;",
        "iota" | "ι" => "&iota;",
        "kappa" | "κ" => "&kappa;",
        "lambda" | "λ" => "&lambda;",
        "mu" | "μ" => "&mu;",
        "nu" | "ν" => "&nu;",
        "xi" | "ξ" => "&xi;",
        "omicron" | "ο" => "&omicron;",
        "pi" | "π" => "&pi;",
        "rho" | "ρ" => "&rho;",
        "sigma" | "σ" | "ς" => "&sigma;",
        "tau" | "τ" => "&tau;",
        "upsilon" | "υ" => "&upsilon;",
        "phi" | "φ" | "ϕ" => "&phi;",
        "chi" | "χ" => "&chi;",
        "psi" | "ψ" => "&psi;",
        "omega" | "ω" => "&omega;",
        _ => return name.to_string(),
    }
    .to_string()
}

struct MathMlWriter {
    lines: Vec<(usize, String)>,
    indent: usize,
}

impl MathMlWriter {
    pub fn new() -> Self {
        let mut obj = Self {
            lines: Vec::new(),
            indent: 0,
        };

        obj.start_tag_with_attrs(
            "math",
            &[
                ("xmlns", "http://www.w3.org/1998/Math/MathML"),
                ("displaystyle", "true"),
            ],
        );
        obj
    }

    fn start_tag_with_attrs(&mut self, name: &str, attrs: &[(&str, &str)]) {
        let mut buf = format!("<{name}");

        for (k, v) in attrs {
            let _ = write!(buf, r#" {k}="{v}""#);
        }

        buf.push('>');

        self.lines.push((self.indent, buf));
        self.indent += 1;
    }

    fn start_tag(&mut self, name: &str) {
        self.start_tag_with_attrs(name, &[]);
    }

    fn end_tag(&mut self, name: &str) {
        self.indent -= 1;
        self.lines.push((self.indent, format!("</{name}>")));
    }

    fn tag<F>(&mut self, name: &str, f: F)
    where
        F: FnOnce(&mut Self),
    {
        self.start_tag(name);
        f(self);
        self.end_tag(name);
    }

    fn text(&mut self, content: &str) {
        self.lines.push((self.indent, content.to_string()));
    }

    fn tagged_line(&mut self, name: &str, content: &str) {
        self.lines
            .push((self.indent, format!("<{name}>{content}</{name}>")));
    }

    fn mn(&mut self, content: &str) {
        self.tagged_line("mn", content);
    }

    fn mo(&mut self, content: &str) {
        self.tagged_line("mo", content);
    }

    fn mi(&mut self, content: &str) {
        self.tagged_line("mi", content);
    }

    fn mtext(&mut self, content: &str) {
        self.tagged_line("mtext", content);
    }

    pub fn minified(mut self) -> String {
        self.end_tag("math");
        let lines: Vec<_> = self.lines.into_iter().map(|(_, s)| s).collect();
        lines.join("")
    }

    #[allow(dead_code)]
    pub fn finish(mut self) -> String {
        self.end_tag("math");
        let lines: Vec<_> = self
            .lines
            .into_iter()
            .map(|(i, s)| "    ".repeat(i) + &s)
            .collect();
        lines.join("\n")
    }

    // ---- Rendering ----

    fn render_one_arg_well_known(&mut self, cmd: &str, arg: &RawExpr) {
        self.tag("mrow", |w| {
            w.start_tag_with_attrs("mi", &[("mathvariant", "normal")]);
            w.text(cmd);
            w.end_tag("mi");
            w.mo("&af;");
            w.tag("mrow", |w| {
                w.mo("(");
                w.expr_to_mathml_with_pos(arg, Position::FnArg);
                w.mo(")");
            });
        });
    }

    fn render_derivative(&mut self, args: &[RawExpr]) {
        let (f, x) = (&args[0], &args[1]);

        if let Some(x) = x.get_symbol() {
            self.tag("mfrac", |w| {
                w.mo("&dd;");
                w.tag("mrow", |w| {
                    w.mo("&dd;");
                    w.mi(x);
                });
            });
            self.tag("mrow", |w| {
                w.mo("(");
                w.expr_to_mathml_inner(f);
                w.mo(")");
            });
        } else {
            self.render_generic_node(builtins::Derivative::head(), args);
        }
    }

    fn render_integrate(&mut self, args: &[RawExpr]) {
        let (f, x) = (&args[0], &args[1]);

        if let Some(x) = x.get_symbol() {
            self.tag("mrow", |w| {
                w.start_tag_with_attrs("mo", &[("largeop", "true"), ("symmetric", "true")]);
                w.text("&int;");
                w.end_tag("mo");

                w.expr_to_mathml_with_pos(f, Position::FnArg);

                w.start_tag_with_attrs("mo", &[("mathvariant", "normal")]);
                w.text("d");
                w.end_tag("mo");
                w.mi(x);
            });
        } else {
            self.render_generic_node(builtins::Integrate::head(), args);
        }
    }

    fn render_generic_node(&mut self, head_name: &str, args: &[RawExpr]) {
        self.tag("mrow", |w| {
            w.start_tag_with_attrs("mi", &[("mathvariant", "normal")]);
            w.text(head_name);
            w.end_tag("mi");
            w.mo("&af;");
            w.tag("mrow", |w| {
                w.mo("[");

                if let Some(arg) = args.first() {
                    w.expr_to_mathml_with_pos(arg, Position::FnArg)
                }

                for arg in args.iter().skip(1) {
                    w.mo(",");
                    w.expr_to_mathml_with_pos(arg, Position::FnArg);
                }

                w.mo("]");
            });
        });
    }

    fn expr_to_mathml_with_pos(&mut self, expr: &RawExpr, pos: Position) {
        if needs_parens(expr, pos) {
            self.tag("mrow", |w| {
                w.mo("(");
                w.expr_to_mathml_inner(expr);
                w.mo(")");
            });
        } else {
            self.expr_to_mathml_inner(expr)
        }
    }

    fn expr_to_mathml_inner(&mut self, expr: &RawExpr) {
        match expr.kind() {
            ExprKind::Atom {
                entry: Atom::Number(Number::Integer(value)),
                ..
            } => self.mn(&value.to_string()),

            ExprKind::Atom {
                entry: Atom::Number(Number::Rational(_)),
                ..
            } => unimplemented!("Should have been resugared to Div"),

            ExprKind::Atom {
                entry: Atom::Symbol(name),
                ..
            } if name == builtins::symbols::INDETERMINATE => self.mtext(name),

            ExprKind::Atom {
                entry: Atom::Symbol(name),
                ..
            } if name == builtins::symbols::INFINITY => self.mo("&infin;"),

            ExprKind::Atom {
                entry: Atom::Symbol(name),
                ..
            } => self.mi(&greek_letter(name)),

            ExprKind::Atom {
                entry: Atom::Boolean(value),
                ..
            } => {
                if *value {
                    self.mtext(builtins::symbols::TRUE)
                } else {
                    self.mtext(builtins::symbols::FALSE)
                }
            }

            ExprKind::Atom {
                entry: Atom::StringLiteral(_),
                ..
            } => unimplemented!(),

            ExprKind::Node { args, .. } if builtins::Neg::is_application(expr) => {
                self.mo("-");
                self.expr_to_mathml_with_pos(&args[0], Position::NegOperand)
            }

            ExprKind::Node { args, .. } if builtins::Add::is_application(expr) => {
                if let Some(a) = args.first() {
                    self.expr_to_mathml_with_pos(a, Position::AddOperand);
                }
                for a in args.iter().skip(1) {
                    self.mo("+");
                    self.expr_to_mathml_with_pos(a, Position::AddOperand);
                }
            }

            ExprKind::Node { args, .. } if builtins::Sub::is_application(expr) => {
                self.expr_to_mathml_with_pos(&args[0], Position::SubLhs);
                self.mo("-");
                self.expr_to_mathml_with_pos(&args[1], Position::SubRhs);
            }

            ExprKind::Node { args, .. } if builtins::Mul::is_application(expr) => {
                if let Some(a) = args.first() {
                    self.expr_to_mathml_with_pos(a, Position::MulOperand);
                }
                for a in args.iter().skip(1) {
                    self.mo("&middot;");
                    self.expr_to_mathml_with_pos(a, Position::MulOperand);
                }
            }

            ExprKind::Node { args, .. } if builtins::Div::is_application(expr) => {
                self.tag("mfrac", |w| {
                    w.tag("mrow", |w| {
                        w.expr_to_mathml_with_pos(&args[0], Position::DivChild);
                    });
                    w.tag("mrow", |w| {
                        w.expr_to_mathml_with_pos(&args[1], Position::DivChild);
                    });
                })
            }

            ExprKind::Node { args, .. } if builtins::Pow::is_application(expr) => {
                self.tag("msup", |w| {
                    w.tag("mrow", |w| {
                        w.expr_to_mathml_with_pos(&args[0], Position::PowBase);
                    });
                    w.tag("mrow", |w| {
                        w.expr_to_mathml_with_pos(&args[1], Position::PowExp);
                    });
                })
            }

            ExprKind::Node { args, .. } if builtins::Factorial::is_application(expr) => {
                self.expr_to_mathml_with_pos(&args[0], Position::FactArg);
                self.mo("!");
            }

            ExprKind::Node { args, .. } if builtins::Sqrt::is_application(expr) => {
                self.tag("msqrt", |w| {
                    w.expr_to_mathml_with_pos(&args[0], Position::FnArg);
                });
            }

            ExprKind::Node { args, .. } if builtins::Exp::is_application(expr) => {
                self.render_one_arg_well_known("exp", &args[0])
            }
            ExprKind::Node { args, .. } if builtins::Log::is_application(expr) => {
                self.render_one_arg_well_known("log", &args[0])
            }
            ExprKind::Node { args, .. } if builtins::Sin::is_application(expr) => {
                self.render_one_arg_well_known("sin", &args[0])
            }
            ExprKind::Node { args, .. } if builtins::Cos::is_application(expr) => {
                self.render_one_arg_well_known("cos", &args[0])
            }
            ExprKind::Node { args, .. } if builtins::Tan::is_application(expr) => {
                self.render_one_arg_well_known("tan", &args[0])
            }

            ExprKind::Node { args, .. } if builtins::Derivative::is_application(expr) => {
                self.render_derivative(args)
            }

            ExprKind::Node { args, .. } if builtins::Integrate::is_application(expr) => {
                self.render_integrate(args)
            }

            ExprKind::Node { head, args, .. } if head.matches_symbol(builtins::Tuple::head()) => {
                self.start_tag("mrow");
                self.mo("(");

                if let Some(a) = args.first() {
                    self.expr_to_mathml_with_pos(a, Position::MulOperand);
                }
                for a in args.iter().skip(1) {
                    self.mo(",");
                    self.expr_to_mathml_with_pos(a, Position::MulOperand);
                }

                self.mo(")");
                self.end_tag("mrow");
            }

            ExprKind::Node { head, args } => {
                let Some(name) = head.get_symbol() else {
                    unimplemented!()
                };
                self.render_generic_node(name, args)
            }
        }
    }

    pub fn render(mut self, expr: &RawExpr) -> Self {
        self.expr_to_mathml_with_pos(expr, Position::Root);
        self
    }
}

pub fn render(expr: &RawExpr) -> String {
    MathMlWriter::new().render(expr).minified()
}

#[cfg(test)]
mod tests {
    use super::MathMlWriter;
    use crate::expr::RawExpr;
    use parser::parse;

    fn clean(s: &str) -> String {
        s.trim()
            .lines()
            .map(|line| line.trim_start()) // Removes the "code" indentation
            .collect::<Vec<_>>()
            .join("\n")
    }

    #[test]
    fn test_render() {
        let expr = RawExpr::from(parse("2 + 3").unwrap());
        let expected = clean(
            r#"
            <math xmlns="http://www.w3.org/1998/Math/MathML">
                <mn>2</mn>
                <mo>+</mo>
                <mn>3</mn>
            </math>
        "#,
        );
        assert_eq!(clean(&MathMlWriter::new().render(&expr).finish()), expected);
    }

    #[test]
    fn test_render_with_parenthesis() {
        let expr = RawExpr::from(parse("(2 + 3) * 6").unwrap());
        let expected = clean(
            r#"
            <math xmlns="http://www.w3.org/1998/Math/MathML">
                <mrow>
                    <mo>(</mo>
                    <mn>2</mn>
                    <mo>+</mo>
                    <mn>3</mn>
                    <mo>)</mo>
                </mrow>
                <mo>&middot;</mo>
                <mn>6</mn>
            </math>
        "#,
        );
        assert_eq!(clean(&MathMlWriter::new().render(&expr).finish()), expected);
    }

    #[test]
    fn test_render_multiple_adds() {
        let expr = RawExpr::from(parse("1+2+3+4").unwrap());
        let expected = clean(
            r#"
            <math xmlns="http://www.w3.org/1998/Math/MathML">
                <mn>1</mn>
                <mo>+</mo>
                <mn>2</mn>
                <mo>+</mo>
                <mn>3</mn>
                <mo>+</mo>
                <mn>4</mn>
            </math>
        "#,
        );
        assert_eq!(clean(&MathMlWriter::new().render(&expr).finish()), expected);
    }

    #[test]
    fn test_render_with_unary_op() {
        let expr = RawExpr::from(parse("-2 + 3").unwrap());
        let expected = clean(
            r#"
            <math xmlns="http://www.w3.org/1998/Math/MathML">
                <mo>-</mo>
                <mn>2</mn>
                <mo>+</mo>
                <mn>3</mn>
            </math>
        "#,
        );
        assert_eq!(clean(&MathMlWriter::new().render(&expr).finish()), expected);
    }

    #[test]
    fn test_render_with_pow() {
        let expr = RawExpr::from(parse("pi^2").unwrap());
        let expected = clean(
            r#"
            <math xmlns="http://www.w3.org/1998/Math/MathML">
                <msup>
                    <mrow>
                        <mi>&pi;</mi>
                    </mrow>
                    <mrow>
                        <mn>2</mn>
                    </mrow>
                </msup>
            </math>
        "#,
        );
        assert_eq!(clean(&MathMlWriter::new().render(&expr).finish()), expected);
    }

    #[test]
    fn test_render_with_function_call() {
        let expr = RawExpr::from(parse("5*pi^2/4*Cos[pi*x/2]*Sin[π*y/2]").unwrap());
        let expected = clean(
            r#"
            <math xmlns="http://www.w3.org/1998/Math/MathML">
                <mfrac>
                    <mrow>
                        <mn>5</mn>
                        <mo>&middot;</mo>
                        <msup>
                            <mrow>
                            <mi>&pi;</mi>
                            </mrow>
                            <mrow>
                            <mn>2</mn>
                            </mrow>
                        </msup>
                    </mrow>
                    <mrow>
                        <mn>4</mn>
                    </mrow>
                </mfrac>
                <mo>&middot;</mo>
                <mrow>
                    <mi mathvariant="normal">
                    cos
                    </mi>
                    <mo>&af;</mo>
                    <mrow>
                        <mo>(</mo>
                        <mfrac>
                            <mrow>
                                <mi>&pi;</mi>
                                <mo>&middot;</mo>
                                <mi>x</mi>
                            </mrow>
                            <mrow>
                                <mn>2</mn>
                            </mrow>
                        </mfrac>
                        <mo>)</mo>
                    </mrow>
                </mrow>
                <mo>&middot;</mo>
                <mrow>
                    <mi mathvariant="normal">
                    sin
                    </mi>
                    <mo>&af;</mo>
                    <mrow>
                        <mo>(</mo>
                        <mfrac>
                            <mrow>
                                <mi>&pi;</mi>
                                <mo>&middot;</mo>
                                <mi>y</mi>
                            </mrow>
                            <mrow>
                                <mn>2</mn>
                            </mrow>
                        </mfrac>
                        <mo>)</mo>
                    </mrow>
                </mrow>
            </math>
        "#,
        );
        assert_eq!(clean(&MathMlWriter::new().render(&expr).finish()), expected);
    }

    // New tests exercising position-sensitive parenthesization
    #[test]
    fn test_sub_rhs_parens() {
        let expr = RawExpr::from(parse("a - (b + c)").unwrap());
        let expected = clean(
            r#"
            <math xmlns="http://www.w3.org/1998/Math/MathML">
                <mi>a</mi>
                <mo>-</mo>
                <mrow>
                    <mo>(</mo>
                    <mi>b</mi>
                    <mo>+</mo>
                    <mi>c</mi>
                    <mo>)</mo>
                </mrow>
            </math>
        "#,
        );
        assert_eq!(clean(&MathMlWriter::new().render(&expr).finish()), expected);
    }

    #[test]
    fn test_sub_rhs_sub_parens() {
        let expr = RawExpr::from(parse("a - (b - c)").unwrap());
        let expected = clean(
            r#"
            <math xmlns="http://www.w3.org/1998/Math/MathML">
                <mi>a</mi>
                <mo>-</mo>
                <mrow>
                    <mo>(</mo>
                    <mi>b</mi>
                    <mo>-</mo>
                    <mi>c</mi>
                    <mo>)</mo>
                </mrow>
            </math>
        "#,
        );
        assert_eq!(clean(&MathMlWriter::new().render(&expr).finish()), expected);
    }

    #[test]
    fn test_pow_base_sum_parens() {
        let expr = RawExpr::from(parse("(a + b)^2").unwrap());
        let expected = clean(
            r#"
            <math xmlns="http://www.w3.org/1998/Math/MathML">
                <msup>
                    <mrow>
                        <mrow>
                            <mo>(</mo>
                            <mi>a</mi>
                            <mo>+</mo>
                            <mi>b</mi>
                            <mo>)</mo>
                        </mrow>
                    </mrow>
                    <mrow>
                        <mn>2</mn>
                    </mrow>
                </msup>
            </math>
        "#,
        );
        assert_eq!(clean(&MathMlWriter::new().render(&expr).finish()), expected);
    }

    #[test]
    fn test_neg_sum_parens() {
        let expr = RawExpr::from(parse("-(a + b)").unwrap());
        let expected = clean(
            r#"
            <math xmlns="http://www.w3.org/1998/Math/MathML">
                <mo>-</mo>
                <mrow>
                    <mo>(</mo>
                    <mi>a</mi>
                    <mo>+</mo>
                    <mi>b</mi>
                    <mo>)</mo>
                </mrow>
            </math>
        "#,
        );
        assert_eq!(clean(&MathMlWriter::new().render(&expr).finish()), expected);
    }
}
