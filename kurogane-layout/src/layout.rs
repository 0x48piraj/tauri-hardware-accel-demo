//! Filesystem layout and low-level bundle utilities.
//!
//! This module owns managed CEF paths, bundled runtime discovery and
//! recursive directory copying.
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

/// Returns the `Contents` directory for a macOS app bundle.
#[cfg(any(target_os = "macos", test))]
fn app_bundle_contents(exe: &Path) -> Option<PathBuf> {
    let macos = exe.parent()?;
    let contents = macos.parent()?;
    let app = contents.parent()?;

    if macos.file_name()? != "MacOS"
        || contents.file_name()? != "Contents"
        || app.extension()? != "app"
    {
        return None;
    }

    Some(contents.to_path_buf())
}

/// Returns the bundled resource directory, if running from a bundle.
pub fn bundled_resource_root() -> Result<Option<PathBuf>, std::io::Error> {
    Ok(bundled_resource_root_for(&std::env::current_exe()?))
}

/// Returns the resource root for a bundled executable.
fn bundled_resource_root_for(exe: &Path) -> Option<PathBuf> {
    #[cfg(any(target_os = "windows", target_os = "linux"))]
    let dir = exe.parent()?;

    #[cfg(target_os = "windows")]
    {
        // Flat bundle; executable, CEF and resources share a directory
        if dir.join("libcef.dll").exists() {
            return Some(dir.to_path_buf());
        }
    }

    #[cfg(target_os = "linux")]
    {
        // The executable is under `runtime/`; `cef/` identifies the bundle
        if dir.join("cef").is_dir() {
            return dir.parent().map(Path::to_path_buf);
        }
    }

    #[cfg(target_os = "macos")]
    {
        if let Some(contents) = app_bundle_contents(exe) {
            return Some(contents.join("Resources"));
        }
    }

    None
}

pub fn bundled_cef_root() -> Result<Option<PathBuf>, std::io::Error> {
    let exe = std::env::current_exe()?;

    let dir = exe.parent().unwrap_or(Path::new("."));

    #[cfg(target_os = "windows")]
    {
        // CEF is next to the executable
        let libcef = dir.join("libcef.dll");

        if libcef.exists() {
            return Ok(Some(dir.to_path_buf()));
        }
    }

    #[cfg(target_os = "linux")]
    {
        // CEF is under `cef/`
        let cef = dir.join("cef");

        if cef.exists() {
            return Ok(Some(cef));
        }
    }

    #[cfg(target_os = "macos")]
    {
        let framework = dir.join("Chromium Embedded Framework.framework");

        if framework.exists() {
            return Ok(Some(dir.to_path_buf()));
        }

        if let Some(contents) = app_bundle_contents(&exe) {
            let frameworks = contents.join("Frameworks");

            if frameworks.join(crate::platform::MACOS_FRAMEWORK).exists() {
                return Ok(Some(frameworks));
            }
        }
    }

    Ok(None)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn app_bundle_contents_found_for_a_bundled_executable() {
        assert_eq!(
            app_bundle_contents(Path::new("/Apps/MyApp.app/Contents/MacOS/myapp")),
            Some(PathBuf::from("/Apps/MyApp.app/Contents"))
        );
    }

    #[test]
    fn app_bundle_contents_rejects_non_bundle_layouts() {
        for exe in [
            "/proj/target/release/myapp",
            "/Apps/MyApp.app/Contents/myapp",
            "/Apps/MyApp/Contents/MacOS/myapp",
            "/myapp",
        ] {
            assert_eq!(
                app_bundle_contents(Path::new(exe)),
                None,
                "{exe} is not inside an .app bundle"
            );
        }
    }

    #[test]
    fn a_cargo_target_directory_is_not_a_bundle() {
        assert_eq!(bundled_resource_root().unwrap(), None);
    }

    #[cfg(target_os = "linux")]
    #[test]
    fn a_linux_bundle_resolves_to_the_bundle_root() {
        let dir = tempfile::tempdir().unwrap();
        let root = dir.path().join("dist");
        let runtime = root.join("runtime");
        std::fs::create_dir_all(runtime.join("cef")).unwrap();

        assert_eq!(
            bundled_resource_root_for(&runtime.join("myapp")),
            Some(root)
        );
    }

    #[cfg(target_os = "linux")]
    #[test]
    fn a_linux_executable_with_no_cef_sibling_is_not_a_bundle() {
        let dir = tempfile::tempdir().unwrap();
        let target = dir.path().join("target").join("debug");
        std::fs::create_dir_all(&target).unwrap();

        assert_eq!(bundled_resource_root_for(&target.join("myapp")), None);
    }

    #[cfg(target_os = "windows")]
    #[test]
    fn a_windows_bundle_resolves_to_the_executable_directory() {
        let dir = tempfile::tempdir().unwrap();
        std::fs::write(dir.path().join("libcef.dll"), b"").unwrap();

        assert_eq!(
            bundled_resource_root_for(&dir.path().join("myapp.exe")),
            Some(dir.path().to_path_buf())
        );
    }

    #[cfg(target_os = "windows")]
    #[test]
    fn a_windows_executable_with_no_libcef_is_not_a_bundle() {
        let dir = tempfile::tempdir().unwrap();

        assert_eq!(
            bundled_resource_root_for(&dir.path().join("myapp.exe")),
            None
        );
    }

    #[cfg(target_os = "macos")]
    #[test]
    fn an_app_bundle_resolves_to_contents_resources() {
        assert_eq!(
            bundled_resource_root_for(Path::new("/Apps/MyApp.app/Contents/MacOS/myapp")),
            Some(PathBuf::from("/Apps/MyApp.app/Contents/Resources"))
        );
    }

    #[cfg(target_os = "macos")]
    #[test]
    fn an_unbundled_macos_executable_is_not_a_bundle() {
        assert_eq!(
            bundled_resource_root_for(Path::new("/proj/target/release/myapp")),
            None
        );
    }
}
