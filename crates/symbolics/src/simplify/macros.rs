#[macro_export]
macro_rules! rw_rule {
    // Match: rw_rule!(pattern => |bindings| { body })
    ($pattern:expr => |$($var:ident),+| $body:block) => {
        (
            $pattern,
            Box::new(|bind: &BindingType| {
                $(
                    let $var = bind.get(stringify!($var)).unwrap().clone();
                )+
                $body
            }),
        )
    };
    ($pattern:expr => |$($var:ident),+| $body:expr) => {
        rw_rule!($pattern => |$($var),+| { $body })
    };
}
