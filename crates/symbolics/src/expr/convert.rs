use crate::{atom::Atom, expr::Expr};

impl<A, T: Into<Atom>> From<T> for Expr<A>
where
    A: Default,
{
    fn from(x: T) -> Self {
        Expr::Atom {
            entry: x.into(),
            annotation: A::default(),
        }
    }
}
