#[macro_export]
macro_rules! symbol {
    ( $($x:expr),* $(,)? ) => {
        ( $($crate::expr::generator::SymbolGenerator::new($x)),* )
    };
}
