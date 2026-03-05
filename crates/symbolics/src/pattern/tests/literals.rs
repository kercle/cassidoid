// ------------------------------------------------------------------------------------------------
// LITERAL TESTS
// ------------------------------------------------------------------------------------------------
//
//  Pattern                             | Test Expr                           | Expected Matches
//  ------------------------------------|-------------------------------------|------------------
//  5                                   | 5                                   | 1
//  5                                   | 6                                   | 0
//  foo                                 | foo                                 | 0
//  f[1, 2, 3]                          | f[1, 2, 3]                          | 1
//  f[1, 2, 3]                          | f[1, 2, 4]                          | 0
//  f[1, 2, 3]                          | f[1, 2]                             | 0
//  f[1, 2, 3]                          | f[3, 2, 1]                          | 0
//  f[g[1, 2], h[3, 4]]                 | f[g[1, 2], h[3, 4]]                 | 1
//  f[g[1, 2], h[3, 4]]                 | f[g[1, 2], h[3, 5]]                 | 0
//  f[g[h[1]]]                          | f[g[h[1]]]                          | 1
//  f[g[h[1]]]                          | f[g[h[2]]]                          | 0
//  f[]                                 | f[]                                 | 1
//  f[]                                 | f[1]                                | 0
//  f[]                                 | g[]                                 | 0

use crate::atom::Atom;
use crate::expr::Expr;
use crate::pattern::program::Compiler;
use crate::pattern::runtime::Runtime;
use expr_macro::expr;

#[test]
fn test_pattern_literal_5() {
    let pattern = expr! { 5 };
    let subject = expr! { 5 };
    let expected_count = 1;

    let program = Compiler::new().compile(&pattern);

    let mut runtime = Runtime::new(&program, &subject);

    runtime.next_match().expect("should match");
}
