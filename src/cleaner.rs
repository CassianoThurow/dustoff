use std::{
    fs,
    path::{Path, PathBuf},
    process::Command,
};

use anyhow::{bail, Context, Result};

use crate::model::{CleanupAction, CleanupItem};

pub fn clean(item: &CleanupItem) -> Result<()> {
    match &item.action {
        CleanupAction::RemoveContents(path) => remove_contents(path),
        CleanupAction::Command { program, args } => run_command(program, args),
    }
    .with_context(|| format!("failed to clean {}", item.label))
}

fn remove_contents(path: &Path) -> Result<()> {
    validate_user_path(path)?;

    if !path.exists() {
        return Ok(());
    }

    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let target = entry.path();
        let metadata = fs::symlink_metadata(&target)?;

        if metadata.is_dir() && !metadata.file_type().is_symlink() {
            fs::remove_dir_all(&target)?;
        } else {
            fs::remove_file(&target)?;
        }
    }

    Ok(())
}

fn validate_user_path(path: &Path) -> Result<()> {
    let home = dirs::home_dir().context("home directory not found")?;
    let absolute = make_absolute(path)?;
    let absolute_home = make_absolute(&home)?;

    if absolute == absolute_home || !absolute.starts_with(&absolute_home) {
        bail!("refusing to clean path outside the user home: {}", path.display());
    }

    if absolute.components().count() <= absolute_home.components().count() + 1 {
        bail!("refusing to clean a broad user directory: {}", path.display());
    }

    Ok(())
}

fn make_absolute(path: &Path) -> Result<PathBuf> {
    if path.is_absolute() {
        Ok(path.to_path_buf())
    } else {
        Ok(std::env::current_dir()?.join(path))
    }
}

fn run_command(program: &str, args: &[&str]) -> Result<()> {
    let status = Command::new(program).args(args).status()?;
    if !status.success() {
        bail!("command exited with status {status}");
    }
    Ok(())
}
