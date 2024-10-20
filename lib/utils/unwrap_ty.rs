use std::fmt::Display;

pub fn unwrap_ty(ty: impl Display) -> String {
    let ty_str = ty.to_string().replace(" ", "");
    if ty_str.starts_with("Box<") {
        return ty_str[4..(ty_str.len() - 1)].to_string();
    }
    if ty_str.starts_with("Option<") {
        return ty_str[7..(ty_str.len() - 1)].to_string();
    }
    ty_str
}
