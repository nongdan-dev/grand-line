use crate::prelude::*;

pub struct ResolverTy {
    ty: TokenStream2,
    name: TokenStream2,
    attr: ResolverTyAttr,
    item: ResolverTyItem,
}

impl ResolverTy {
    pub fn g(
        ty: TokenStream2,
        name: TokenStream2,
        attr: ResolverTyAttr,
        item: ResolverTyItem,
    ) -> TokenStream {
        let g = Self {
            ty,
            name,
            attr,
            item,
        };

        let ty = &g.ty;
        let resolver = g.gen_resolver_fn();

        let r = quote! {
            use sea_orm::*;
            use sea_orm::prelude::*;
            use sea_orm::entity::prelude::*;

            #[derive(Default)]
            pub struct #ty;
            #[async_graphql::Object]
            impl #ty {
                #resolver
            }
        };

        #[cfg(feature = "debug_macro")]
        debug_macro(&g.item.gql_name, r.clone());

        r.into()
    }
}

impl DebugPanic for ResolverTy {
    fn debug(&self) -> String {
        self.item.gql_name.clone()
    }
}

impl GenResolverFn for ResolverTy {
    fn name(&self) -> TokenStream2 {
        self.name.clone()
    }
    fn gql_name(&self) -> String {
        self.item.gql_name.clone()
    }
    fn inputs(&self) -> TokenStream2 {
        self.item.inputs.clone()
    }
    fn output(&self) -> TokenStream2 {
        self.item.output.clone()
    }
    fn body(&self) -> TokenStream2 {
        self.item.body.clone()
    }
    fn no_tx(&self) -> bool {
        self.attr.no_tx
    }
    fn no_ctx(&self) -> bool {
        self.attr.no_ctx
    }
    fn no_async(&self) -> bool {
        self.attr.no_async
    }
}
