/// should only use in macro builder since it might not efficient
#[macro_export]
macro_rules! join_str {
    ($($s:expr),*) => {{
        let mut r = String::new();
        $(r.push_str(&$s.to_string());)*
        r
    }};
}

/// should only use in macro builder since it might not efficient
#[macro_export]
macro_rules! pascal_str {
    ($($s:expr),*) => {{
        use heck::ToUpperCamelCase;
        join_str!($($s.to_string().to_upper_camel_case()),*)
    }};
}

/// should only use in macro builder since it might not efficient
#[macro_export]
macro_rules! pascal {
    ($($s:expr),*) => {{
        use proc_macro2::TokenStream as TokenStream2;
        pascal_str!($($s),*).parse::<TokenStream2>().unwrap()
    }};
}
