use super::prelude::*;

/// Abstract extra Select methods implementation.
pub trait SelectX<E>
where
    E: EntityX,
{
    /// Helper to filter with option.
    fn filter_opt(self, c: Option<Condition>) -> Self;

    /// Helper to filter with ChainSelect.
    fn chain<C>(self, c: C) -> Self
    where
        C: ChainSelect<E>;

    /// Select only columns from requested fields in the graphql context.
    fn _gql_select(
        self,
        look_ahead: &Vec<LookaheadX<E>>,
        col: E::C,
    ) -> Res<Selector<SelectModel<E::G>>>;
    /// Select only columns from requested fields in the graphql context.
    fn gql_select(self, ctx: &Context<'_>) -> Res<Selector<SelectModel<E::G>>>;

    /// Select only id for the graphql delete response.
    fn gql_select_id(self) -> Res<Selector<SelectModel<E::G>>>;
}

/// Automatically implement for Select<E>.
impl<E> SelectX<E> for Select<E>
where
    E: EntityX,
{
    fn filter_opt(self, c: Option<Condition>) -> Self {
        match c {
            Some(c) => self.filter(c),
            None => self,
        }
    }

    fn chain<C>(self, c: C) -> Self
    where
        C: ChainSelect<E>,
    {
        c.chain_select(self)
    }

    fn _gql_select(
        self,
        look_ahead: &Vec<LookaheadX<E>>,
        col: E::C,
    ) -> Res<Selector<SelectModel<E::G>>> {
        let mut q = self;
        q = q.select_only();
        q = q.select_column(col);
        for l in look_ahead {
            match l.col {
                Some(c) => {
                    if c.as_str() != col.as_str() {
                        q = q.select_column(c)
                    }
                }
                None => {}
            }
            match l.expr.clone() {
                Some(expr) => q = q.column_as(expr, l.c),
                None => {}
            }
        }
        let r = q.into_model::<E::G>();
        Ok(r)
    }
    fn gql_select(self, ctx: &Context<'_>) -> Res<Selector<SelectModel<E::G>>> {
        let look_ahead = E::gql_look_ahead(ctx)?;
        self._gql_select(&look_ahead, E::_col_id()?)
    }

    fn gql_select_id(self) -> Res<Selector<SelectModel<E::G>>> {
        E::_col_id().map(|c| self.select_only().column(c).into_model::<E::G>())
    }
}
