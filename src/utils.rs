//! Common utility functions
use std::time::SystemTime;

/// Seconds since 1970.
pub fn unix_time() -> u64 {
    SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .map(|x| x.as_secs())
        .unwrap_or(0)
}

/// Check if a string contains only hex characters.
pub fn is_hex(s: &str) -> bool {
    s.chars().all(|x| char::is_ascii_hexdigit(&x))
}
