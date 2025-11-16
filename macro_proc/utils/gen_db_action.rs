use crate::prelude::*;

struct Args {
    db: Expr,
    _comma: Token![,],
    struk: Expr,
    _trailing: Option<Token![,]>,
}

impl Parse for Args {
    fn parse(s: ParseStream) -> SynRes<Self> {
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

    let am = f!("am_{am}").ts2();
    let db_fn = db_fn.ts2();

    quote! {
        #am!(#struk)
            .#db_fn(#db)
            .await?
    }
    .into()
}
