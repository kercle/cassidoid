use crate::{
    builtin::{ADD_HEAD, MUL_HEAD},
    expr::{NormExpr, pool::ExprPool},
    pattern::{environment::Environment, program::Compiler, runtime::Runtime},
};

pub(super) fn first_match<'p, 's>(
    program: &'p crate::pattern::program::Program,
    subject: &'s NormExpr,
) -> Option<Environment<'p>> {
    let mut pool = ExprPool::new();
    let handle = pool.insert_norm(subject);
    Runtime::new(program, &pool, handle).next()
}

pub(super) fn count_matches(pattern: &NormExpr, subject: &NormExpr) -> usize {
    let mut pool = ExprPool::new();
    let subject_handle = pool.insert_norm(subject);
    let pattern_handle = pool.insert_norm(pattern);

    let program = Compiler::new(&pool)
        .with_multiset_predicate(|pool, expr| {
            expr.view(pool).is_node(pool, "CommutativeOp", None)
                || expr.view(pool).is_node(pool, ADD_HEAD, None)
                || expr.view(pool).is_node(pool, MUL_HEAD, None)
        })
        .compile(pattern_handle);
    let runtime = Runtime::new(&program, &pool, subject_handle);
    runtime.count()
}
