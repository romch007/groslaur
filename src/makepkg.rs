use anyhow::{bail, Result};
use std::{
    fs,
    path::{Path, PathBuf},
    process::Command,
};

pub fn build(dir: &str) -> Result<()> {
    let status = Command::new("makepkg").current_dir(dir).status()?;

    if status.success() {
        Ok(())
    } else {
        bail!("non-zero exit code during makepkg");
    }
}

pub fn find_pkg_files(dir: impl AsRef<Path>) -> Result<Vec<PathBuf>> {
    let dir = fs::read_dir(dir)?;
    let mut pkgs = Vec::new();

    for entry in dir {
        let entry = entry?;
        let os_filename = entry.file_name();
        let filename = match os_filename.to_str() {
            Some(filename) => filename,
            None => continue,
        };
        if filename.ends_with(".pkg.tar.zst") {
            pkgs.push(entry.path());
        }
    }

    Ok(pkgs)
}
