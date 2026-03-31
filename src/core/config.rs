use crate::model::{Config, DEFAULT_CONFIG, LabelDefinition, SerdeConfig};
use anyhow::{Context, Result, anyhow, bail};
use dirs::home_dir;
use std::env::current_dir;
use std::fs::{create_dir_all, read_to_string, write};
use std::path::{Path, PathBuf};
use toml::Value as TomlValue;

/// Resolve the config file path, either from the provided argument or by using the default location
pub fn resolve_config_path(config_path: Option<PathBuf>) -> Result<PathBuf> {
    let config_path = match config_path {
        Some(path) => {
            if !path.exists() {
                bail!("config file not found: {}", path.display());
            }
            path
        }
        None => resolve_default_path()?,
    };
    Ok(config_path)
}

/// 設定ファイルを読み込む。
/// - initコマンドでカレントディレクトリに作成されるsuppa.tomlを優先的に読み込む
/// - それがなければ、ホームディレクトリの.config/suppa/config.tomlを読み込む
/// - どちらもなければ、ホームディレクトリの.config/suppa/config.tomlを作成して読み込む
fn resolve_default_path() -> Result<PathBuf> {
    let default_path = current_dir()
        .with_context(|| "Failed to get current directory")?
        .join("suppa.toml");
    if default_path.exists() {
        return Ok(default_path);
    }
    let default_path = home_dir()
        .unwrap_or_else(|| PathBuf::from("~"))
        .join(".config")
        .join("suppa/config.toml");
    if default_path.exists() {
        return Ok(default_path);
    }
    create_default_config(&default_path, false)?;
    Ok(default_path)
}

/// デフォルトファイルをカレントディレクトリに作成する
pub fn run_init() -> Result<()> {
    let default_path = current_dir()
        .with_context(|| "Failed to get current directory")?
        .join("suppa.toml");
    create_default_config(&default_path, true)?;
    Ok(())
}

/// デフォルトファイルを作成する
fn create_default_config(path: &Path, text:bool) -> Result<()> {
    if path.exists() {
        bail!("config file already exists: {}", path.display());
    }
    let parent = path.parent().context("config path has no parent")?;
    create_dir_all(parent).context("Failed to Create config directory")?;
    write(path, DEFAULT_CONFIG)
        .with_context(|| format!("Failed to Write {DEFAULT_CONFIG:?}"))?;
    if text {
        println!("Initialized default config file at: {}", path.display());
    }
    Ok(())
}

/// 指定されたパスから設定ファイルを読み込む
pub fn load_config(path: &Path) -> Result<Config> {
    let content = read_to_string(path)
        .with_context(|| format!("Failed to read config file: {}", path.display()))?;
    let value: TomlValue = toml::from_str(&content)
        .with_context(|| format!("Failed to parse TOML config: {}", path.display()))?;

    let table = value
        .as_table()
        .ok_or_else(|| anyhow!("top-level config must be a table"))?;

    let comments = table
        .get("comment")
        .and_then(TomlValue::as_array)
        .map(|arr| {
            arr.iter()
                .filter_map(TomlValue::as_str)
                .map(ToString::to_string)
                .collect::<Vec<_>>()
        })
        .unwrap_or_else(|| vec!["//".to_string(), "#".to_string(), "--".to_string()]);
    let exclude = table
        .get("exclude")
        .and_then(TomlValue::as_array)
        .map(|arr| {
            arr.iter()
                .filter_map(TomlValue::as_str)
                .map(ToString::to_string)
                .collect::<Vec<_>>()
        })
        .unwrap_or_else(|| vec!["md".to_string()]);

    let mut labels = Vec::new();
    for (key, val) in table {
        if key == "comment" || key == "exclude" {
            continue;
        }

        let label_cfg: SerdeConfig = val
            .clone()
            .try_into()
            .with_context(|| format!("Invalid label config for [{key}]"))?;

        let mut terms = vec![key.to_string()];
        terms.extend(label_cfg.alias.iter().cloned());

        labels.push(LabelDefinition {
            label: key.to_string(),
            enabled: label_cfg.enabled,
            update: label_cfg.update,
            mark: label_cfg.mark,
            checkbox: label_cfg.checkbox,
            alias: terms,
        });
    }

    if labels.is_empty() {
        bail!("config must contain at least one label table like [TODO]");
    }

    Ok(Config {
        comments,
        exclude,
        labels,
    })
}
