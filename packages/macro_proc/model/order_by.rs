use crate::prelude::*;

pub fn order_by(f: &Field, struk: &mut Vec<Ts2>, query: &mut Vec<Ts2>) {
    push(f, struk, query, "asc");
    push(f, struk, query, "desc");
}
fn push(f: &Field, struk: &mut Vec<Ts2>, query: &mut Vec<Ts2>, direction_str: &str) {
    // sea_orm generated order_by_#direction(Column::Name)
    let column = f
        .ident
        .to_token_stream()
        .to_string()
        .to_pascal_case()
        .ts2_or_panic();
    let direction_fn = format!("order_by_{direction_str}")
        .to_snake_case()
        .ts2_or_panic();
    // enum EnumField
    // graphql EnumField
    let gql_name = format!("{column}_{direction_str}").to_pascal_case();
    let name = gql_name.ts2_or_panic();
    // push
    struk.push(quote! {
        #[graphql(name = #gql_name)]
        #name,
    });
    query.push(quote! {
        Self::#name => q.#direction_fn(Column::#column),
    });
}
