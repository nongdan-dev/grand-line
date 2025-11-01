use super::prelude::*;

/// Abstract extra model methods implementation.
#[async_trait]
pub trait ModelXAsync<E>
where
    E: EntityX<M = Self>,
    Self: ModelX<E>,
{
    // Convert sql model to gql model, with checking virtual fields from context.
    async fn into_gql(self, ctx: &Context<'_>) -> Res<E::G> {
        let r = if E::gql_look_ahead(ctx)?.iter().any(|l| l.expr.is_some()) {
            let _tx = ctx.tx().await?;
            let tx = _tx.as_ref();
            let id = self._get_id();
            E::find()
                .by_id(&id)?
                .gql_select(ctx)?
                .one(tx)
                .await?
                .ok_or(MyErr::Db404)?
        } else {
            self._into_gql()
        };
        Ok(r)
    }
}

#[async_trait]
impl<E, M> ModelXAsync<E> for M
where
    E: EntityX<M = M>,
    M: ModelX<E>,
{
}
