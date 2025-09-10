use crate::prelude::*;

/// Shortcut for `chrono::Utc::now()`.
pub fn now() -> DateTimeUtc {
    chrono::Utc::now()
}
