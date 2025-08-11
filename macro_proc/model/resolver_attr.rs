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
            call: a
                .str("call")
                .unwrap_or_else(|| f!("resolve_{}", a.field_name())),
            sql_dep: a
                .str("sql_dep")
                .unwrap_or_default()
                .split('+')
                .map(|s| s.trim().to_string())
                .collect(),
            ra: a.clone().into(),
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
