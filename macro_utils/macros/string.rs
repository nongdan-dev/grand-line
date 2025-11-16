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
macro_rules! pascal_str {
    ($($s:expr),*) => {
        s!($(s!($s).to_upper_camel_case()),*)
    };
}

/// Internal macro utils to handle strings and casings.
#[macro_export]
macro_rules! pascal {
    ($($s:expr),*) => {
        pascal_str!($($s),*).ts2()
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
        camel_str!($($s),*).ts2()
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
        snake_str!($($s),*).ts2()
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
        scream_str!($($s),*).ts2()
    };
}

/// Internal macro utils to handle strings and casings.
#[macro_export]
macro_rules! bug {
    ($($s:tt)*) => {{
        let err = f!($($s)*);
        pan!("SHOULD NOT HAPPEN, FRAMEWORK BUG: {err}");
    }};
}

/// Internal macro utils to handle strings and casings.
#[macro_export]
macro_rules! pan {
    ($($s:tt)*) => {{
        let err = f!($($s)*);
        let file = file!();
        let line = line!();
        panic!("panic at {file}:{line}\n{err}");
    }};
}
