//! Creates macOS DMG disk-image creation.

use std::fs;
use std::path::Path;

use anyhow::{Context, Result, bail};

use crate::tui;

/// Returns the DMG path for an application name.
fn dmg_path(output_dir: &Path, name: &str) -> std::path::PathBuf {
    output_dir.join(format!("{name}.dmg"))
}

/// Creates a compressed DMG containing the application bundle.
pub fn build(app_dir: &Path, output_dir: &Path, name: &str) -> Result<std::path::PathBuf> {
    let dmg_path = dmg_path(output_dir, name);

    if dmg_path.exists() {
        fs::remove_file(&dmg_path)
            .with_context(|| format!("failed to remove {}", dmg_path.display()))?;
    }

    let status = std::process::Command::new("hdiutil")
        .arg("create")
        .arg("-volname")
        .arg(name)
        .arg("-srcfolder")
        .arg(app_dir)
        .arg("-ov")
        .arg("-format")
        .arg("UDZO")
        .arg(&dmg_path)
        .status()?;

    if !status.success() {
        bail!("hdiutil create failed; macOS tools are required to build a DMG");
    }

    tui::field("dmg", tui::format_path(&dmg_path));

    Ok(dmg_path)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dmg_is_named_after_the_app_and_written_beside_it() {
        assert_eq!(
            dmg_path(Path::new("/proj/dist"), "MyApp"),
            Path::new("/proj/dist/MyApp.dmg")
        );
    }
}
