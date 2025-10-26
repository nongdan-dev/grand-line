use crate::prelude::*;
use syn::{
    Expr, Path, Result, Token,
    parse::{Parse, ParseStream},
    parse_macro_input,
};

struct Args {
    db: Expr,
    _comma1: Token![,],
    ty: Path,
    _trailing: Option<Token![,]>,
}

impl Parse for Args {
    fn parse(s: ParseStream) -> Result<Self> {
        Ok(Self {
            db: s.parse()?,
            _comma1: s.parse()?,
            ty: s.parse()?,
            _trailing: s.parse().ok(),
        })
    }
}

pub fn gen_db_soft_delete_many(item: TokenStream) -> TokenStream {
    let Args { db, ty, .. } = parse_macro_input!(item as Args);

    quote! {
        #ty::soft_delete_many()?
            .exec(#db)
            .await?
    }
    .into()
}
