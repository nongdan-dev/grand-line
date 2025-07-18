use std::fmt::Display;

pub fn unwrap_option(ty: impl Display) -> (bool, String) {
    let ty_str = ty.to_string().replace(" ", "");
    if ty_str.starts_with("Option<") {
        return (true, ty_str[7..(ty_str.len() - 1)].to_string());
    }
    (false, ty_str)
}
