use crate::prelude::*;
use syn::Field;

pub fn push_filter(f: &Field, struk: &mut Vec<TokenStream2>, query: &mut Vec<TokenStream2>) {
    push(f, struk, query, "eq");
    push(f, struk, query, "ne");
    let (opt, uw_str) = unwrap_option_str(f.ty.to_token_stream());
    if opt {
        push(f, struk, query, "is_null");
        push(f, struk, query, "is_not_null");
    }
    if uw_str == "bool" {
        return;
    }
    push(f, struk, query, "is_in");
    push(f, struk, query, "is_not_in");
    let name_str = str!(f.ident.to_token_stream());
    if name_str == "id" || name_str.ends_with("_id") {
        return;
    }
    push(f, struk, query, "gt");
    push(f, struk, query, "gte");
    push(f, struk, query, "lt");
    push(f, struk, query, "lte");
    if uw_str != "String" {
        return;
    }
    push(f, struk, query, "like");
    push(f, struk, query, "not_like");
    push(f, struk, query, "starts_with");
    push(f, struk, query, "ends_with");
}

fn push(f: &Field, struk: &mut Vec<TokenStream2>, query: &mut Vec<TokenStream2>, op_str: &str) {
    // sea_orm generated Column::Name.op(v)
    let column = pascal!(f.ident.to_token_stream());
    let op = ts2!(op_str);
    // unwrap Option<type>
    // the type can be generic such as Box<type>
    let (_, mut uw) = unwrap_option(f.ty.to_token_stream());
    // handle special operators
    if op_str == "is_null" || op_str == "is_not_null" {
        uw = quote!(bool);
    }
    let mut as_op_str = str!(op_str);
    if op_str == "is_in" || op_str == "is_not_in" {
        as_op_str = op_str.replace("is_", "");
        uw = quote!(Vec<#uw>);
    }
    // struct struct_field_some_op
    // graphql structField_someOp
    let mut name = f.ident.to_token_stream();
    let mut gql_name = camel_str!(name);
    if op_str != "eq" {
        name = snake!(name, as_op_str);
        gql_name = str!(gql_name, "_", camel_str!(as_op_str));
    }
    // push
    struk.push(quote! {
        #[graphql(name=#gql_name)]
        pub #name: Option<#uw>,
    });
    if op_str == "is_null" || op_str == "is_not_null" {
        query.push(quote! {
            if let Some(v) = this.#name {
                if v {
                    c = c.add(Column::#column.#op());
                }
            }
        });
    } else {
        query.push(quote! {
            if let Some(v) = this.#name {
                c = c.add(Column::#column.#op(v));
            }
        });
    }
}
