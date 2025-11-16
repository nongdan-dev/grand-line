use super::prelude::*;

static DEFAULT: LazyLock<CoreConfig> = LazyLock::new(CoreConfig::default);

pub trait ConfigContext<'a> {
    fn config(&self) -> &'a CoreConfig;
}

impl<'a> ConfigContext<'a> for Context<'a> {
    fn config(&self) -> &'a CoreConfig {
        if let Some(cfg) = self.data_opt::<CoreConfig>() {
            cfg
        } else {
            &DEFAULT
        }
    }
}
