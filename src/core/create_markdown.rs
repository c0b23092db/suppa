use crate::model::{Annotation, Config};
use anyhow::{Context, Result};
use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::{Path, PathBuf};

pub fn create_markdown(path: &Path, markdown: &str) -> Result<()> {
    fs::write(path, markdown)
        .with_context(|| format!("failed to write output file: {}", path.display()))?;
    Ok(())
}

pub fn update_markdown(
    path: &Path,
    root: &Path,
    config: &Config,
    annotations: &[Annotation],
) -> Result<()> {
    let existing = fs::read_to_string(path).unwrap_or_default();
    let existing_checkbox = collect_existing_checkbox_states(&existing, config);

    let project_name = root
        .file_name()
        .and_then(|s| s.to_str())
        .unwrap_or("Project");

    let mut out = String::new();
    out.push_str(&format!("# {project_name}\n"));

    for label in &config.labels {
        out.push('\n');
        let header = header_for_label(label.mark.as_deref(), &label.name);
        out.push_str(&header);

        if label.checkbox {
            let existing_states = existing_checkbox
                .get(&label.name)
                .cloned()
                .unwrap_or_default();
            let mut seen_keys = HashSet::new();
            for item_line in checkbox_lines_for_label(root, annotations, &label.name) {
                if let Some(key) = checkbox_content_key(&item_line) {
                    if seen_keys.insert(key.clone()) {
                        out.push('\n');
                        if existing_states.get(&key).copied().unwrap_or(false) {
                            out.push_str(&item_line.replacen("- [ ] ", "- [x] ", 1));
                        } else {
                            out.push_str(&item_line);
                        }
                    }
                }
            }
        } else {
            for item in plain_lines_for_label(root, annotations, &label.name) {
                out.push('\n');
                out.push_str(&item);
            }
        }

        out.push('\n');
    }

    create_markdown(path, &out)
}

/// アノテーション一覧から Markdown テキストを新規生成します。
///
/// `update_markdown` と異なり、既存ファイルのチェック状態は参照せず、
/// チェックボックス項目はすべて未チェック（`[ ]`）で出力します。
///
/// ## 引数
/// - `root`: プロジェクトルート（相対パス表示の基準）
/// - `config`: ラベル定義を含む設定
/// - `annotations`: 抽出済みアノテーション一覧
///
/// ## 戻り値
/// 生成された Markdown 文字列を返します。
pub fn build_markdown(root: &Path, config: &Config, annotations: &[Annotation]) -> String {
    let project_name = root
        .file_name()
        .and_then(|s| s.to_str())
        .unwrap_or("Project");

    let mut out = String::new();
    out.push_str(&format!("# {project_name}\n"));

    for label in &config.labels {
        out.push('\n');
        let header = header_for_label(label.mark.as_deref(), &label.name);
        out.push_str(&header);

        if label.checkbox {
            for item in checkbox_lines_for_label(root, annotations, &label.name) {
                out.push('\n');
                out.push_str(&item);
            }
        } else {
            for item in plain_lines_for_label(root, annotations, &label.name) {
                out.push('\n');
                out.push_str(&item);
            }
        }

        out.push('\n');
    }

    out
}

/// ラベル見出し文字列を生成します。
///
/// `mark` が有効な場合は `## {mark} {name}`、
/// 空または未指定の場合は `## {name}` を返します。
fn header_for_label(mark: Option<&str>, name: &str) -> String {
    match mark {
        Some(mark) if !mark.trim().is_empty() => format!("## {} {}", mark, name),
        _ => format!("## {}", name),
    }
}

/// 指定ラベルに一致するアノテーションを抽出し、
/// `file -> line -> content` の順で返します。
fn sorted_annotations_for_label(annotations: &[Annotation], label_name: &str) -> Vec<Annotation> {
    let mut items = annotations
        .iter()
        .filter(|annotation| annotation.label == label_name)
        .cloned()
        .collect::<Vec<_>>();

    items.sort_by(|left, right| {
        left.file
            .cmp(&right.file)
            .then_with(|| left.line.cmp(&right.line))
            .then_with(|| left.content.cmp(&right.content))
    });
    items
}

