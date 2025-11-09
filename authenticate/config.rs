#[derive(Debug, Clone)]
pub struct GrandLineConfigAuth {
    pub default_ensure: GrandLineConfigAuthEnsure,
    pub cookie_login_session_key: &'static str,
    pub cookie_login_session_expires: i64,
    pub max_otp_attempt: i64,
}

impl Default for GrandLineConfigAuth {
    fn default() -> Self {
        Self {
            default_ensure: GrandLineConfigAuthEnsure::Authenticate,
            cookie_login_session_key: "login_session",
            cookie_login_session_expires: 7 * 24 * 60 * 60 * 1000,
            max_otp_attempt: 5,
        }
    }
}

#[derive(Debug, Clone)]
pub enum GrandLineConfigAuthEnsure {
    None,
    Authenticate,
    Unauthenticated,
    Authorize,
}
