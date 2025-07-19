/// Internal macro utils to handle strings and casings
#[macro_export]
macro_rules! str {
    ($s:expr $(, $ss:expr)*) => {{
        let mut r = $s.to_string();
        $(r.push_str(&$ss.to_string());)*
        r
    }};
}

/// Internal macro utils to handle strings and casings
#[macro_export]
macro_rules! ts2 {
    ($s:expr $(, $ss:expr)*) => {
        str!($s $(, $ss)*).parse::<TokenStream2>().unwrap()
    };
}

/// Internal macro utils to handle strings and casings
#[macro_export]
macro_rules! pascal_str {
    ($s:expr $(, $ss:expr)*) => {
        str!(str!($s).to_upper_camel_case() $(, str!($ss).to_upper_camel_case())*)
    };
}

/// Internal macro utils to handle strings and casings
#[macro_export]
macro_rules! pascal {
    ($s:expr $(, $ss:expr)*) => {
        ts2!(pascal_str!($s $(, $ss)*))
    };
}

/// Internal macro utils to handle strings and casings
#[macro_export]
macro_rules! camel_str {
    ($s:expr $(, $ss:expr)*) => {
        str!(str!($s).to_lower_camel_case() $(, str!($ss).to_upper_camel_case())*)
    };
}

/// Internal macro utils to handle strings and casings
#[macro_export]
macro_rules! camel {
    ($s:expr $(, $ss:expr)*) => {
        ts2!(camel_str!($s $(, $ss)*))
    };
}

/// Internal macro utils to handle strings and casings
#[macro_export]
macro_rules! snake_str {
    ($s:expr $(, $ss:expr)*) => {
        str!(str!($s).to_snake_case() $(, "_", str!($ss).to_snake_case())*)
    };
}

/// Internal macro utils to handle strings and casings
#[macro_export]
macro_rules! snake {
    ($s:expr $(, $ss:expr)*) => {
        ts2!(snake_str!($s $(, $ss)*))
    };
}
