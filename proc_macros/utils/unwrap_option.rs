use crate::prelude::*;
use std::fmt::Display;

pub fn unwrap_option(ty: impl Display) -> (bool, TokenStream2) {
    let (opt, uw_str) = unwrap_option_str(ty);
    (opt, ts2!(uw_str))
}

pub fn unwrap_option_str(ty: impl Display) -> (bool, String) {
    let uw_str = str!(ty).replace(" ", "");
    if uw_str.starts_with("Option<") {
        return (true, str!(uw_str[7..(uw_str.len() - 1)]));
    }
    (false, uw_str)
}
