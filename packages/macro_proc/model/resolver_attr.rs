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
            call: a.str(Self::FIELD_CALL).unwrap_or_else(|| {
                let field = a.field_name();
                format!("resolve_{field}")
            }),
            sql_dep: a
                .str(Self::FIELD_SQL_DEP)
                .unwrap_or_default()
                .split(',')
                .map(|s| s.trim().to_owned())
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
            .copied()
            .map(|f| f.to_owned())
            .chain(ResolverTyAttr::attr_fields(a))
            .collect()
    }
}
