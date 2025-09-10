use crate::prelude::*;

/// Shortcut for `ulid::Ulid::new().to_string().to_lowercase()`.
pub fn ulid() -> String {
    ulid::Ulid::new().to_string().to_lowercase()
}
