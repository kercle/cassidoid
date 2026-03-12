pub mod atom;
pub mod builtin;
pub mod calculus;
pub mod eval;
pub mod expr;
pub mod format;
pub mod macros;
pub mod pattern;
pub mod poly;
pub mod rewrite;
pub mod simplify;

#[cfg(test)]
mod tests;

// for proc macros: route numbers through parser crate
#[doc(hidden)]
pub use parser_macros as _parser_macros;
