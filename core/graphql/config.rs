use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct GrandLineConfig {
    pub limit_default: u64,
    pub limit_max: u64,
    #[cfg(feature = "authenticate")]
    pub auth: GrandLineConfigAuth,
}

impl Default for GrandLineConfig {
    fn default() -> Self {
        Self {
            limit_default: 10,
            limit_max: 100,
            #[cfg(feature = "authenticate")]
            auth: Default::default(),
        }
    }
}
