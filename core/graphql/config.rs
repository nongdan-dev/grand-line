#[derive(Clone)]
pub struct CoreConfig {
    pub limit_default: u64,
    pub limit_max: u64,
}

impl Default for CoreConfig {
    fn default() -> Self {
        Self {
            limit_default: 10,
            limit_max: 100,
        }
    }
}
