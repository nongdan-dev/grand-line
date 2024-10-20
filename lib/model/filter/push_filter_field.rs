use crate::prelude::*;
use heck::{ToLowerCamelCase, ToUpperCamelCase};
use proc_macro2::TokenStream as TokenStream2;
use quote::{format_ident, quote, ToTokens};
use syn::Field;

pub fn push_filter_field(
    f: &Field,
    fields: &mut Vec<TokenStream2>,
    matches: &mut Vec<TokenStream2>,
    op_str: &str,
) {
    // sea_orm generated Column::Name.op(v)
    let column = format_ident!(
        "{}",
        f.ident.as_ref().unwrap().to_string().to_upper_camel_case(),
    );
    let op = format_ident!("{}", op_str);
    // unwrap Option<type>
    // the type can be generic such as Box<type>
    let ty_str = unwrap_ty(f.ty.to_token_stream());
    let mut ty: TokenStream2 = ty_str.parse().unwrap();
    // handle special operators
    let mut as_op_str = op_str.to_string();
    if op_str == "is_in" || op_str == "is_not_in" {
        as_op_str = op_str.replace("is_", "");
        ty = quote!(Vec<#ty>);
    }
    // struct struct_field_some_op
    // graphql structField_someOp
    let mut name_str = f.ident.as_ref().unwrap().to_string();
    let mut graphql_name_str = name_str.to_lower_camel_case();
    if op_str != "eq" {
        name_str = format!("{}_{}", name_str, as_op_str);
        graphql_name_str = format!("{}_{}", graphql_name_str, as_op_str.to_lower_camel_case());
    }
    let name = format_ident!("{}", name_str);
    // push
    fields.push(quote! {
        #[graphql(name=#graphql_name_str)]
        pub #name: Option<#ty>,
    });
    matches.push(quote! {
        if let Some(v) = this.#name {
            c = c.add(Column::#column.#op(v));
        }
    });
}
