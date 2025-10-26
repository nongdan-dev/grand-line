use crate::prelude::*;
use syn::{
    Expr, Result, Token,
    parse::{Parse, ParseStream},
    parse_macro_input,
};

struct Args {
    db: Expr,
    _comma: Token![,],
    struk: Expr,
    _trailing: Option<Token![,]>,
}

impl Parse for Args {
    fn parse(s: ParseStream) -> Result<Self> {
        Ok(Self {
            db: s.parse()?,
            _comma: s.parse()?,
            struk: s.parse()?,
            _trailing: s.parse().ok(),
        })
    }
}

pub fn gen_db_action(am: &str, db_fn: &str, item: TokenStream) -> TokenStream {
    let Args { db, struk, .. } = parse_macro_input!(item as Args);

    let am = ts2!("am_", am);
    let db_fn = ts2!(db_fn);

    quote! {
        #am!(#struk)
            .#db_fn(#db)
            .await?
    }
    .into()
}
