use crate::prelude::*;
use heck::ToSnakeCase;
use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::{format_ident, quote, ToTokens};
use syn::{parse_macro_input, Fields, ItemStruct};

pub fn gen_macro(_attr: TokenStream, _item: TokenStream) -> TokenStream {
    let mut attr: TokenStream2 = _attr.clone().into();

    let mut a = parse_attr!(_attr);
    let mut struk = parse_macro_input!(_item as ItemStruct);

    let upper_ = struk.clone().ident;
    let upper = upper_.to_string();
    a.model = upper.clone();

    if !a.no_macro {
        attr = append_attr!(attr, "no_macro=true");
    }
    if !a.no_builtin {
        attr = append_attr!(attr, "no_builtin=true");
        struk = insert_builtin(&a, struk);
    }
    let mut meta = MacroMeta::default();
    for ref f in parse_unwrap_ref!(struk.fields => Fields::Named)
        .named
        .iter()
    {
        meta.fields.push(f.ident.to_token_stream().to_string());
        meta.types.push(f.ty.to_token_stream().to_string());
    }
    let meta = serde_json::to_string(&meta).unwrap();

    let snake = upper.to_snake_case();
    let model = format_ident!("{}_model", snake);

    let a2: TokenStream2 = "#attr".to_string().parse().unwrap();
    let i2: TokenStream2 = "#item".to_string().parse().unwrap();

    let mut op_streams: Vec<TokenStream2> = vec![];
    for op_str in util_macros() {
        let op = format_ident!("{}", op_str);
        let alias = util_macro(&a.model, &op);
        let stream = quote! {
            #[proc_macro]
            pub fn #alias(_item: proc_macro::TokenStream) -> proc_macro::TokenStream {
                let item: TokenStream2 = _item.into();
                quote! {
                    #op!(#upper_ #i2)
                }
                .into()
            }
        };
        op_streams.push(stream);
    }
    let mut handler_streams: Vec<TokenStream2> = vec![];
    for func_str in crud_util_macros() {
        let func = format_ident!("{}", func_str);
        let alias = format_ident!("{}_{}", snake, func_str);
        let stream = quote! {
            #[proc_macro_attribute]
            pub fn #alias(_attr: proc_macro::TokenStream, _item: proc_macro::TokenStream) -> proc_macro::TokenStream {
                let mut attr: TokenStream2 = append_attr!(#upper, _attr);
                if #func_str == "create" || #func_str == "update" {
                    let meta = format!("meta={}", stringify!(#meta));
                    attr = append_attr!(attr, meta);
                }
                let item: TokenStream2 = _item.into();
                quote! {
                    use grand_line::build::*;
                    #[#func(#a2)]
                    #i2
                }.into()
            }
        };
        handler_streams.push(stream);
    }

    quote! {
        use grand_line::build::*;

        #[proc_macro]
        pub fn #model(_item: proc_macro::TokenStream) -> proc_macro::TokenStream {
            quote! {
                use grand_line::build::*;
                #[model(#attr)]
                #struk
            }
            .into()
        }

        #(#op_streams)*
        #(#handler_streams)*
    }
    .into()
}
