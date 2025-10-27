use crate::prelude::*;

pub struct GenResolver {
    pub a: ResolverAttr,
}

impl VirtualResolverFn for GenResolver {
    fn sql_dep(&self) -> Vec<String> {
        self.a.sql_dep.clone()
    }
}
impl AttrDebug for GenResolver {
    fn attr_debug(&self) -> String {
        self.a.inner.attr_debug()
    }
}

impl ResolverFn for GenResolver {
    fn no_tx(&self) -> bool {
        self.a.ra.no_tx
    }
    fn no_ctx(&self) -> bool {
        self.a.ra.no_ctx
    }
    fn name(&self) -> Ts2 {
        ts2!(self.a.inner.field_name())
    }
    fn gql_name(&self) -> String {
        camel_str!(self.name())
    }
    fn inputs(&self) -> Ts2 {
        ts2!()
    }
    fn output(&self) -> Ts2 {
        ts2!(self.a.inner.field_ty())
    }
    fn body(&self) -> Ts2 {
        let f = ts2!(self.a.call);
        if self.no_ctx() {
            quote!(#f(self).await?)
        } else {
            quote!(#f(self, ctx).await?)
        }
    }
}
