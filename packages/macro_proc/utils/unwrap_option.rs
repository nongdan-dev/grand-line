use crate::prelude::*;

pub fn unwrap_option(ty: impl Display) -> (bool, Ts2) {
    let (opt, uw_str) = unwrap_option_str(ty);
    (opt, uw_str.ts2_or_panic())
}

pub fn unwrap_option_str(ty: impl Display) -> (bool, String) {
    let uw_str = ty.to_string().replace(" ", "");
    if uw_str.starts_with("Option<") {
        return (true, uw_str[7..(uw_str.len() - 1)].to_owned());
    }
    (false, uw_str)
}
