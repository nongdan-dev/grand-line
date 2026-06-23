use grand_line::prelude::*;

pub fn row_policy(k: String, script: String) -> RowPolicy {
    hashmap! {
        k => script,
    }
}
