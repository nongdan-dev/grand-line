#[macro_export]
// TODO: remove this and use match just like in gen_model
macro_rules! parse_unwrap_ref {
    ($v:expr => $t:path) => {
        if let $t(ref mut e) = $v {
            e
        } else {
            panic!("Unwrap must be {}", stringify!($t))
        }
    };
}
