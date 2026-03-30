use crate::prelude::*;
use chrono::Duration;

/// Shortcut for `chrono::Utc::now()`.
pub fn now() -> DateTimeUtc {
    chrono::Utc::now()
}

pub fn duration_w(weeks: i64) -> Duration {
    Duration::weeks(weeks)
}
pub fn duration_d(days: i64) -> Duration {
    Duration::days(days)
}
pub fn duration_h(hours: i64) -> Duration {
    Duration::hours(hours)
}
pub fn duration_m(minutes: i64) -> Duration {
    Duration::minutes(minutes)
}
pub fn duration_s(seconds: i64) -> Duration {
    Duration::seconds(seconds)
}
pub fn duration_ms(ms: i64) -> Duration {
    Duration::milliseconds(ms)
}
