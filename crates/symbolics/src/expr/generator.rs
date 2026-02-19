use crate::expr::Expr;

pub trait ExprBuilder {
    fn build(&self) -> Expr<()>;
}

pub struct SymbolGenerator {
    name: String,
}

impl SymbolGenerator {
    pub fn new<T: AsRef<str>>(name: T) -> SymbolGenerator {
        SymbolGenerator {
            name: name.as_ref().to_string(),
        }
    }
}

impl ExprBuilder for SymbolGenerator {
    fn build(&self) -> Expr {
        Expr::new_symbol(&self.name)
    }
}

pub fn f() -> Expr<()> {
    Expr::new_compound(Expr::new_symbol("f"), Vec::new())
}

pub fn g(a: Expr) -> Expr<()> {
    Expr::new_compound(Expr::new_symbol("f"), vec![a])
}

pub fn h(a: Expr, b: Expr) -> Expr<()> {
    Expr::new_compound(Expr::new_symbol("f"), vec![a, b])
}

pub fn cos(a: Expr) -> Expr<()> {
    Expr::new_compound(Expr::new_symbol("Cos"), vec![a])
}

pub fn sin(a: Expr) -> Expr<()> {
    Expr::new_compound(Expr::new_symbol("Sin"), vec![a])
}

pub fn exp(a: Expr) -> Expr<()> {
    Expr::new_compound(Expr::new_symbol("Exp"), vec![a])
}