/// 指定ラベルのアノテーションをチェックボックス形式の行に整形して返します。
///
/// 出力形式: `- [ ] {content} ({relative_path}:{line})`
///
/// `file` は可能であれば `root` からの相対パスで表示されます。
fn checkbox_lines_for_label(
    root: &Path,
    annotations: &[Annotation],
    label_name: &str,
) -> Vec<String> {
    sorted_annotations_for_label(annotations, label_name)
        .into_iter()
        .map(|item| {
            let relative_path = item
                .file
                .strip_prefix(root)
                .map(PathBuf::from)
                .unwrap_or(item.file.clone());
            format!(
                "- [ ] {} ({}:{})",
                item.content,
                relative_path.display(),
                item.line
            )
        })
        .collect()
}

/// 指定ラベルのアノテーションを通常箇条書き形式の行に整形して返します。
///
/// 出力形式: `- {content} ({relative_path}:{line})`
///
/// `file` は可能であれば `root` からの相対パスで表示されます。
fn plain_lines_for_label(root: &Path, annotations: &[Annotation], label_name: &str) -> Vec<String> {
    sorted_annotations_for_label(annotations, label_name)
        .into_iter()
        .map(|item| {
            let relative_path = item
                .file
                .strip_prefix(root)
                .map(PathBuf::from)
                .unwrap_or(item.file.clone());
            format!(
                "- {} ({}:{})",
                item.content,
                relative_path.display(),
                item.line
            )
        })
        .collect()
}

/// 既存 Markdown から、ラベルごとのチェックボックス状態を収集します。
///
/// 見出し行（`## ...`）を `config` のラベルに対応付け、
/// 該当セクション内のチェックボックス行を解析して
/// `content -> checked(bool)` のマップを構築します。
///
/// # 戻り値
/// `HashMap<label_name, HashMap<content_key, is_checked>>`
fn collect_existing_checkbox_states(
    markdown: &str,
    config: &Config,
) -> HashMap<String, HashMap<String, bool>> {
    let header_to_label = config
        .labels
        .iter()
        .map(|label| {
            (
                header_for_label(label.mark.as_deref(), &label.name),
                label.name.clone(),
            )
        })
        .collect::<HashMap<_, _>>();

    let checkbox_labels = config
        .labels
        .iter()
        .filter(|label| label.checkbox)
        .map(|label| label.name.clone())
        .collect::<HashSet<_>>();

    let mut current_label: Option<String> = None;
    let mut result: HashMap<String, HashMap<String, bool>> = HashMap::new();

    for raw_line in markdown.lines() {
        let line = raw_line.trim_end();

        if let Some(label_name) = header_to_label.get(line) {
            current_label = Some(label_name.clone());
            continue;
        }

        if line.starts_with("## ") {
            current_label = None;
            continue;
        }

        let Some(label_name) = &current_label else {
            continue;
        };

        if !checkbox_labels.contains(label_name) {
            continue;
        }

        if let Some(key) = checkbox_content_key(line) {
            result
                .entry(label_name.clone())
                .or_default()
                .insert(key, checkbox_is_checked(line));
        }
    }

    result
}

/// 1行のチェックボックス項目がチェック済みかどうかを判定します
fn checkbox_is_checked(line: &str) -> bool {
    let line = line.trim();
    line.starts_with("- [x] ") || line.starts_with("- [X] ")
}

/// チェックボックス行から重複判定・状態引き継ぎ用のキー（本文）を抽出します
///
/// `- [ ] ` / `- [x] ` / `- [X] ` の接頭辞を取り除き、
/// 末尾の位置情報 ` ({path}:{line})` があれば除去した本文を返します
/// 本文が空の場合は `None` を返します。
fn checkbox_content_key(line: &str) -> Option<String> {
    let line = line.trim();
    let rest = line
        .strip_prefix("- [ ] ")
        .or_else(|| line.strip_prefix("- [x] "))
        .or_else(|| line.strip_prefix("- [X] "))?;
    let content = rest.split_once(" (").map(|(text, _)| text).unwrap_or(rest);
    let content = content.trim();
    if content.is_empty() {
        return None;
    }
    Some(content.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn ok_test_checkbox_content_key() {
        let line = "- [x] Refactor this function (src/main.rs:11)";
        let key = checkbox_content_key(line);
        assert_eq!(key, Some("Refactor this function".into()));
    }
    #[test]
    fn none_test_checkbox_content_key() {
        let line = "- Refactor this function (src/main.rs:11)";
        let key = checkbox_content_key(line);
        assert_eq!(key, None);
    }
}