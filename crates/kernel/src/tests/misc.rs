use parser::parse;
use symbolics::expr::RawExpr;

use crate::Kernel;

#[test]
fn test_examples() {
    let kernel = Kernel::default();
    for builtin in kernel.builtins.iter() {
        let doc = builtin.doc();

        for (input, expected_output) in doc.examples {
            let res = kernel
                .eval(input)
                .expect(&format!("Input `{input}` cannot be evaluated."));

            let ast_exp_out = parse(expected_output.as_ref()).expect(&format!(
                "Expected output `{expected_output}` cannot be parsed."
            ));

            assert_eq!(RawExpr::from(ast_exp_out).normalize(), res);
        }
    }
}
