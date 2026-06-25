use crate::prelude::*;

#[async_trait]
pub trait AuthEnsureContext<'a>
where
    Self: AuthCacheContext<'a>,
{
    async fn auth_ensure_in_macro(&self, check: AuthDirectiveCheck) -> Res<()> {
        match check {
            AuthDirectiveCheck::Authenticated => self.auth_ensure_authenticated().await?,
            AuthDirectiveCheck::Unauthenticated => self.auth_ensure_not_authenticated().await?,
        }
        Ok(())
    }

    async fn auth_ensure_authenticated(&self) -> Res<()> {
        if self.auth_unchecked().await?.as_ref().0.is_none() {
            return Err(MyErr::Unauthenticated.into());
        }
        Ok(())
    }

    async fn auth_ensure_not_authenticated(&self) -> Res<()> {
        if self.auth_unchecked().await?.as_ref().0.is_some() {
            return Err(MyErr::AlreadyAuthenticated.into());
        }
        Ok(())
    }
}

#[async_trait]
impl<'a> AuthEnsureContext<'a> for Context<'a> {
}
