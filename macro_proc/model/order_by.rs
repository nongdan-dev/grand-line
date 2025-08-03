use crate::prelude::*;
use syn::Field;

pub fn order_by(f: &Field, struk: &mut Vec<Ts2>, query: &mut Vec<Ts2>) {
    push(f, struk, query, "asc");
    push(f, struk, query, "desc");
}
fn push(f: &Field, struk: &mut Vec<Ts2>, query: &mut Vec<Ts2>, direction_str: &str) {
    // sea_orm generated order_by_#direction(Column::Name)
    let column = pascal!(f.ident.to_token_stream());
    let direction_fn = snake!("order_by", direction_str);
    // enum EnumField
    // graphql EnumField
    let name = pascal!(column, direction_str);
    let gql_name = s!(name);
    // push
    struk.push(quote! {
        #[graphql(name=#gql_name)]
        #name,
    });
    query.push(quote! {
        Self::#name => q.#direction_fn(Column::#column),
    });
}
