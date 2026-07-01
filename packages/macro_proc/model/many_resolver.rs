use crate::prelude::*;

pub fn gen_many_resolver(attr: TokenStream, item: TokenStream) -> TokenStream {
    let a = parse_macro_input!(attr as AttrParse);
    let ifn = parse_macro_input!(item as ItemFn);
    try_gen_many_resolver(a, &ifn).unwrap_or_else(|e| e.to_compile_error().into())
}

#[field_names]
pub struct ManyResolverAttr {
    #[field_names(skip)]
    pub model: String,
    pub parent: String,
}
impl TryFrom<Attr> for ManyResolverAttr {
    type Error = SynErr;
    fn try_from(a: Attr) -> SynRes<Self> {
        Ok(Self {
            model: a.model_from_first_path()?,
            parent: a.str_required(Self::FIELD_PARENT)?,
        })
    }
}
impl AttrValidate for ManyResolverAttr {
    fn attr_fields(a: &Attr) -> Vec<String> {
        Self::FIELDS
            .iter()
            .copied()
            .map(|f| f.to_owned())
            .chain(a.first_path.iter().cloned())
            .collect()
    }
}

fn try_gen_many_resolver(a: AttrParse, ifn: &ItemFn) -> SynRes<TokenStream> {
    let a = Attr::from_proc_macro("many_resolver", a)?.try_into_with_validate::<ManyResolverAttr>()?;

    let f = &ifn.sig.ident;
    let vis = &ifn.vis;
    let body = &ifn.block;

    let parent = ty_gql(&a.parent)?;
    let filter = ty_filter(&a.model)?;
    let order_by = ty_order_by(&a.model)?;

    let r = quote! {
        #vis async fn #f<D>(
            parent: &#parent,
            ctx: &Context<'_>,
            tx: &D,
            filter: &Option<#filter>,
            order_by: &Option<Vec<#order_by>>,
            page: &Option<Pagination>,
            include_deleted: &Option<bool>,
        ) -> Res<(Option<#filter>, Option<Vec<#order_by>>)>
        where
            D: ConnectionTrait,
        {
            Ok(#body)
        }
    };
    Ok(r.into())
}
