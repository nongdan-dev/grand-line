use crate::prelude::*;
use std::marker::PhantomData;

pub struct AuthMergedMutation<U: AuthUser>(PhantomData<U>);

impl<U: AuthUser> Default for AuthMergedMutation<U> {
    fn default() -> Self {
        Self(PhantomData)
    }
}

#[Object]
impl<U: AuthUser> AuthMergedMutation<U> {
    async fn register(&self, ctx: &Context<'_>, data: Register) -> Res<AuthOtpWithSecret> {
        ctx.auth_ensure_not_authenticated().await?;
        register_impl::<U>(ctx, data).await
    }

    async fn register_resolve(
        &self,
        ctx: &Context<'_>,
        data: AuthOtpResolve,
    ) -> Res<LoginSessionWithSecret> {
        ctx.auth_ensure_not_authenticated().await?;
        register_resolve_impl::<U>(ctx, data).await
    }

    async fn login(&self, ctx: &Context<'_>, data: Login) -> Res<LoginSessionWithSecret> {
        ctx.auth_ensure_not_authenticated().await?;
        login_impl::<U>(ctx, data).await
    }

    async fn forgot(&self, ctx: &Context<'_>, data: Forgot) -> Res<AuthOtpWithSecret> {
        ctx.auth_ensure_not_authenticated().await?;
        forgot_impl::<U>(ctx, data).await
    }

    async fn forgot_resolve(
        &self,
        ctx: &Context<'_>,
        data: AuthOtpResolve,
        password: String,
    ) -> Res<LoginSessionWithSecret> {
        ctx.auth_ensure_not_authenticated().await?;
        forgot_resolve_impl::<U>(ctx, data, password).await
    }

    async fn auth_otp_resolve(
        &self,
        ctx: &Context<'_>,
        ty: AuthOtpTy,
        data: AuthOtpResolve,
    ) -> Res<AuthOtpGql> {
        ctx.auth_ensure_not_authenticated().await?;
        auth_otp_resolve_impl(ctx, ty, data).await
    }

    async fn logout(&self, ctx: &Context<'_>) -> Res<LoginSessionGql> {
        ctx.auth_ensure_authenticated().await?;
        logout_impl(ctx).await
    }

    async fn login_session_delete(&self, ctx: &Context<'_>, id: String) -> Res<LoginSessionGql> {
        ctx.auth_ensure_authenticated().await?;
        login_session_delete_impl(ctx, id).await
    }

    async fn login_session_delete_all(&self, ctx: &Context<'_>) -> Res<Vec<LoginSessionGql>> {
        ctx.auth_ensure_authenticated().await?;
        login_session_delete_all_impl(ctx).await
    }
}

pub struct AuthMergedQuery;

impl Default for AuthMergedQuery {
    fn default() -> Self {
        Self
    }
}

#[Object]
impl AuthMergedQuery {
    async fn login_session_current(&self, ctx: &Context<'_>) -> Res<Option<LoginSessionGql>> {
        login_session_current_impl(ctx).await
    }

    async fn login_session_search(
        &self,
        ctx: &Context<'_>,
        filter: Option<LoginSessionFilter>,
        order_by: Option<Vec<LoginSessionOrderBy>>,
        page: Option<Pagination>,
    ) -> Res<Vec<LoginSessionGql>> {
        login_session_search_impl(ctx, filter, order_by, page).await
    }

    async fn login_session_count(
        &self,
        ctx: &Context<'_>,
        filter: Option<LoginSessionFilter>,
    ) -> Res<u64> {
        login_session_count_impl(ctx, filter).await
    }
}
