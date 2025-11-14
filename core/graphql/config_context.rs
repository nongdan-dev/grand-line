use super::prelude::*;

static DEFAULT: LazyLock<GrandLineConfig> = LazyLock::new(GrandLineConfig::default);

pub trait ConfigContext<'a> {
    fn config(&self) -> &'a GrandLineConfig;
}

impl<'a> ConfigContext<'a> for Context<'a> {
    fn config(&self) -> &'a GrandLineConfig {
        if let Some(cfg) = self.data_opt::<GrandLineConfig>() {
            cfg
        } else {
            &DEFAULT
        }
    }
}
