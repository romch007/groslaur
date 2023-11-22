use anyhow::{bail, Result};
use std::{path::PathBuf, process::Command};

pub fn clone(url: &str) -> Result<()> {
    let status = Command::new("git").arg("clone").arg(url).status()?;

    if status.success() {
        Ok(())
    } else {
        bail!("non-zero exit code during git clone");
    }
}

pub fn clean(repo: &str) -> Result<()> {
    let status = Command::new("git")
        .arg("clean")
        .arg("-fdx")
        .current_dir(repo)
        .status()?;

    if status.success() {
        Ok(())
    } else {
        bail!("non-zero exit code during git clean");
    }
}

pub fn pull(repo: &PathBuf) -> Result<bool> {
    let output = Command::new("git").arg("pull").current_dir(repo).output()?;

    if !output.status.success() {
        bail!("non-zero exit code during git fetch");
    }

    let stdout = String::from_utf8(output.stdout)?;

    Ok(!stdout.contains("Already"))
}
