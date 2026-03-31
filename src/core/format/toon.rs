use crate::core::{hashmap_annotations, project_name_from_root};
use crate::model::{Annotation, Config};
use anyhow::{Context, Result};
use serde::Serialize;
use serde_json::Value;
use std::fs::write;
use std::path::Path;
use toon_format::encode_default;

pub fn create_toon(
    path: &Path,
    root: &Path,
    config: &Config,
    annotations: &[Annotation],
) -> Result<()> {
    let toon = build_toon(root, config, annotations);
    write(path, toon)
        .with_context(|| format!("Failed to write output file: {}", path.display()))?;
    Ok(())
}

pub fn build_toon(root: &Path, config: &Config, annotations: &[Annotation]) -> String {
    #[derive(Serialize)]
    struct CheckboxRow {
        check: bool,
        path: String,
        line: u64,
        context: String,
    }

    #[derive(Serialize)]
    struct PlainRow {
        path: String,
        line: u64,
        context: String,
    }

    let project_name = project_name_from_root(root);
    let grouped = hashmap_annotations(config, annotations);
    let mut labels = serde_json::Map::new();

    for label in &config.labels {
        if !label.enabled {
            continue;
        }

        let display_name = format_label(label.mark.as_deref(), &label.label);

        if label.checkbox {
            let rows: Vec<CheckboxRow> = grouped
                .get(label.label.as_str())
                .map(|items| {
                    items
                        .iter()
                        .map(|item| CheckboxRow {
                            check: false,
                            path: format!("{}", item.file.display()),
                            line: item.line,
                            context: item.content.clone(),
                        })
                        .collect()
                })
                .unwrap_or_default();

            labels.insert(
                display_name,
                serde_json::to_value(rows).unwrap_or(Value::Array(Vec::new())),
            );
        } else {
            let rows: Vec<PlainRow> = grouped
                .get(label.label.as_str())
                .map(|items| {
                    items
                        .iter()
                        .map(|item| PlainRow {
                            path: format!("{}", item.file.display()),
                            line: item.line,
                            context: item.content.clone(),
                        })
                        .collect()
                })
                .unwrap_or_default();

            labels.insert(
                display_name,
                serde_json::to_value(rows).unwrap_or(Value::Array(Vec::new())),
            );
        }
    }

    let mut root_map = serde_json::Map::new();
    root_map.insert(project_name, Value::Object(labels));

    match encode_default(&root_map) {
        Ok(mut text) => {
            if !text.ends_with('\n') {
                text.push('\n');
            }
            text
        }
        Err(_) => String::new(),
    }
}

fn format_label(mark: Option<&str>, label: &str) -> String {
    match mark {
        Some(mark) if !mark.trim().is_empty() => format!("{} {}", mark.trim(), label),
        _ => label.to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::build_toon;
    use crate::model::{Annotation, Config, LabelDefinition};
    use serde_json::Value;
    use std::path::{Path, PathBuf};
    use toon_format::decode_default;

    #[test]
    fn build_toon_encodes_annotations() {
        let config = Config {
            comments: vec!["//".to_string()],
            exclude: vec![],
            labels: vec![
                LabelDefinition {
                    label: "TODO".to_string(),
                    enabled: true,
                    update: true,
                    mark: Some("✅".to_string()),
                    checkbox: true,
                    alias: vec!["TASK".to_string()],
                },
                LabelDefinition {
                    label: "INFO".to_string(),
                    enabled: true,
                    update: true,
                    mark: None,
                    checkbox: false,
                    alias: vec![],
                },
            ],
        };

        let annotations = vec![
            Annotation {
                file: PathBuf::from("src/main.rs"),
                line: 3,
                label: "TASK".to_string(),
                content: "first todo".to_string(),
            },
            Annotation {
                file: PathBuf::from("src/lib.rs"),
                line: 10,
                label: "INFO".to_string(),
                content: "note here".to_string(),
            },
        ];

        let output = build_toon(Path::new("suppa"), &config, &annotations);
        let parsed: Value = decode_default(&output).expect("output should be valid TOON");

        let project = parsed.get("suppa").expect("project key should exist");

        assert_eq!(
            project["✅ TODO"][0]["path"],
            Value::String("src/main.rs".to_string())
        );
        assert_eq!(project["✅ TODO"][0]["check"], Value::Bool(false));
        assert_eq!(project["✅ TODO"][0]["line"], Value::from(3u64));
        assert_eq!(
            project["✅ TODO"][0]["context"],
            Value::String("first todo".to_string())
        );

        assert_eq!(
            project["INFO"][0]["path"],
            Value::String("src/lib.rs".to_string())
        );
        assert_eq!(project["INFO"][0]["line"], Value::from(10u64));
        assert_eq!(
            project["INFO"][0]["context"],
            Value::String("note here".to_string())
        );
    }
}
