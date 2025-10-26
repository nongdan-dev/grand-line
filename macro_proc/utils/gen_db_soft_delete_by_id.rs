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
    _comma2: Token![,],
    id: Expr,
    _trailing: Option<Token![,]>,
}

impl Parse for Args {
    fn parse(s: ParseStream) -> Result<Self> {
        Ok(Self {
            db: s.parse()?,
            _comma1: s.parse()?,
            ty: s.parse()?,
            _comma2: s.parse()?,
            id: s.parse()?,
            _trailing: s.parse().ok(),
        })
    }
}

pub fn gen_db_soft_delete_by_id(item: TokenStream) -> TokenStream {
    let Args { db, ty, id, .. } = parse_macro_input!(item as Args);

    quote! {
        #ty::soft_delete_by_id(#id)?
            .exec(#db)
            .await?
    }
    .into()
}
