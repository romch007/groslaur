use anyhow::{bail, Result};
use std::{path::PathBuf, process::Command};

pub fn strip_package_version(package: &str) -> &str {
    &package[..package
        .find(|ch: char| ch == '>' || ch == '=' || ch == '<')
        .unwrap_or(package.len())]
}

pub fn install_packages(packages: &[&str]) -> Result<()> {
    let status = Command::new("sudo")
        .arg("pacman")
        .arg("-S")
        .arg("-q")
        .arg("--needed")
        .arg("--asdeps")
        .arg("--noconfirm")
        .args(packages)
        .status()?;

    if status.success() {
        Ok(())
    } else {
        bail!("non-zero exit code during pacman -S");
    }
}

pub fn is_package_installed(package: &str) -> Result<bool> {
    let output = Command::new("pacman").arg("-Q").arg(package).output()?;

    let stdout = String::from_utf8(output.stdout)?;

    if output.status.success() {
        Ok(true)
    } else if stdout.contains("not found") {
        Ok(false)
    } else {
        bail!("non-zero exit code during pacman -Q");
    }
}

pub fn install_local_pacakges(packages: &[PathBuf]) -> Result<()> {
    let status = Command::new("sudo")
        .arg("pacman")
        .arg("--noconfirm")
        .arg("-U")
        .args(packages)
        .status()?;

    if status.success() {
        Ok(())
    } else {
        bail!("non-zero exit code during pacman -U");
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn strip_package_version() {
        assert_eq!(super::strip_package_version("alsa-lib>=1.0.14"), "alsa-lib");
        assert_eq!(super::strip_package_version("alsa-lib"), "alsa-lib");
    }
}
