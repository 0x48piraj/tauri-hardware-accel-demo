//! Locating a packaged application's own files at runtime.

use std::path::PathBuf;

/// The directory a packaged application's bundled resources live in.
///
/// Returns `None` when the application is not running from a recognised
/// bundle or its executable path cannot be resolved.
///
/// ```no_run
/// let config = match kurogane::resource_dir() {
///     Some(resources) => resources.join("config.toml"),
///     None => std::path::PathBuf::from("config.toml"),
/// };
/// ```
pub fn resource_dir() -> Option<PathBuf> {
    kurogane_layout::bundled_resource_root().ok().flatten()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn there_is_no_resource_dir_outside_a_bundle() {
        assert_eq!(resource_dir(), None);
    }
}
