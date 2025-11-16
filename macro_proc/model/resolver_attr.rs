use crate::prelude::*;

#[field_names]
pub struct ResolverAttr {
    pub call: String,
    pub sql_dep: Vec<String>,
    #[field_names(skip)]
    pub ra: ResolverTyAttr,
    #[field_names(skip)]
    pub inner: Attr,
}
impl From<Attr> for ResolverAttr {
    fn from(a: Attr) -> Self {
        Self {
            call: a.str("call").unwrap_or_else(|| {
                let field = a.field_name();
                f!("resolve_{field}")
            }),
            sql_dep: a
                .str("sql_dep")
                .unwrap_or_default()
                .split('+')
                .map(|s| s!(s.trim()))
                .collect(),
            ra: a.clone().into(),
            inner: a,
        }
    }
}
impl AttrValidate for ResolverAttr {
    fn attr_fields(a: &Attr) -> Vec<String> {
        Self::F
            .iter()
            .map(|f| s!(f))
            .chain(ResolverTyAttr::attr_fields(a))
            .collect()
    }
}
