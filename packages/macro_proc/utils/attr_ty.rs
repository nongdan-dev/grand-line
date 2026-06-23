use crate::prelude::*;
use strum_macros::{AsRefStr, Display};

#[derive(Clone, Eq, PartialEq, AsRefStr, Display, PartialEqString)]
#[strum(serialize_all = "snake_case")]
pub enum MacroTy {
    Model,
    Search,
    Count,
    Detail,
    Create,
    Update,
    Delete,
}

pub static ATTR_RAW: LazyLock<HashSet<String>> = LazyLock::new(|| {
    let mut set = HashSet::new();
    set.insert(AttrTy::Default.to_string());
    set.insert(VirtualTy::SqlExpr.to_string());
    set
});

#[derive(Clone, Eq, PartialEq, AsRefStr, Display, PartialEqString)]
#[strum(serialize_all = "snake_case")]
pub enum AttrTy {
    Graphql,
    Default,
    #[strum(serialize = "{0}")]
    Virtual(VirtualTy),
}
impl AttrTy {
    pub fn all() -> Vec<Self> {
        let mut all = VirtualTy::all()
            .iter()
            .map(|r| Self::Virtual(r.clone()))
            .collect::<Vec<_>>();
        all.push(Self::Default);
        all
    }
}

#[derive(Clone, Eq, PartialEq, AsRefStr, Display, PartialEqString)]
#[strum(serialize_all = "snake_case")]
pub enum VirtualTy {
    #[strum(serialize = "{0}")]
    Relation(RelationTy),
    SqlExpr,
    Resolver,
}
impl VirtualTy {
    pub fn all() -> Vec<Self> {
        let mut all = RelationTy::all()
            .iter()
            .map(|r| Self::Relation(r.clone()))
            .collect::<Vec<_>>();
        all.push(Self::SqlExpr);
        all.push(Self::Resolver);
        all
    }
}

#[derive(Clone, Eq, PartialEq, AsRefStr, Display, PartialEqString)]
#[strum(serialize_all = "snake_case")]
pub enum RelationTy {
    BelongsTo,
    HasOne,
    HasMany,
    ManyToMany,
}
impl RelationTy {
    pub fn all() -> Vec<Self> {
        vec![Self::BelongsTo, Self::HasOne, Self::HasMany, Self::ManyToMany]
    }
}
