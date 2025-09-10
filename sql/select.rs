use crate::prelude::*;

/// Abstract extra Select methods implementation.
pub trait SelectX<T>
where
    T: EntityX,
{
    /// Helper to filter with option.
    fn filter_opt(self, c: Option<Condition>) -> Self;

    /// Helper to filter with Chainable.
    fn chain<C>(self, c: C) -> Self
    where
        C: Chainable<T>;

    /// Select only columns from requested fields in the graphql context.
    fn gql_select(self, ctx: &Context<'_>) -> Res<Selector<SelectModel<T::G>>>;

    /// Select only id for the graphql delete response.
    fn gql_select_id(self) -> Res<Selector<SelectModel<T::G>>>;
}

/// Automatically implement for Select<T>.
impl<T> SelectX<T> for Select<T>
where
    T: EntityX,
{
    /// Helper to filter with option.
    fn filter_opt(self, c: Option<Condition>) -> Self {
        match c {
            Some(c) => self.filter(c),
            None => self,
        }
    }

    /// Helper to filter with Chainable.
    fn chain<C>(self, c: C) -> Self
    where
        C: Chainable<T>,
    {
        c.chain(self)
    }

    /// Select only columns from requested fields in the graphql context.
    fn gql_select(self, ctx: &Context<'_>) -> Res<Selector<SelectModel<T::G>>> {
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
        let r = q.into_model::<T::G>();
        Ok(r)
    }

    /// Select only id for the graphql delete response.
    fn gql_select_id(self) -> Res<Selector<SelectModel<T::G>>> {
        T::_col_id().map(|c| self.select_only().column(c).into_model::<T::G>())
    }
}
