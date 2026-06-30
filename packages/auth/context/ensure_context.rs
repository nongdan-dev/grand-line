use crate::prelude::*;

pub enum AuthEnsure {
    Authenticated,
    Unauthenticated,
}

#[async_trait]
pub trait AuthEnsureContext<'a>
where
    Self: AuthCacheContext<'a>,
{
    async fn auth_ensure_in_macro(&self, check: AuthEnsure) -> Res<()> {
        match check {
            AuthEnsure::Authenticated => self.auth_ensure_authenticated().await?,
            AuthEnsure::Unauthenticated => self.auth_ensure_not_authenticated().await?,
        }
        Ok(())
    }

    async fn auth_ensure_authenticated(&self) -> Res<()> {
        if self.auth_unchecked().await?.as_ref().is_none() {
            return Err(MyErr::Unauthenticated.into());
        }
        Ok(())
    }

    async fn auth_ensure_not_authenticated(&self) -> Res<()> {
        if self.auth_unchecked().await?.as_ref().is_some() {
            return Err(MyErr::AlreadyAuthenticated.into());
        }
        Ok(())
    }
}

#[async_trait]
impl<'a> AuthEnsureContext<'a> for Context<'a> {
}
