use crate::prelude::*;
use serde::{Deserialize, Serialize};
use serde_qs::{from_str, to_string};

#[derive(Serialize, Deserialize)]
pub struct QsToken {
    pub id: String,
    pub secret: String,
}

pub fn qs_token(id: &str, secret: &str) -> Res<String> {
    let t = to_string(&QsToken {
        id: id.to_string(),
        secret: secret.to_string(),
    })
    .map_err(MyErr::from)?;
    Ok(t)
}

pub fn qs_token_parse(token: &str) -> Option<QsToken> {
    if token.is_empty() {
        None
    } else {
        from_str::<QsToken>(token).ok()
    }
}
