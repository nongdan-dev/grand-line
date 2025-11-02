#[derive(Debug, Clone)]
pub struct GrandLineConfig {
    pub limit_default: u64,
    pub limit_max: u64,
    #[cfg(feature = "authenticate")]
    pub auth_default: GrandLineConfigAuth,
}

impl Default for GrandLineConfig {
    fn default() -> Self {
        Self {
            limit_default: 10,
            limit_max: 100,
            #[cfg(feature = "authenticate")]
            auth_default: GrandLineConfigAuth::Authenticate,
        }
    }
}

#[cfg(feature = "authenticate")]
#[derive(Debug, Clone)]
pub enum GrandLineConfigAuth {
    None,
    Authenticate,
    Unauthenticated,
    Authorize,
}
