use crate::prelude::*;
use heck::ToSnakeCase;
use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::{format_ident, quote};
use syn::{parse_macro_input, Fields, ItemStruct};

pub fn gen_model(_attr: TokenStream, _item: TokenStream) -> TokenStream {
    let mut a = parse_attr!(_attr);
    let mut struk = parse_macro_input!(_item as ItemStruct);
    a.model = struk.ident.to_string();
    // ------------------------------------------------------------------------
    // get the original model name, and set the new name that sea_orm requires
    // get the original model name in snake case for sql table, non-plural
    let alias = struk.ident;
    let alias_column = format_ident!("{}Column", alias);
    let alias_entity = format_ident!("{}Entity", alias);
    let alias_active_model = format_ident!("{}ActiveModel", alias);
    struk.ident = format_ident!("Model");
    let alias_str = alias.to_string();
    let alias_snake_str = alias_str.to_snake_case();
    // ------------------------------------------------------------------------
    // insert built-in fields: id, created_at, updated_at
    struk = insert_builtin(&a, struk);
    // ------------------------------------------------------------------------
    // filter / order_by fields
    let filter = format_ident!("{}Filter", alias);
    let order_by = format_ident!("{}OrderBy", alias);
    let mut filter_fields = vec![];
    let mut filter_matches = vec![];
    let mut order_by_fields = vec![];
    let mut order_by_matches = vec![];
    for ref f in parse_unwrap_ref!(struk.fields => Fields::Named)
        .named
        .iter()
    {
        push_filter_fields(f, &mut filter_fields, &mut filter_matches);
        push_order_by_fields(f, &mut order_by_fields, &mut order_by_matches);
    }
    push_filter_and_or(&filter, &mut filter_fields, &mut filter_matches, "and");
    push_filter_and_or(&filter, &mut filter_fields, &mut filter_matches, "or");
    push_filter_not(&filter, &mut filter_fields, &mut filter_matches);
    // ------------------------------------------------------------------------
    // db fn builtin for crud to be used in graphql handler
    let search_fn = db_search(&a.model);
    let count_fn = db_count(&a.model);
    let detail_fn = db_detail(&a.model);
    let delete_fn = db_delete(&a.model);
    // ------------------------------------------------------------------------
    // return
    let active = q_model_active();
    let relationship = q_model_relationship();
    let model_filter = q_model_filter(&filter, &filter_fields, &filter_matches);
    let model_order_by = q_model_order_by(&order_by, &order_by_fields, &order_by_matches);
    let model_search = q_model_search(&alias, &filter, &order_by, &search_fn);
    let model_detail = q_model_detail(&alias, &detail_fn);
    let model_delete = q_model_delete(&alias, &delete_fn);

    let mut macro_streams: Vec<TokenStream2> = vec![];
    if !a.no_macro {
        for op_str in util_macros() {
            let op = format_ident!("{}", op_str);
            let alias_op = util_macro(&a.model, &op);
            let stream = quote! {
                #[macro_export]
                macro_rules! #alias_op {
                    ($tt:tt) => {
                        #op!(#alias $tt)
                    };
                }
            };
            macro_streams.push(stream);
        }
    }

    quote! {
        use grand_line::*;
        use sea_orm::*;
        use sea_orm::entity::prelude::*;

        #[derive(
            Clone,
            Debug,
            serde::Deserialize,
            serde::Serialize,
            async_graphql::SimpleObject,
            build::GrandLineModel,
            DeriveEntityModel,
        )]
        #[sea_orm(table_name=#alias_snake_str)]
        #[graphql(name=#alias_str)]
        #struk

        pub type #alias = Model;
        pub type #alias_column = Column;
        pub type #alias_entity = Entity;
        pub type #alias_active_model = ActiveModel;

        #active
        #relationship

        #(#macro_streams)*


        #model_filter
        #model_order_by

        #model_search
        #model_detail
        #model_delete

        pub async fn #count_fn(
            tx: &DatabaseTransaction,
            filter: Option<#filter>,
        ) -> Result<u64, Box<dyn std::error::Error + Send + Sync>> {
            let mut q = Entity::find();
            if let Some(f) = filter {
                q = f.chain(q);
            }
            Ok(q.count(tx).await?)
        }
    }
    .into()
}
