use crate::prelude::*;

static DEFAULT: LazyLock<AuthzConfig> = LazyLock::new(AuthzConfig::default);

pub trait AuthzConfigContext<'a> {
    fn authz_config(&self) -> &'a AuthzConfig;
}

impl<'a> AuthzConfigContext<'a> for Context<'a> {
    fn authz_config(&self) -> &'a AuthzConfig {
        if let Some(cfg) = self.data_opt::<AuthzConfig>() {
            cfg
        } else {
            &DEFAULT
        }
    }
}
