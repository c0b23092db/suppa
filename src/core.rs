mod create_config;
mod create_markdown;
mod search;

pub use create_config::resolve_config_path;
pub use create_markdown::build_markdown;
pub use create_markdown::create_markdown;
pub use create_markdown::update_markdown;
pub use search::collect_annotations;

use crate::model::{Config, LabelConfig, LabelDefinition};

use anyhow::{Context, Result, anyhow, bail};
use std::fs::read_to_string;
use std::path::Path;
use toml::Value as TomlValue;

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
        .unwrap_or_else(|| vec!["//".to_string(), "#".to_string()]);

    if comments.is_empty() {
        bail!("config.comment must contain at least one comment token");
    }

    let mut labels = Vec::new();
    for (key, val) in table {
        if key == "comment" {
            continue;
        }

        let label_cfg: LabelConfig = val
            .clone()
            .try_into()
            .with_context(|| format!("invalid label config for [{key}]"))?;

        let mut terms = vec![key.to_string()];
        terms.extend(label_cfg.alias.iter().cloned());

        labels.push(LabelDefinition {
            name: key.to_string(),
            mark: label_cfg.mark,
            checkbox: label_cfg.checkbox,
            terms,
        });
    }

    if labels.is_empty() {
        bail!("config must contain at least one label table like [TODO]");
    }

    Ok(Config { comments, labels })
}
