use parser::parse;
use symbolics::{
    expr::RawExpr,
    format::MathDisplay,
    pattern::{environment::EnvBinding, program::Compiler},
};

fn main() {
    let [pattern, subject]: [String; 2] = std::env::args()
        .skip(1)
        .collect::<Vec<_>>()
        .try_into()
        .expect("Usage: '<pattern>' '<subject>'");

    let pattern: RawExpr = parse(&pattern).expect("Failed to parse pattern.").into();
    let subject: RawExpr = parse(&subject).expect("Failed to parse pattern.").into();

    let pattern = pattern.normalize();
    let subject = subject.normalize();

    let program = Compiler::default().compile(&pattern);

    for m in program.run(&subject) {
        println!("Subject matches with bindings:");

        for (name, binding) in m.iter() {
            print!("  {name} := ");
            match binding {
                EnvBinding::One(v) => println!("{}", v.as_raw_ref().to_input_form()),
                EnvBinding::Many(v) => {
                    let v = v
                        .iter()
                        .map(|e| e.as_raw_ref().to_input_form())
                        .collect::<Vec<String>>();
                    print!("[{}]", v.join(", "))
                }
            }
        }
    }
}
