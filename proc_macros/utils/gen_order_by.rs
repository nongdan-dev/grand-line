use crate::prelude::*;
use syn::{
    Ident, Token, bracketed,
    parse::{Parse, ParseStream},
    parse_macro_input,
    punctuated::Punctuated,
};

struct Item {
    model: Ident,
    fields: Punctuated<Ident, Token![,]>,
}

impl Parse for Item {
    fn parse(s: ParseStream) -> syn::Result<Self> {
        let model = s.parse::<Ident>()?;
        let c;
        bracketed!(c in s);
        let fields = c.parse_terminated(Ident::parse, Token![,])?;
        Ok(Item { model, fields })
    }
}

/// Need to use proc macro to concat model name with OrderBy.
/// Can be simpler general macro with paste, but we must export it.
pub fn gen_order_by(item: TokenStream) -> TokenStream {
    let item = parse_macro_input!(item as Item);
    let order_by = ty_order_by(item.model);
    let paths = item.fields.iter().map(|f| quote!(#order_by::#f));

    quote!(vec![#(#paths,)*]).into()
}
