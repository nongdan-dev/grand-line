use crate::prelude::*;
use std::fmt::Display;

pub fn ty_sql(model: impl Display) -> TokenStream2 {
    pascal!(model, "Sql")
}
pub fn ty_gql(model: impl Display) -> TokenStream2 {
    pascal!(model, "Gql")
}
pub fn ty_column(model: impl Display) -> TokenStream2 {
    pascal!(model, "Column")
}
pub fn ty_active_model(model: impl Display) -> TokenStream2 {
    pascal!(model, "ActiveModel")
}
pub fn ty_filter(model: impl Display) -> TokenStream2 {
    pascal!(model, "Filter")
}
pub fn ty_order_by(model: impl Display) -> TokenStream2 {
    pascal!(model, "OrderBy")
}

pub fn ty_query(pre: impl Display) -> TokenStream2 {
    pascal!(pre, "Query")
}
pub fn ty_mutation(pre: impl Display) -> TokenStream2 {
    pascal!(pre, "Mutation")
}
