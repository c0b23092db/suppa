use crate::model::DEFAULT_CONFIG;

use anyhow::{Context, Result, bail};
use dirs::home_dir;
use std::fs::{create_dir_all, write};
use std::path::PathBuf;

pub fn resolve_config_path(config: Option<PathBuf>) -> Result<PathBuf> {
    match config {
        Some(path) => {
            if !path.exists() {
                bail!("config file not found: {}", path.display());
            }
            Ok(path)
        }
        None => {
            let default_path = resolve_default_path()?;
            Ok(default_path)
        }
    }
}

fn resolve_default_path() -> Result<PathBuf> {
    let default_path = home_dir()
        .unwrap_or_else(|| PathBuf::from("~"))
        .join(".config/suppa/config.toml");
    if default_path.exists() {
        return Ok(default_path);
    }
    let parent = default_path.parent()
        .context("config path has no parent")?;
    create_dir_all(parent)
        .context("failed to create config directory")?;
    write(&default_path, DEFAULT_CONFIG)
        .with_context(|| format!("failed to write {DEFAULT_CONFIG:?}"))?;
    Ok(default_path)
}