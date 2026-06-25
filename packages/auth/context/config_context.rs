use crate::prelude::*;

static DEFAULT: LazyLock<AuthConfig> = LazyLock::new(AuthConfig::default);

pub trait AuthConfigContext<'a>
where
    Self: CoreContext<'a>,
{
    fn auth_config(&self) -> &'a AuthConfig {
        if let Some(cfg) = self.data_opt_impl::<AuthConfig>() {
            cfg
        } else {
            &DEFAULT
        }
    }
}

impl<'a> AuthConfigContext<'a> for Context<'a> {
}
