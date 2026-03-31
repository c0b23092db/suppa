use crate::core::project_name_from_root;
use anyhow::{Context, Result};
use serde::Serialize;
use std::fs::write;
use std::path::Path;

use crate::core::hashmap_annotations;
use crate::model::{Annotation, Config};

/// JSONファイルを新規作成する
pub fn create_json(
    path: &Path,
    root: &Path,
    config: &Config,
    annotations: &[Annotation],
) -> Result<()> {
    let json = build_json(root, config, annotations);
    write_json(path, &json)?;
    Ok(())
}

/// アノテーション一覧からJSONテキストを生成
pub fn build_json(root: &Path, config: &Config, annotations: &[Annotation]) -> String {
    #[derive(Serialize)]
    struct JsonAnnotation {
        file: String,
        line: u64,
        content: String,
    }

    #[derive(Serialize)]
    struct JsonLabel {
        label: String,
        mark: Option<String>,
        checkbox: bool,
        annotations: Vec<JsonAnnotation>,
    }

    #[derive(Serialize)]
    struct JsonOutput {
        project: String,
        labels: Vec<JsonLabel>,
    }

    let grouped = hashmap_annotations(config, annotations);
    let labels = config
        .labels
        .iter()
        .filter(|label| label.enabled)
        .map(|label| {
            let entries = grouped
                .get(label.label.as_str())
                .map(|items| {
                    items
                        .iter()
                        .map(|item| JsonAnnotation {
                            file: item.file.display().to_string(),
                            line: item.line,
                            content: item.content.clone(),
                        })
                        .collect::<Vec<_>>()
                })
                .unwrap_or_default();

            JsonLabel {
                label: label.label.clone(),
                mark: label.mark.clone(),
                checkbox: label.checkbox,
                annotations: entries,
            }
        })
        .collect::<Vec<_>>();

    let output = JsonOutput {
        project: project_name_from_root(root),
        labels,
    };

    match serde_json::to_string_pretty(&output) {
        Ok(mut text) => {
            text.push('\n');
            text
        }
        Err(_) => "{}\n".to_string(),
    }
}

/// JSONファイルを作成する
fn write_json(path: &Path, json: &str) -> Result<()> {
    write(path, json)
        .with_context(|| format!("Failed to write output file: {}", path.display()))?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::build_json;
    use crate::model::{Annotation, Config, LabelDefinition};
    use std::path::{Path, PathBuf};

    #[test]
    fn build_json_outputs_enabled_labels_with_annotations() {
        let config = Config {
            comments: vec!["//".to_string()],
            exclude: Vec::new(),
            labels: vec![
                LabelDefinition {
                    label: "TODO".to_string(),
                    enabled: true,
                    update: true,
                    mark: Some("!".to_string()),
                    checkbox: true,
                    alias: vec!["TASK".to_string()],
                },
                LabelDefinition {
                    label: "HACK".to_string(),
                    enabled: false,
                    update: true,
                    mark: None,
                    checkbox: false,
                    alias: Vec::new(),
                },
            ],
        };
        let annotations = vec![Annotation {
            file: PathBuf::from("src/main.rs"),
            line: 12,
            label: "TASK".to_string(),
            content: "write tests".to_string(),
        }];

        let output = build_json(Path::new("suppa"), &config, &annotations);
        let value: serde_json::Value = serde_json::from_str(&output).expect("JSON must be valid");

        assert_eq!(value["project"], "suppa");
        assert_eq!(value["labels"].as_array().map(|v| v.len()), Some(1));
        assert_eq!(value["labels"][0]["label"], "TODO");
        assert_eq!(
            value["labels"][0]["annotations"][0]["content"],
            "write tests"
        );
    }
}
