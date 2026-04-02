pub mod api;
pub mod execute;

mod register;

#[cfg(test)]
mod tests;

use parser::parse;
use symbolics::{
    builtins::traits::BuiltIn,
    expr::{NormExpr, RawExpr},
};

use crate::api::ExecutionError;

pub struct Kernel {
    builtins: Vec<Box<dyn BuiltIn + Send>>,
    auto_apply: Vec<usize>,
}

impl Default for Kernel {
    fn default() -> Self {
        let mut result = Self {
            builtins: Vec::new(),
            auto_apply: Vec::new(),
        };

        result.register_initial_builtins();
        result
    }
}

impl Kernel {
    pub fn register_builtin_default<B: BuiltIn + Default + Send + 'static>(
        &mut self,
        auto_apply: bool,
    ) {
        self.register_builtin(Box::new(B::default()), auto_apply);
    }

    pub fn register_builtin(&mut self, builtin: Box<dyn BuiltIn + Send>, auto_apply: bool) {
        let id = self.builtins.len();
        self.builtins.push(builtin);

        if auto_apply {
            self.set_auto_apply_by_id(id);
        }
    }

    pub fn get_builtin(&self, head_name: impl AsRef<str>) -> Option<&(dyn BuiltIn + Send)> {
        self.builtins
            .iter()
            .find(|b| b.head_dyn() == head_name.as_ref())
            .map(|v| &**v)
    }

    fn set_auto_apply_by_id(&mut self, id: usize) {
        if !self.auto_apply.contains(&id) {
            self.auto_apply.push(id);
        }
    }

    pub fn set_auto_apply(&mut self, head_name: impl AsRef<str>) -> Result<(), ExecutionError> {
        let Some(id) = self.get_builtin_id(head_name) else {
            return Err(ExecutionError::UnknownBuiltIn);
        };

        self.set_auto_apply_by_id(id);

        Ok(())
    }

    // TODO: This is just a quick & dirty function for
    // table of contents. Should be improved.
    pub fn help_builtins(&self) -> Vec<(String, String, String)> {
        self.builtins
            .iter()
            .map(|b| {
                let doc = b.doc();
                (
                    b.head_dyn().to_string(),
                    doc.summary.to_string(),
                    doc.category.to_string(),
                )
            })
            .collect()
    }

    fn get_builtin_id(&self, head_name: impl AsRef<str>) -> Option<usize> {
        self.builtins
            .iter()
            .position(|b| b.head_dyn() == head_name.as_ref())
    }

    pub fn eval(&self, input: impl AsRef<str>) -> Result<NormExpr, ExecutionError> {
        let ast_in = parse(input.as_ref()).map_err(|err| ExecutionError::EvaluationError {
            msg: err.to_string(),
        })?;

        let input_expr = RawExpr::from(ast_in);
        Ok(self.apply_auto_builtins_until_stable(input_expr.normalize(), 100))
    }

    fn apply_auto_builtins_until_stable(&self, mut expr: NormExpr, limit: usize) -> NormExpr {
        // For now we just rely on the fingerprint for stabelization.
        let mut current_fingerprint = expr.fingerprint();

        for _ in 0..limit {
            for index in self.auto_apply.iter() {
                let builtin = self.builtins.get(*index).expect("Builtin not registered");
                expr = builtin.apply_all(expr);
            }

            if expr.fingerprint() == current_fingerprint {
                break;
            } else {
                current_fingerprint = expr.fingerprint();
            }
        }

        expr
    }
}
