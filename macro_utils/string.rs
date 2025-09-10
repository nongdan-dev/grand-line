/// Internal macro utils to handle strings and casings.
#[macro_export]
macro_rules! s {
    () => {
        String::new()
    };
    ($($s:expr),*) => {{
        let mut r = s!();
        $(r.push_str($s.to_string().as_ref());)*
        r
    }};
}

/// Internal macro utils to handle strings and casings.
#[macro_export]
macro_rules! f {
    ($($s:tt)*) => {
        format!($($s)*)
    };
}

/// Internal macro utils to handle strings and casings.
#[macro_export]
macro_rules! ident {
    ($($s:tt)*) => {
        format_ident!($($s)*)
    };
}

/// Internal macro utils to handle strings and casings.
#[macro_export]
macro_rules! ts2 {
    ($($s:expr),*) => {
        s!($($s),*).parse::<Ts2>()
            .unwrap_or_else(|e| {
                let err = f!("failed to parse token stream: {}", e);
                pan!(err);
            })
    };
}

/// Internal macro utils to handle strings and casings.
#[macro_export]
macro_rules! pascal_str {
    ($($s:expr),*) => {
        s!($(s!($s).to_upper_camel_case()),*)
    };
}

/// Internal macro utils to handle strings and casings.
#[macro_export]
macro_rules! pascal {
    ($($s:expr),*) => {
        ts2!(pascal_str!($($s),*))
    };
}

/// Internal macro utils to handle strings and casings.
#[macro_export]
macro_rules! camel_str {
    ($s:expr $(, $ss:expr)*) => {
        s!(s!($s).to_lower_camel_case() $(, s!($ss).to_upper_camel_case())*)
    };
}

/// Internal macro utils to handle strings and casings.
#[macro_export]
macro_rules! camel {
    ($($s:expr),*) => {
        ts2!(camel_str!($($s),*))
    };
}

/// Internal macro utils to handle strings and casings.
#[macro_export]
macro_rules! snake_str {
    ($s:expr $(, $ss:expr)*) => {
        s!(s!($s).to_snake_case() $(, "_", s!($ss).to_snake_case())*)
    };
}

/// Internal macro utils to handle strings and casings.
#[macro_export]
macro_rules! snake {
    ($($s:expr),*) => {
        ts2!(snake_str!($($s),*))
    };
}

/// Internal macro utils to handle strings and casings.
#[macro_export]
macro_rules! scream_str {
    ($s:expr $(, $ss:expr)*) => {
        s!(s!($s).to_shouty_snake_case() $(, "_", s!($ss).to_shouty_snake_case())*)
    };
}

/// Internal macro utils to handle strings and casings.
#[macro_export]
macro_rules! scream {
    ($($s:expr),*) => {
        ts2!(scream_str!($($s),*))
    };
}

/// Internal macro utils to handle strings and casings.
#[macro_export]
macro_rules! bug {
    ($err:ident) => {{
        let err = f!("SHOULD NOT HAPPEN, FRAMEWORK BUG: {}", $err);
        pan!(err);
    }};
}

/// Internal macro utils to handle strings and casings.
#[macro_export]
macro_rules! pan {
    ($err:ident) => {
        panic!("panic at {}:{}\n{}", file!(), line!(), $err)
    };
}
