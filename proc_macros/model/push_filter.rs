use crate::prelude::*;
use syn::Field;

pub fn push_filter(f: &Field, struk: &mut Vec<TokenStream2>, query: &mut Vec<TokenStream2>) {
    push(f, struk, query, "eq");
    push(f, struk, query, "ne");
    let (_, uw_str) = unwrap_option_str(f.ty.to_token_stream());
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
    let mut gql_op = str!(op_str);
    // unwrap Option<type>
    // the type can be generic such as Box<type>
    let (opt, mut ty) = unwrap_option(f.ty.to_token_stream());
    // handle special operators
    if op_str == "is_in" || op_str == "is_not_in" {
        gql_op = op_str.replace("is_", "");
        ty = quote!(Vec<#ty>);
    }
    // struct struct_field_some_op
    // graphql structField_someOp
    let mut name = f.ident.to_token_stream();
    let mut gql_name = camel_str!(name);
    if op_str != "eq" {
        name = snake!(name, gql_op);
        gql_name = str!(gql_name, "_", camel_str!(gql_op));
    }
    // push struk
    ty = if opt && (op_str == "eq" || op_str == "ne") {
        quote!(Undefined<#ty>)
    } else {
        quote!(Option<#ty>)
    };
    struk.push(quote! {
        #[graphql(name=#gql_name)]
        pub #name: #ty,
    });
    // push query
    let q = if opt && (op_str == "eq" || op_str == "ne") {
        let op_null = ts2!(if op_str == "eq" {
            "is_null"
        } else {
            "is_not_null"
        });
        quote! {
            if matches!(this.#name, Undefined::Null) {
                c = c.add(Column::#column.#op_null());
            }
            if let Undefined::Value(v) = this.#name {
                c = c.add(Column::#column.#op(v));
            }
        }
    } else {
        quote! {
            if let Some(v) = this.#name {
                c = c.add(Column::#column.#op(v));
            }
        }
    };
    query.push(q);
}
