#[macro_export]
macro_rules! parse_unwrap {
    ($v:expr => $t:path) => {
        if let $t(e) = $v {
            e
        } else {
            panic!("not a {}", stringify!($t))
        }
    };
}

#[macro_export]
macro_rules! parse_unwrap_ref {
    ($v:expr => $t:path) => {
        if let $t(ref mut e) = $v {
            e
        } else {
            panic!("not a {}", stringify!($t))
        }
    };
}
