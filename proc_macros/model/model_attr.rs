use crate::prelude::*;
use field_names::FieldNames;

#[derive(FieldNames)]
pub struct ModelAttr {
    pub no_created_at: bool,
    pub no_updated_at: bool,
    pub no_deleted_at: bool,
    pub no_by_id: bool,
    pub limit_default: u64,
    pub limit_max: u64,
}

impl From<Attr> for ModelAttr {
    fn from(a: Attr) -> Self {
        let f = Self::FIELDS;
        a.validate(f.to_vec());
        Self {
            no_created_at: a.bool(f[0]),
            no_updated_at: a.bool(f[1]),
            no_deleted_at: a.bool(f[2]),
            no_by_id: a.bool(f[3]),
            limit_default: a.parse_opt(f[4]).unwrap_or(10),
            limit_max: a.parse_opt(f[5]).unwrap_or(100),
        }
    }
}
