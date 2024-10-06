use crate::prelude::*;
use heck::ToSnakeCase;
use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::{format_ident, quote};

pub fn gen_update(_attr: TokenStream, _item: TokenStream) -> TokenStream {
    let a = parse_attr!(_attr);
    let g = parse_resolver!(m, _item, name_update(&a.model));
    let (a, mut g) = check_crud_io(a, g);

    let model = ty_model(&a.model);
    let ty = ty_input(&g.name);

    if !a.resolver_inputs {
        if a.meta == "" {
            panic!("{} meta is required to build the input data type", &g.name);
        }

        let builtin = vec!["id", "created_at", "updated_at"];
        let mut fields: TokenStream2 = "".parse().unwrap();
        let meta: MacroMeta = serde_json::from_str(&a.meta).unwrap();
        for (i, f) in meta.fields.into_iter().enumerate() {
            if builtin.contains(&&f[..]) {
                continue;
            }
            let f2 = format_ident!("{}", f.to_snake_case());
            let t = unwrap_ty(meta.types.get(i).unwrap());
            let t: TokenStream2 = format!("Option<{}>", t).parse().unwrap();
            fields = quote! {
                #fields
                #[graphql(name=#f)]
                #f2: #t,
            }
        }

        g.inputs_ty = quote! {
            use grand_line::*;
            #[input]
            pub struct #ty {
                #fields
            }
        };

        g.inputs = quote! {
            id: String,
            data: #ty,
        };
    }

    if !a.resolver_output {
        g.output = quote! {
            #model
        };
    }

    g.no_tx = a.no_tx;
    gen_resolver(g)
}
