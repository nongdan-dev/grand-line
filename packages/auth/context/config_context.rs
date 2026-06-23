use crate::prelude::*;

static DEFAULT: LazyLock<AuthConfig> = LazyLock::new(AuthConfig::default);

pub trait AuthConfigContext<'a> {
    fn auth_config(&self) -> &'a AuthConfig;
}

impl<'a> AuthConfigContext<'a> for Context<'a> {
    fn auth_config(&self) -> &'a AuthConfig {
        if let Some(cfg) = self.data_opt::<AuthConfig>() {
            cfg
        } else {
            &DEFAULT
        }
    }
}

static DEFAULT_USER_IMPL: LazyLock<AuthUserImpl> = LazyLock::new(AuthUserImpl::default);

pub trait AuthUserImplContext<'a> {
    fn auth_user_impl(&self) -> &'a AuthUserImpl;
}

impl<'a> AuthUserImplContext<'a> for Context<'a> {
    fn auth_user_impl(&self) -> &'a AuthUserImpl {
        if let Some(cfg) = self.data_opt::<AuthUserImpl>() {
            cfg
        } else {
            &DEFAULT_USER_IMPL
        }
    }
}
