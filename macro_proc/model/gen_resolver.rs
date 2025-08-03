use crate::prelude::*;

#[field_names]
pub struct ResolverAttr {
    pub call: String,
    pub sql_dep: Vec<String>,
    #[field_names(skip)]
    pub resolver_attr: ResolverTyAttr,
    #[field_names(skip)]
    pub inner: Attr,
}
impl From<Attr> for ResolverAttr {
    fn from(a: Attr) -> Self {
        Self {
            call: a
                .str("call")
                .unwrap_or_else(|| f!("resolve_{}", a.field_name())),
            sql_dep: a
                .str("sql_dep")
                .unwrap_or_default()
                .split('+')
                .map(|s| s.trim().to_string())
                .collect(),
            resolver_attr: a.clone().into(),
            inner: a,
        }
    }
}
impl AttrValidate for ResolverAttr {
    fn attr_fields(a: &Attr) -> Vec<String> {
        let mut f = Self::F.iter().map(|f| s!(f)).collect::<Vec<_>>();
        f.extend(ResolverTyAttr::attr_fields(a));
        f
    }
}

pub struct GenResolver {
    pub a: ResolverAttr,
}

impl VirtualGen for GenResolver {
    fn sql_deps(&self) -> Vec<String> {
        self.a.sql_dep.clone()
    }
}
impl AttrDebug for GenResolver {
    fn attr_debug(&self) -> String {
        self.a.inner.attr_debug()
    }
}

impl GenResolverFn for GenResolver {
    fn no_tx(&self) -> bool {
        self.a.resolver_attr.no_tx
    }

    fn no_ctx(&self) -> bool {
        self.a.resolver_attr.no_ctx
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
