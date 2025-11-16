/// Internal macro utils to create a new struct field from token stream.
#[macro_export]
macro_rules! field {
    ($($v:tt)*) => {{
        Parser::parse2(Field::parse_named, quote!($($v)*))
            .unwrap_or_else(|e| {
                bug!("Parser::parse2 Field::parse_named: {e}");
            })
    }};
}
