use crate::{expr::Expr, pattern::{program::Compiler, runtime::Runtime}};


pub(super) fn count_matches(pattern: &Expr, subject: &Expr) -> usize {
    let program = Compiler::new().compile(pattern);
    let runtime = Runtime::new(&program, subject);
    runtime.count()
}
