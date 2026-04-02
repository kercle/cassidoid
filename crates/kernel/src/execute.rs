use symbolics::{
    eval::{program::EvalProgram, runtime::EvalRuntime},
    expr::NormExpr,
    format::MathDisplay,
    norm_expr,
    pattern::{program::Compiler as PatternCompiler, runtime::Runtime as PatternRuntime},
};

use crate::{
    Kernel,
    api::{ExecutionError, ExecutionResult, ExpressionForms, KernelResult},
};

impl Kernel {
    fn execute_inner(&self, input: String) -> Result<ExecutionResult, ExecutionError> {
        let expr = self.eval(input.clone())?;

        if let Some(data) = generate_data_if_plot(&expr) {
            Ok(ExecutionResult::Plot {
                input: input.clone(),
                data,
            })
        } else if let Some(msg) = self.help(input.clone(), &expr) {
            Ok(msg)
        } else {
            let expr = expr.resugar();

            Ok(ExecutionResult::Expression {
                input: input.clone(),
                output: ExpressionForms {
                    raw: expr.to_input_form(),
                    latex: expr.to_latex_form(),
                },
            })
        }
    }

    pub fn execute(&self, input: String) -> KernelResult {
        match self.execute_inner(input) {
            Ok(v) => KernelResult::Ok(v),
            Err(v) => KernelResult::Err(v),
        }
    }

    pub fn help(&self, input: String, expr: &NormExpr) -> Option<ExecutionResult> {
        if expr == &norm_expr!(Help[]) {
            return Some(ExecutionResult::HelpTableOfContents {
                input,
                builtins: self.help_builtins(),
            });
        }

        let program = PatternCompiler::new().compile(&norm_expr!(
            Help[name_?IsSymbol]
        ));

        let mut runtime = PatternRuntime::new(&program, expr);
        let env = runtime.first_match()?;

        let builtin = self.get_builtin(env.get_one("name")?.get_symbol()?)?;
        let doc = builtin.doc();
        Some(ExecutionResult::HelpBuiltin {
            input,
            title: doc.title.to_string(),
            patterns: doc
                .pattern_doc
                .into_iter()
                .map(|e| (e.pattern.to_input_form(), e.summary))
                .collect(),
            examples: doc
                .examples
                .iter()
                .map(|(a, b)| (a.to_string(), b.to_string()))
                .collect(),
            related: doc.related.iter().map(|a| a.to_string()).collect(),
        })
    }
}

fn generate_data_if_plot(expr: &NormExpr) -> Option<Vec<(f64, f64)>> {
    let program = PatternCompiler::new().compile(&norm_expr!(
        Plot[f_, (x_?IsSymbol, x0_?IsNumber, x1_?IsNumber)]
    ));

    let mut runtime = PatternRuntime::new(&program, expr);
    let env = runtime.first_match()?;

    let f = env.get_one("f")?;
    let var = env.get_one("x")?.get_symbol()?;
    let x0 = env.get_one("x0")?.get_number()?.to_f64()?;
    let x1 = env.get_one("x1")?.get_number()?.to_f64()?;

    let program = EvalProgram::compile(f).ok()?;
    let mut runtime = EvalRuntime::new(&program);

    const N: u32 = 100;
    let mut data = Vec::new();
    for i in 0..=N {
        let x = x0 + (x1 - x0) * (i as f64) / (N as f64);
        let _ = runtime.bind_var(var, x);

        let y = runtime.execute();
        data.push((x, y.ok()?));
    }

    Some(data)
}
