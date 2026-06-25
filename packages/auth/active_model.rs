use crate::prelude::*;

// ============================================================================
// IntoActiveModelCtx - sets audit fields (created/updated/deleted_by_id) from ctx

#[async_trait]
pub trait IntoActiveModelCtx<A> {
    async fn into_active_model(self, ctx: &Context<'_>) -> Res<A>;
}

#[async_trait]
impl<E, A> IntoActiveModelCtx<A> for ActiveModelWrapper<AmCreate, E, A>
where
    E: EntityX<A = A>,
    A: ActiveModelX<E> + Send,
{
    async fn into_active_model(self, ctx: &Context<'_>) -> Res<A> {
        let mut am = self.into_active_model_without_ctx();
        if !am.get_created_by_id().is_set() && E::col_created_by_id().is_some() {
            let user_id = ctx.auth().await?;
            am = am.set_created_by_id(Some(user_id));
        }
        Ok(am)
    }
}

#[async_trait]
impl<E, A> IntoActiveModelCtx<A> for ActiveModelWrapper<AmUpdate, E, A>
where
    E: EntityX<A = A>,
    A: ActiveModelX<E> + Send,
{
    async fn into_active_model(self, ctx: &Context<'_>) -> Res<A> {
        let mut am = self.into_active_model_without_ctx();
        if !am.get_updated_by_id().is_set() && E::col_updated_by_id().is_some() {
            let user_id = ctx.auth().await?;
            am = am.set_updated_by_id(Some(user_id));
        }
        Ok(am)
    }
}

#[async_trait]
impl<E, A> IntoActiveModelCtx<A> for ActiveModelWrapper<AmSoftDelete, E, A>
where
    E: EntityX<A = A>,
    A: ActiveModelX<E> + Send,
{
    async fn into_active_model(self, ctx: &Context<'_>) -> Res<A> {
        let mut am = self.into_active_model_without_ctx();
        if !am.get_deleted_by_id().is_set() && E::col_deleted_by_id().is_some() {
            let user_id = ctx.auth().await?;
            am = am.set_deleted_by_id(Some(user_id));
        }
        Ok(am)
    }
}

// ============================================================================
// AmExec - resolves ctx, builds active model, runs db operation

#[async_trait]
pub trait AmExecCtx: Sized {
    type Model: Send;
    async fn exec(self, ctx: &Context<'_>) -> Res<Self::Model>;
}

#[async_trait]
impl<E, A> AmExecCtx for ActiveModelWrapper<AmCreate, E, A>
where
    E: EntityX<A = A>,
    A: ActiveModelX<E> + Send,
    E::M: Send,
{
    type Model = E::M;

    async fn exec(self, ctx: &Context<'_>) -> Res<Self::Model> {
        let am = self.into_active_model(ctx).await?;
        let tx = &*ctx.tx().await?;
        let r = am.insert(tx).await?;
        Ok(r)
    }
}

#[async_trait]
impl<E, A> AmExecCtx for ActiveModelWrapper<AmUpdate, E, A>
where
    E: EntityX<A = A>,
    A: ActiveModelX<E> + Send,
    E::M: Send,
{
    type Model = E::M;

    async fn exec(self, ctx: &Context<'_>) -> Res<Self::Model> {
        let am = self.into_active_model(ctx).await?;
        let tx = &*ctx.tx().await?;
        let r = am.update(tx).await?;
        Ok(r)
    }
}

#[async_trait]
impl<E, A> AmExecCtx for ActiveModelWrapper<AmSoftDelete, E, A>
where
    E: EntityX<A = A>,
    A: ActiveModelX<E> + Send,
    E::M: Send,
{
    type Model = E::M;

    async fn exec(self, ctx: &Context<'_>) -> Res<Self::Model> {
        let am = self.into_active_model(ctx).await?;
        let tx = &*ctx.tx().await?;
        let r = am.update(tx).await?;
        Ok(r)
    }
}
