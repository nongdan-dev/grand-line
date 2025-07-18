#[macro_export]
macro_rules! parse_optional {
    ($v:expr => $t:path) => {
        if let $t(e) = $v { Some(e) } else { None }
    };
}

#[macro_export]
macro_rules! parse_optional_ref {
    ($v:expr => $t:path) => {
        if let $t(ref mut e) = $v {
            Some(e)
        } else {
            None
        }
    };
}
