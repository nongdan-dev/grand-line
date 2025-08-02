/// Internal macro utils to create a new struct field from token stream.
#[macro_export]
macro_rules! field {
    ($($v:tt)*) => {{
        use syn::{parse::Parser, Field};
        Parser::parse2(Field::parse_named, quote!($($v)*))
            .unwrap_or_else(|e| bug!("failed to parse field from token stream: {}", e))
    }};
}
