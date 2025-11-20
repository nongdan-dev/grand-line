use crate::prelude::*;

pub struct ResolverTy {
    ty: Ts2,
    name: Ts2,
    ra: ResolverTyAttr,
    item: ResolverTyItem,
}

impl ResolverTy {
    pub fn g(ty: Ts2, name: Ts2, ra: ResolverTyAttr, item: ResolverTyItem) -> TokenStream {
        let g = Self { ty, name, ra, item };

        let ty = &g.ty;
        let resolver = g.resolver_fn();
        let m = g.ty.to_string().to_snake_case().ts2_or_panic();

        let r = quote! {
            mod #m {
                pub use super::*;

                #[derive(Default)]
                pub struct #ty;
                #[Object]
                impl #ty {
                    #resolver
                }
            }
            pub use #m::#ty;
        };

        #[cfg(feature = "debug_macro")]
        debug_macro(&g.item.gql_name, r.clone());

        r.into()
    }
}

impl AttrDebug for ResolverTy {
    fn attr_debug(&self) -> String {
        self.ra.inner.attr_debug()
    }
}

impl ResolverFn for ResolverTy {
    fn name(&self) -> Ts2 {
        self.name.clone()
    }
    fn gql_name(&self) -> String {
        self.item.gql_name.clone()
    }
    fn inputs(&self) -> Ts2 {
        self.item.inputs.clone()
    }
    fn output(&self) -> Ts2 {
        self.item.output.clone()
    }
    fn body(&self) -> Ts2 {
        self.item.body.clone()
    }

    fn root_operation_ty(&self) -> Option<String> {
        let ty = self.ty.to_string();
        let operations = ["Query", "Mutation", "Subscription"];
        let operation = operations
            .iter()
            .find(|o| ty.ends_with(*o))
            .copied()
            .map(|o| o.to_owned());
        if operation.is_none() {
            let valid = operations.join(", ");
            panic!("root resolver {ty} should be one of: {valid}");
        }
        operation
    }

    fn no_tx(&self) -> bool {
        self.ra.no_tx
    }
    fn no_ctx(&self) -> bool {
        self.ra.no_ctx
    }
    fn auth(&self) -> Option<AuthAttr> {
        self.ra.auth.clone()
    }
    fn authz(&self) -> Option<AuthzAttr> {
        self.ra.authz.clone()
    }
}
