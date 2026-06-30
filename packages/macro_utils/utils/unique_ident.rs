use crate::prelude::*;
use ulid::Ulid;

pub fn unique_ident() -> Ts2 {
    let id = Ulid::new().to_string().to_lowercase();
    let tmp = format!("__grandline_{id}");
    Ident::new(&tmp, Span::mixed_site()).to_token_stream()
}
