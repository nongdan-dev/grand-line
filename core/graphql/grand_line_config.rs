use super::prelude::*;

#[derive(Debug, Clone)]
pub struct GrandLineConfig {
    pub limit_default: u64,
    pub limit_max: u64,
}

impl Default for GrandLineConfig {
    fn default() -> Self {
        Self {
            limit_default: 10,
            limit_max: 100,
        }
    }
}

static DEFAULT: LazyLock<GrandLineConfig> = LazyLock::new(GrandLineConfig::default);

pub trait GrandLineConfigContext<'a> {
    fn config(&self) -> &'a GrandLineConfig;
}

impl<'a> GrandLineConfigContext<'a> for Context<'a> {
    fn config(&self) -> &'a GrandLineConfig {
        if let Some(cfg) = self.data_opt::<GrandLineConfig>() {
            cfg
        } else {
            &DEFAULT
        }
    }
}
