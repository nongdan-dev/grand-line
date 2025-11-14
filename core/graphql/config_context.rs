use super::prelude::*;

static DEFAULT: LazyLock<GrandLineCoreConfig> = LazyLock::new(GrandLineCoreConfig::default);

pub trait ConfigContext<'a> {
    fn config(&self) -> &'a GrandLineCoreConfig;
}

impl<'a> ConfigContext<'a> for Context<'a> {
    fn config(&self) -> &'a GrandLineCoreConfig {
        if let Some(cfg) = self.data_opt::<GrandLineCoreConfig>() {
            cfg
        } else {
            &DEFAULT
        }
    }
}
