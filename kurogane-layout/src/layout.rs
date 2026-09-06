//! Filesystem layout and low-level bundle utilities.
//!
//! This module owns paths for managed CEF installations, bundled CEF
//! discovery and recursive directory copying.
//!
//! It does not define package formats or application metadata.

use std::path::{Path, PathBuf};

use crate::platform;

pub fn install_root() -> PathBuf {
    platform::data_local_dir().join("kurogane").join("cef")
}

pub fn cef_install_dir(version: &str) -> PathBuf {
    install_root().join(version)
}

/// Resolves a versioned managed CEF installation if it exists locally.
pub fn installed_cef_root(version: &str) -> Option<PathBuf> {
    let root = cef_install_dir(version);

    root.exists().then_some(root)
}

pub fn copy_dir(src: &Path, dst: &Path) -> std::io::Result<()> {
    std::fs::create_dir_all(dst)?;

    for entry in std::fs::read_dir(src)? {
        let entry = entry?;
        let path = entry.path();
        let dest = dst.join(entry.file_name());

        if path.is_dir() {
            copy_dir(&path, &dest)?;
        } else {
            std::fs::copy(&path, &dest)?;
        }
    }

    Ok(())
}

pub fn bundled_cef_root() -> Result<Option<PathBuf>, std::io::Error> {
    let exe = std::env::current_exe()?;

    Ok(bundled_cef_root_beside(
        exe.parent().unwrap_or(Path::new(".")),
    ))
}

/// Resolves a CEF runtime bundled beside the executable in `dir`.
fn bundled_cef_root_beside(dir: &Path) -> Option<PathBuf> {
    #[cfg(target_os = "windows")]
    {
        // Windows bundle: CEF is flattened next to the exe.
        let libcef = dir.join("libcef.dll");

        if libcef.exists() {
            return Some(dir.to_path_buf());
        }
    }

    #[cfg(target_os = "linux")]
    {
        // Linux: CEF lives in a cef/ subdirectory.
        let cef = dir.join("cef");

        if cef.exists() {
            return Some(cef);
        }
    }

    #[cfg(target_os = "macos")]
    {
        // macOS: the framework sits beside the executable, or in the
        // Contents/Frameworks of the application bundle whose Contents/MacOS
        // the executable runs from.
        for root in [dir.to_path_buf(), dir.join("..").join("Frameworks")] {
            if root.join("Chromium Embedded Framework.framework").exists() {
                return Some(root);
            }
        }
    }

    None
}

#[cfg(all(test, target_os = "macos"))]
mod tests {
    use super::*;
    use std::fs;

    const FRAMEWORK: &str = "Chromium Embedded Framework.framework";

    #[test]
    fn framework_beside_the_executable_is_found() {
        let dir = tempfile::tempdir().unwrap();
        fs::create_dir_all(dir.path().join(FRAMEWORK)).unwrap();

        assert_eq!(
            bundled_cef_root_beside(dir.path()),
            Some(dir.path().to_path_buf())
        );
    }

    #[test]
    fn framework_in_the_bundle_frameworks_dir_is_found() {
        let dir = tempfile::tempdir().unwrap();
        let contents = dir.path().join("App.app").join("Contents");
        let exe_dir = contents.join("MacOS");
        let frameworks = contents.join("Frameworks");
        fs::create_dir_all(&exe_dir).unwrap();
        fs::create_dir_all(frameworks.join(FRAMEWORK)).unwrap();

        assert_eq!(
            bundled_cef_root_beside(&exe_dir),
            Some(exe_dir.join("..").join("Frameworks"))
        );
    }

    #[test]
    fn no_framework_means_no_bundle() {
        let dir = tempfile::tempdir().unwrap();
        fs::create_dir_all(dir.path().join("MacOS")).unwrap();

        assert_eq!(bundled_cef_root_beside(&dir.path().join("MacOS")), None);
    }
}
