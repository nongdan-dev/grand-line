use crate::prelude::*;

pub fn gen_update(attr: TokenStream, item: TokenStream) -> TokenStream {
    let a = parse_macro_input!(attr as AttrParse);
    let r = parse_macro_input!(item as ResolverTyItem);
    try_gen_update(a, r).unwrap_or_else(|e| e.to_compile_error().into())
}

fn try_gen_update(attr: AttrParse, r: ResolverTyItem) -> SynRes<TokenStream> {
    let a = attr.into_inner::<CrudAttr>("update")?;
    let (mut r, ty, name) = r.init("mutation", "update", &a.model)?;
    a.validate(&r)?;

    if !a.resolver_inputs {
        let data = name.to_string().to_pascal_case().ts2_or_err()?;
        r.inputs = quote! {
            id: String,
            data: #data,
        };
    }

    if !a.resolver_output {
        let output = ty_gql(&a.model)?;
        r.output = quote!(#output);

        let body = r.body;
        let model = a.model.ts2_or_err()?;
        let am = ty_active_model(&a.model)?;

        let exec = if a.ra.has_auth() {
            quote!(exec(ctx))
        } else {
            quote!(exec_without_ctx(tx))
        };

        r.body = quote! {
            let am: ActiveModelWrapper<AmUpdate, #model, #am> = {
                #body
            };
            am.#exec.await?.into_gql(ctx).await?
        }
    }

    ResolverTy::g(ty, name, a.ra, r)
}
