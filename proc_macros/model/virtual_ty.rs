use crate::prelude::*;
use strum_macros::{Display, EnumString};

#[derive(Debug, Clone, Eq, PartialEq, Display, PartialEqString)]
pub enum VirtualTy {
    #[strum(serialize = "{0}")]
    Relation(RelationTy),
    #[strum(serialize = "sql_expr")]
    SqlExpr,
    #[strum(serialize = "resolver")]
    Resolver,
}
impl VirtualTy {
    pub fn all() -> Vec<VirtualTy> {
        let mut all = RelationTy::all()
            .iter()
            .map(|r| VirtualTy::Relation(r.clone()))
            .collect::<Vec<_>>();
        all.push(VirtualTy::SqlExpr);
        all.push(VirtualTy::Resolver);
        all
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Display, EnumString, PartialEqString)]
pub enum RelationTy {
    #[strum(serialize = "belongs_to")]
    BelongsTo,
    #[strum(serialize = "has_one")]
    HasOne,
    #[strum(serialize = "has_many")]
    HasMany,
    #[strum(serialize = "many_to_many")]
    ManyToMany,
}
impl RelationTy {
    pub fn all() -> Vec<RelationTy> {
        vec![
            RelationTy::BelongsTo,
            RelationTy::HasOne,
            RelationTy::HasMany,
            RelationTy::ManyToMany,
        ]
    }
}
