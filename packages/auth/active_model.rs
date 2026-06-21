use crate::prelude::*;

#[async_trait]
pub trait AmExec: Sized {
    type Model: Send;
    async fn exec(self, ctx: &Context<'_>) -> Res<Self::Model>;
}

#[async_trait]
impl<E, A> AmExec for ActiveModelWrapper<AmCreate, E, A>
where
    E: EntityX<A = A>,
    A: ActiveModelX<E> + Send,
    E::M: Send,
{
    type Model = E::M;

    async fn exec(self, ctx: &Context<'_>) -> Res<Self::Model> {
        let mut am = self.into_active_model();
        if !am.get_created_by_id().is_set() && E::col_created_by_id().is_some() {
            let user_id = ctx.auth().await?;
            am = am.set_created_by_id(Some(user_id));
        }
        let tx = &*ctx.tx().await?;
        let r = am.insert(tx).await?;
        Ok(r)
    }
}

#[async_trait]
impl<E, A> AmExec for ActiveModelWrapper<AmUpdate, E, A>
where
    E: EntityX<A = A>,
    A: ActiveModelX<E> + Send,
    E::M: Send,
{
    type Model = E::M;

    async fn exec(self, ctx: &Context<'_>) -> Res<Self::Model> {
        let mut am = self.into_active_model();
        if !am.get_updated_by_id().is_set() && E::col_updated_by_id().is_some() {
            let user_id = ctx.auth().await?;
            am = am.set_updated_by_id(Some(user_id));
        }
        let tx = &*ctx.tx().await?;
        let r = am.update(tx).await?;
        Ok(r)
    }
}

#[async_trait]
impl<E, A> AmExec for ActiveModelWrapper<AmSoftDelete, E, A>
where
    E: EntityX<A = A>,
    A: ActiveModelX<E> + Send,
    E::M: Send,
{
    type Model = E::M;

    async fn exec(self, ctx: &Context<'_>) -> Res<Self::Model> {
        let mut am = self.into_active_model();
        if !am.get_deleted_by_id().is_set() && E::col_deleted_by_id().is_some() {
            let user_id = ctx.auth().await?;
            am = am.set_deleted_by_id(Some(user_id));
        }
        let tx = &*ctx.tx().await?;
        let r = am.update(tx).await?;
        Ok(r)
    }
}
