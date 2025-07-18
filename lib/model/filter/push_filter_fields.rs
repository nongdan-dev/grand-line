use crate::prelude::*;
use heck::{ToLowerCamelCase, ToUpperCamelCase};
use proc_macro2::TokenStream as TokenStream2;
use quote::{ToTokens, format_ident, quote};
use syn::Field;

pub fn push_filter_fields(
    f: &Field,
    fields: &mut Vec<TokenStream2>,
    matches: &mut Vec<TokenStream2>,
) {
    push(f, fields, matches, "eq");
    push(f, fields, matches, "ne");
    let (is_option, ty_str) = unwrap_option(f.ty.to_token_stream());
    if is_option {
        push(f, fields, matches, "is_null");
        push(f, fields, matches, "is_not_null");
    }
    if ty_str == "bool" {
        return;
    }
    push(f, fields, matches, "is_in");
    push(f, fields, matches, "is_not_in");
    let name = f.ident.as_ref().unwrap().to_string();
    if name == "id" || name.ends_with("_id") {
        return;
    }
    push(f, fields, matches, "gt");
    push(f, fields, matches, "gte");
    push(f, fields, matches, "lt");
    push(f, fields, matches, "lte");
    if ty_str != "String" {
        return;
    }
    push(f, fields, matches, "like");
    push(f, fields, matches, "not_like");
    push(f, fields, matches, "starts_with");
    push(f, fields, matches, "ends_with");
}

fn push(f: &Field, fields: &mut Vec<TokenStream2>, matches: &mut Vec<TokenStream2>, op_str: &str) {
    // sea_orm generated Column::Name.op(v)
    let column = format_ident!(
        "{}",
        f.ident.as_ref().unwrap().to_string().to_upper_camel_case(),
    );
    let op = format_ident!("{}", op_str);
    // unwrap Option<type>
    // the type can be generic such as Box<type>
    let (_, ty_str) = unwrap_option(f.ty.to_token_stream());
    let mut ty: TokenStream2 = ty_str.parse().unwrap();
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
    let mut name_str = f.ident.as_ref().unwrap().to_string();
    let mut gql_name_str = name_str.to_lower_camel_case();
    if op_str != "eq" {
        name_str = format!("{}_{}", name_str, as_op_str);
        gql_name_str = format!("{}_{}", gql_name_str, as_op_str.to_lower_camel_case());
    }
    let name = format_ident!("{}", name_str);
    // push
    fields.push(quote! {
        #[graphql(name=#gql_name_str)]
        pub #name: Option<#ty>,
    });
    if op_str == "is_null" || op_str == "is_not_null" {
        matches.push(quote! {
            if let Some(v) = this.#name {
                if v {
                    c = c.add(Column::#column.#op());
                }
            }
        });
    } else {
        matches.push(quote! {
            if let Some(v) = this.#name {
                c = c.add(Column::#column.#op(v));
            }
        });
    }
}
