use crate::prelude::*;
use heck::{ToLowerCamelCase, ToUpperCamelCase};
use proc_macro2::TokenStream as TokenStream2;
use quote::{ToTokens, quote};
use syn::Field;

pub fn push_filter(f: &Field, struk: &mut Vec<TokenStream2>, query: &mut Vec<TokenStream2>) {
    push(f, struk, query, "eq");
    push(f, struk, query, "ne");
    let (is_option, ty_str) = unwrap_option(f.ty.to_token_stream());
    if is_option {
        push(f, struk, query, "is_null");
        push(f, struk, query, "is_not_null");
    }
    if ty_str == "bool" {
        return;
    }
    push(f, struk, query, "is_in");
    push(f, struk, query, "is_not_in");
    let name = f.ident.to_token_stream().to_string();
    if name == "id" || name.ends_with("_id") {
        return;
    }
    push(f, struk, query, "gt");
    push(f, struk, query, "gte");
    push(f, struk, query, "lt");
    push(f, struk, query, "lte");
    if ty_str != "String" {
        return;
    }
    push(f, struk, query, "like");
    push(f, struk, query, "not_like");
    push(f, struk, query, "starts_with");
    push(f, struk, query, "ends_with");
}

fn push(f: &Field, struk: &mut Vec<TokenStream2>, query: &mut Vec<TokenStream2>, op_str: &str) {
    // sea_orm generated Column::Name.op(v)
    let column = f
        .ident
        .to_token_stream()
        .to_string()
        .to_upper_camel_case()
        .parse::<TokenStream2>()
        .unwrap();
    let op = op_str.parse::<TokenStream2>().unwrap();
    // unwrap Option<type>
    // the type can be generic such as Box<type>
    let (_, ty_str) = unwrap_option(f.ty.to_token_stream());
    let mut ty = ty_str.parse::<TokenStream2>().unwrap();
    // handle special operators
    if op_str == "is_null" || op_str == "is_not_null" {
        ty = quote!(bool);
    }
    let mut as_op_str = op_str.to_string();
    if op_str == "is_in" || op_str == "is_not_in" {
        as_op_str = op_str.replace("is_", "");
        ty = quote!(Vec<#ty>);
    }
    // struct struct_field_some_op
    // graphql structField_someOp
    let mut name = f.ident.to_token_stream();
    let mut gql_name = name.to_string().to_lower_camel_case();
    if op_str != "eq" {
        name = format!("{}_{}", name, as_op_str)
            .parse::<TokenStream2>()
            .unwrap();
        gql_name = format!("{}_{}", gql_name, as_op_str.to_lower_camel_case());
    }
    // push
    struk.push(quote! {
        #[graphql(name=#gql_name)]
        pub #name: Option<#ty>,
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
