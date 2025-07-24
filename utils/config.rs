use crate::*;
use std::sync::atomic::{AtomicU64, Ordering};

pub static CONFIG: LazyLock<GrandLineConfig> = LazyLock::new(|| GrandLineConfig::default());

pub struct GrandLineConfig {
    limit_default: AtomicU64,
    limit_max: AtomicU64,
}

impl Default for GrandLineConfig {
    fn default() -> Self {
        Self {
            limit_default: AtomicU64::new(100),
            limit_max: AtomicU64::new(1000),
        }
    }
}

impl GrandLineConfig {
    pub fn config_limit(&self, limit_default: u64, limit_max: u64) {
        self.limit_default.store(limit_default, Ordering::Relaxed);
        self.limit_max.store(limit_max, Ordering::Relaxed);
    }

    pub fn limit(&self) -> (u64, u64) {
        (
            self.limit_default.load(Ordering::Relaxed),
            self.limit_max.load(Ordering::Relaxed),
        )
    }
}
