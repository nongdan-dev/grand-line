#[derive(Clone)]
pub struct GrandLineCoreConfig {
    pub limit_default: u64,
    pub limit_max: u64,
}

impl Default for GrandLineCoreConfig {
    fn default() -> Self {
        Self {
            limit_default: 10,
            limit_max: 100,
        }
    }
}
