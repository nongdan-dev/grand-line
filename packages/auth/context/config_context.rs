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

pub trait AuthUserConfigContext<'a> {
    fn auth_user_config<U: AuthUser + 'static>(&self) -> Res<&'a AuthUserConfig<U>>;
}

impl<'a> AuthUserConfigContext<'a> for Context<'a> {
    fn auth_user_config<U: AuthUser + 'static>(&self) -> Res<&'a AuthUserConfig<U>> {
        self.data_opt::<AuthUserConfig<U>>()
            .ok_or_else(|| MyErr::AuthUserConfigNotFound.into())
    }
}
