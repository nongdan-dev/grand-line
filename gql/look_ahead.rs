use crate::prelude::*;
use sea_query::SimpleExpr;

#[derive(Clone)]
pub struct LookaheadX<E>
where
    E: EntityX,
{
    pub c: &'static str,
    pub col: Option<E::C>,
    pub expr: Option<SimpleExpr>,
}
