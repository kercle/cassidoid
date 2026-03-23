use std::hash::{Hash, Hasher};

use crate::atom::Atom;

impl Hash for Atom {
    fn hash<H: Hasher>(&self, state: &mut H) {
        use Atom::*;

        match self {
            Number(n) => {
                0u8.hash(state);
                n.hash(state);
            }
            Symbol(s) => {
                1u8.hash(state);
                s.hash(state);
            }
            StringLiteral(v) => {
                2u8.hash(state);
                v.hash(state);
            }
            Boolean(b) => {
                3u8.hash(state);
                b.hash(state);
            }
        }
    }
}
