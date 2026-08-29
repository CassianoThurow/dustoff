use std::{
    path::{Path, PathBuf},
    process::Command,
};

use anyhow::Result;
use walkdir::WalkDir;

use crate::model::{CleanupAction, CleanupItem, Risk};

pub fn scan() -> Result<Vec<CleanupItem>> {
    let home = dirs::home_dir().ok_or_else(|| anyhow::anyhow!("home directory not found"))?;
    let mut items = vec![
        directory_item("npm-cache", "npm cache", "Downloaded npm package cache", home.join(".npm/_cacache"), Risk::Low),
        directory_item("npx-cache", "npx cache", "Temporary packages installed by npx", home.join(".npm/_npx"), Risk::Low),
        command_item("pnpm-store", "pnpm store", "Unreferenced packages from the pnpm store", "pnpm", &["store", "prune"], Risk::Low),
        command_item("yarn-cache", "Yarn cache", "Packages cached by Yarn", "yarn", &["cache", "clean"], Risk::Low),
        directory_item("cargo-registry", "Cargo registry cache", "Downloaded Rust crate archives", home.join(".cargo/registry/cache"), Risk::Low),
        directory_item("cargo-git", "Cargo Git cache", "Git dependencies cached by Cargo", home.join(".cargo/git/db"), Risk::Moderate),
        directory_item("thumbnails", "Thumbnail cache", "Desktop-generated image thumbnails", home.join(".cache/thumbnails"), Risk::Low),
        directory_item("trash", "User trash", "Files currently in the desktop trash", home.join(".local/share/Trash/files"), Risk::High),
        command_item("docker-containers", "Docker stopped containers", "All stopped containers", "docker", &["container", "prune", "--force"], Risk::Moderate),
        command_item("docker-images", "Docker unused images", "Dangling Docker images only", "docker", &["image", "prune", "--force"], Risk::Moderate),
        command_item("docker-networks", "Docker unused networks", "Networks unused by any container", "docker", &["network", "prune", "--force"], Risk::Low),
        command_item("docker-build", "Docker build cache", "Unused Docker build cache", "docker", &["builder", "prune", "--force"], Risk::Moderate),
        command_item("docker-volumes", "Docker unused volumes", "Volumes unused by any container; may contain data", "docker", &["volume", "prune", "--force"], Risk::High),
    ];

    items.sort_by_key(|item| item.id);
    Ok(items)
}

fn directory_item(
    id: &'static str,
    label: &'static str,
    description: &'static str,
    path: PathBuf,
    risk: Risk,
) -> CleanupItem {
    let available = path.is_dir();
    let estimated_bytes = available.then(|| directory_size(&path));

    CleanupItem {
        id,
        label,
        description: description.to_owned(),
        estimated_bytes,
        risk,
        action: CleanupAction::RemoveContents(path),
        available,
    }
}

fn command_item(
    id: &'static str,
    label: &'static str,
    description: &'static str,
    program: &'static str,
    args: &'static [&'static str],
    risk: Risk,
) -> CleanupItem {
    CleanupItem {
        id,
        label,
        description: description.to_owned(),
        estimated_bytes: None,
        risk,
        action: CleanupAction::Command { program, args },
        available: command_exists(program),
    }
}

fn command_exists(program: &str) -> bool {
    Command::new("sh")
        .args(["-c", "command -v \"$1\" >/dev/null 2>&1", "sh", program])
        .status()
        .is_ok_and(|status| status.success())
}

fn directory_size(path: &Path) -> u64 {
    WalkDir::new(path)
        .follow_links(false)
        .into_iter()
        .filter_map(Result::ok)
        .filter_map(|entry| entry.metadata().ok())
        .filter(|metadata| metadata.is_file())
        .map(|metadata| metadata.len())
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn missing_directory_is_unavailable() {
        let item = directory_item(
            "missing",
            "Missing",
            "Test",
            PathBuf::from("/a/path/that/does/not/exist"),
            Risk::Low,
        );

        assert!(!item.available);
        assert_eq!(item.estimated_bytes, None);
    }
}
