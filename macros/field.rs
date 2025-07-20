#[macro_export]
macro_rules! field {
    ($($v:tt)*) => {{
        use syn::{parse::Parser, Field};
        Parser::parse2(Field::parse_named, quote!($($v)*)).unwrap()
    }};
}
