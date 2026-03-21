use std::collections::HashMap;

use parser::parse;
use symbolics::{
    expr::RawExpr,
    pattern::program::{ArgPlan, Compiler, InstrId, Instruction, Program},
};

struct InstructionGraph<'p> {
    nodes: HashMap<String, HashMap<&'static str, String>>,
    edges: Vec<(String, String)>,
    program: &'p Program,
}

impl<'p> InstructionGraph<'p> {
    pub fn new(program: &'p Program) -> Self {
        let mut ret = Self {
            nodes: HashMap::new(),
            edges: Vec::new(),
            program,
        };

        ret.enter();

        ret
    }

    pub fn to_dot(&self) -> String {
        let mut output = String::new();
        output.push_str("digraph G {\n");

        for (node_id, opts) in self.nodes.iter() {
            output.push_str(&format!("{node_id}["));
            let opts_str: Vec<String> = opts
                .iter()
                .map(|(key, value)| format!(r#"{key}="{value}""#))
                .collect();
            output.push_str(&opts_str.join(", ").to_string());
            output.push_str("];\n");
        }

        for (a, b) in self.edges.iter() {
            output.push_str(&format!("{a} -> {b};\n"));
        }

        output.push_str("}");

        output
    }

    fn bind_to(&self, bind: Option<u32>) -> String {
        let Some(bind) = bind else {
            return "".to_string();
        };

        let bind = self.program.var(bind).unwrap();
        format!("{bind} := ")
    }

    fn enter(&mut self) {
        let mut node_opts = HashMap::new();
        let entry_node_id = format!("entry");

        node_opts.insert("shape", "circle".to_string());
        node_opts.insert("label", "Start".to_string());
        self.nodes.insert(entry_node_id.clone(), node_opts);

        self.walk_program(self.program.entry(), entry_node_id);
    }

    fn walk_program(&mut self, cur_instr: InstrId, prev_node: String) {
        use Instruction::*;

        let mut node_opts = HashMap::new();
        let mut current_node_id = format!("i{cur_instr}");

        match self.program.instruction(cur_instr).unwrap() {
            Literal { inner, bind } => {
                let bind = self.bind_to(*bind);
                node_opts.insert("label", format!("{bind}Literal: `{inner:?}`"));
            }
            Variadic {
                min_len,
                head_pattern: _,
                bind,
            } => {
                let bind = self.bind_to(*bind);
                let label = format!("{bind}Variadic({min_len})");

                node_opts.insert("shape", "box".to_string());
                node_opts.insert("label", label);
            }
            Wildcard {
                head_pattern: _,
                bind,
            } => {
                let bind = self.bind_to(*bind);
                let label = format!("{bind}Wildcard");

                node_opts.insert("shape", "box".to_string());
                node_opts.insert("label", label);
            }
            Predicate { .. } => todo!(),
            Node { head, plan, bind } => {
                let bind = self.bind_to(*bind);
                let mut label = format!("<n> {bind}Node ");
                current_node_id.push_str(":n");

                let args = match plan {
                    ArgPlan::Multiset(args) => {
                        label.push_str(&format!("Multiset"));
                        args
                    }
                    ArgPlan::Sequence(args) => {
                        label.push_str(&format!("Sequence"));
                        args
                    }
                };

                label.push_str(&format!(" | <h> Head"));

                for (slot_index, &next_instr) in args.iter().enumerate() {
                    self.walk_program(next_instr, format!("i{cur_instr}:a{slot_index}"));
                    label.push_str(&format!(" | <a{slot_index}> Arg {slot_index}"));
                }

                node_opts.insert("shape", "record".to_string());
                node_opts.insert("label", label);
                self.walk_program(*head, format!("i{cur_instr}:h"));
            }
            Alternatives { branches } => {
                let mut label = format!("<n> Alternatives ");

                for (slot_index, (pattern_id, next_instr)) in branches.iter().enumerate() {
                    self.walk_program(*next_instr, format!("i{cur_instr}:a{slot_index}"));
                    label.push_str(&format!(
                        " | <a{slot_index}> Branch({pattern_id}) {slot_index}"
                    ));
                }

                node_opts.insert("shape", "record".to_string());
                node_opts.insert("label", label);
            }
        }

        self.nodes.insert(current_node_id.clone(), node_opts);
        self.edges.push((prev_node, current_node_id));
    }
}

fn main() {
    let mut programs = Vec::new();

    for arg in std::env::args().skip(1) {
        let ast = parse(&arg).expect(&format!("Cannot compile pattern `{arg}`"));

        let expr: RawExpr = ast.into();
        let program = Compiler::default().compile(&expr.normalize());

        programs.push(program);
    }

    if let Some(program) = programs.get(0) {
        let graph = InstructionGraph::new(program);
        println!("{}", graph.to_dot());
    } else {
        eprintln!("No pattern specified.");
    }
}
