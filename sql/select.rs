use crate::prelude::*;

/// Abstract extra Select methods implementation.
pub trait SelectX<T, M, A, F, O, G>
where
    T: EntityX<M, A, F, O, G>,
    M: FromQueryResult + Send + Sync,
    A: ActiveModelTrait<Entity = T>,
    F: Filter<T>,
    O: OrderBy<T>,
    G: FromQueryResult + Send + Sync,
{
    /// Helper to filter with option.
    fn filter_opt(self, c: Option<Condition>) -> Self;
    /// Helper to filter with Chainable.
    fn chain<C>(self, c: C) -> Self
    where
        C: Chainable<T>;
    /// Select only columns from requested fields in the graphql context.
    fn gql_select(self, ctx: &Context<'_>) -> Res<Selector<SelectModel<G>>>;
    /// Select only id for the graphql delete response.
    fn gql_select_id(self) -> Res<Selector<SelectModel<G>>>;
}

/// Automatically implement for Select<T>.
impl<T, M, A, F, O, G> SelectX<T, M, A, F, O, G> for Select<T>
where
    T: EntityX<M, A, F, O, G>,
    M: FromQueryResult + Send + Sync,
    A: ActiveModelTrait<Entity = T>,
    F: Filter<T>,
    O: OrderBy<T>,
    G: FromQueryResult + Send + Sync,
{
    fn filter_opt(self, c: Option<Condition>) -> Self {
        match c {
            Some(c) => self.filter(c),
            None => self,
        }
    }

    fn chain<C>(self, c: C) -> Self
    where
        C: Chainable<T>,
    {
        c.chain(self)
    }

    fn gql_select(self, ctx: &Context<'_>) -> Res<Selector<SelectModel<G>>> {
        let mut q = self;
        let cols = T::gql_look_ahead(ctx)?;
        if cols.len() > 0 {
            q = q.select_only();
            for (c, col, expr) in cols {
                match col {
                    Some(col) => q = q.select_column(col),
                    None => {}
                }
                match expr {
                    Some(expr) => q = q.column_as(expr, c),
                    None => {}
                }
            }
        }
        let r = q.into_model::<G>();
        Ok(r)
    }

    fn gql_select_id(self) -> Res<Selector<SelectModel<G>>> {
        T::conf_col_id().map(|c| self.select_only().column(c).into_model::<G>())
    }
}
