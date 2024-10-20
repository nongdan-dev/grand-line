use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::Ident;

pub fn q_model_search(
    alias: &Ident,
    filter: &Ident,
    order_by: &Ident,
    search: &Ident,
) -> TokenStream2 {
    quote! {
        pub async fn #search(
            tx: &DatabaseTransaction,
            filter: Option<#filter>,
            order_by: Option<Vec<#order_by>>,
            page: Option<Pagination>,
        ) -> Result<Vec<#alias>, Box<dyn std::error::Error + Send + Sync>> {
            let mut q = Entity::find();
            if let Some(f) = filter {
                q = f.chain(q);
            }
            if let Some(os) = order_by {
                if os.len() > 0 {
                    for o in os {
                        q = o.chain(q);
                    }
                } else {
                    q = q.order_by_desc(Column::Id);
                }
            } else {
                q = q.order_by_desc(Column::Id);
            }
            let mut offset = 0;
            let mut limit = 10;
            if let Some(p) = page {
                if let Some(o) = p.offset {
                    offset = o;
                }
                if let Some(l) = p.limit {
                    limit = if l > 1000 { 1000 } else { l };
                }
            }
            Ok(q.offset(offset).limit(limit).all(tx).await?)
        }
    }
}
