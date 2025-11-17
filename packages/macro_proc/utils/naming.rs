use crate::prelude::*;

pub fn ty_sql(model: impl Display) -> Ts2 {
    format!("{model}_Sql").to_pascal_case().ts2_or_panic()
}
pub fn ty_gql(model: impl Display) -> Ts2 {
    format!("{model}_Gql").to_pascal_case().ts2_or_panic()
}
pub fn ty_column(model: impl Display) -> Ts2 {
    format!("{model}_Column").to_pascal_case().ts2_or_panic()
}
pub fn ty_active_model(model: impl Display) -> Ts2 {
    format!("{model}_ActiveModel")
        .to_pascal_case()
        .ts2_or_panic()
}
pub fn ty_filter(model: impl Display) -> Ts2 {
    format!("{model}_Filter").to_pascal_case().ts2_or_panic()
}
pub fn ty_order_by(model: impl Display) -> Ts2 {
    format!("{model}_OrderBy").to_pascal_case().ts2_or_panic()
}
