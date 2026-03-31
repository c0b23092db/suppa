use crate::core::{hashmap_annotations, project_name_from_root};
use crate::model::{Annotation, Config};
use anyhow::{Context, Result};
use std::collections::{HashMap, HashSet};
use std::fs::{read_to_string, write};
use std::path::Path;

#[derive(Debug, Default)]
struct ExistingCheckboxData {
    states: HashMap<String, HashMap<String, bool>>,
    archived: HashMap<String, Vec<(String, String)>>,
}

/// Markdownファイルを新規作成する
pub fn create_markdown(
    path: &Path,
    root: &Path,
    config: &Config,
    annotations: &[Annotation],
) -> Result<()> {
    let markdown = build_markdown(root, config, annotations);
    write_markdown(path, &markdown)?;
    Ok(())
}

/// 既存Markdownを更新する
pub fn update_markdown(
    path: &Path,
    root: &Path,
    config: &Config,
    annotations: &[Annotation],
) -> Result<()> {
    let existing = read_to_string(path).unwrap_or_default();
    let markdown = build_updated_markdown(&existing, root, config, annotations);
    write_markdown(path, &markdown)?;
    Ok(())
}

/// アノテーション一覧からMarkdownテキストを生成
pub fn build_markdown(root: &Path, config: &Config, annotations: &[Annotation]) -> String {
    let mut output = String::new();
    let project_name = project_name_from_root(root);
    output.push_str(&format!("# {}\n", project_name));
    let grouped = hashmap_annotations(config, annotations);
    for label in &config.labels {
        if !label.enabled {
            continue;
        }
        if !label.update {
            output.push('\n');
            output.push_str(&header_for_label(label.mark.as_deref(), &label.label));
            output.push('\n');
            continue;
        }
        let Some(entries) = grouped.get(label.label.as_str()) else {
            continue;
        };
        output.push('\n');
        output.push_str(&header_for_label(label.mark.as_deref(), &label.label));
        output.push('\n');
        if label.checkbox {
            output.push_str(&checkbox_lines_for_label(root, entries, false));
        } else {
            output.push_str(&plain_lines_for_label(root, entries));
        }
    }
    output
}

fn build_updated_markdown(
    existing: &str,
    root: &Path,
    config: &Config,
    annotations: &[Annotation],
) -> String {
    let existing_checkbox = collect_existing_checkbox_data(existing, config);
    let existing_sections = collect_existing_section_bodies(existing, config);

    let mut output = String::new();
    let project_name = project_name_from_root(root);
    output.push_str(&format!("# {}\n", project_name));
    let grouped = hashmap_annotations(config, annotations);
    for label in &config.labels {
        if !label.enabled {
            continue;
        }
        if !label.update {
            output.push('\n');
            output.push_str(&header_for_label(label.mark.as_deref(), &label.label));
            output.push('\n');
            if let Some(existing_body) = existing_sections.get(&label.label)
                && !existing_body.is_empty()
            {
                output.push_str(existing_body);
                if !existing_body.ends_with('\n') {
                    output.push('\n');
                }
            }
            continue;
        }
        if label.checkbox {
            let entries = grouped.get(label.label.as_str());
            let (active_lines, archive_lines) =
                checkbox_sections_for_label(entries, &label.label, &existing_checkbox);
            if active_lines.is_empty() && archive_lines.is_empty() {
                continue;
            }
            output.push('\n');
            output.push_str(&header_for_label(label.mark.as_deref(), &label.label));
            output.push('\n');
            output.push_str(&active_lines);
            if !archive_lines.is_empty() {
                output.push('\n');
                output.push_str(&archive_header_for_label(&label.label));
                output.push('\n');
                output.push_str(&archive_lines);
            }
        } else {
            let Some(entries) = grouped.get(label.label.as_str()) else {
                continue;
            };
            output.push('\n');
            output.push_str(&header_for_label(label.mark.as_deref(), &label.label));
            output.push('\n');
            output.push_str(&plain_lines_for_label(root, entries));
        }
    }
    output
}

/// Markdownファイルを作成する
fn write_markdown(path: &Path, markdown: &str) -> Result<()> {
    write(path, markdown)
        .with_context(|| format!("Failed to write output file: {}", path.display()))?;
    Ok(())
}

/// ラベル見出し文字列を生成する
///
/// - `mark` が有効な場合: `## {mark} {name}`
/// - 空または未指定の場合: `## {name}`
fn header_for_label(mark: Option<&str>, name: &str) -> String {
    match mark {
        Some(mark) if !mark.trim().is_empty() => format!("## {} {}", mark, name),
        _ => format!("## {}", name),
    }
}

fn archive_header_for_label(name: &str) -> String {
    format!("### Archive:{}", name)
}

/// 指定ラベルのアノテーションを通常箇条書き形式の行に整形して返します。
///
/// 出力形式: `- {content} ({relative_path}:{line})`
fn plain_lines_for_label(_root: &Path, annotation: &[&Annotation]) -> String {
    annotation
        .iter()
        .map(|item| {
            format!(
                "- {} ({}:{})\n",
                item.content,
                item.file.display(),
                item.line
            )
        })
        .collect()
}

/// 指定ラベルのアノテーションをチェックボックス形式の行に整形して返します。
///
/// 出力形式: `- [ ] {content} ({relative_path}:{line})`
fn checkbox_lines_for_label(_root: &Path, annotation: &[&Annotation], check: bool) -> String {
    annotation
        .iter()
        .map(|item| format_checkbox_line(item, check))
        .collect()
}

/// チェックボックス行のフォーマットを生成します。
/// `check` が `true` の場合はチェック済み、`false` の場合は未チェックの行を生成します。
fn format_checkbox_line(item: &Annotation, check: bool) -> String {
    format!(
        "- [{}] {} ({}:{})\n",
        if check { "x" } else { " " },
        item.content,
        item.file.display(),
        item.line
    )
}

/// チェックボックス形式のラベルについて、既存Markdownから状態を引き継いで更新後のセクション内容を生成します。
fn checkbox_sections_for_label(
    entries: Option<&Vec<&Annotation>>,
    label_name: &str,
    existing: &ExistingCheckboxData,
) -> (String, String) {
    let existing_states = existing.states.get(label_name);
    let archived_items = existing.archived.get(label_name);
    let mut active_lines = String::new();
    let mut archive_lines = String::new();
    let mut seen_keys = HashSet::new();
    let mut archived_keys = HashSet::new();

    if let Some(entries) = entries {
        for item in entries.iter() {
            let key = item.content.trim().to_string();
            if key.is_empty() || !seen_keys.insert(key.clone()) {
                continue;
            }

            let checked = existing_states
                .and_then(|states| states.get(&key))
                .copied()
                .unwrap_or(false);
            if checked {
                archived_keys.insert(key);
                archive_lines.push_str(&format_checkbox_line(item, true));
            } else {
                active_lines.push_str(&format_checkbox_line(item, false));
            }
        }
    }

    if let Some(archived_items) = archived_items {
        for (key, line) in archived_items {
            if seen_keys.contains(key) || !archived_keys.insert(key.clone()) {
                continue;
            }
            archive_lines.push_str(line);
            archive_lines.push('\n');
        }
    }

    (active_lines, archive_lines)
}

/// 既存 Markdown から、ラベルごとのチェックボックス状態を収集します。
///
/// 見出し行（`## ...`）を `config` のラベルに対応付け、
/// 該当セクション内のチェックボックス行を解析して
/// `content -> checked(bool)` のマップを構築します。
///
/// # 戻り値
/// `HashMap<label_name, HashMap<content_key, is_checked>>`
fn collect_existing_checkbox_data(markdown: &str, config: &Config) -> ExistingCheckboxData {
    // <`## {mark} {label}` or `## {label}`, ラベル名>
    let header_to_label = config
        .labels
        .iter()
        .map(|label| {
            (
                header_for_label(label.mark.as_deref(), &label.label),
                label.label.clone(),
            )
        })
        .collect::<HashMap<_, _>>();
    // チェックボックス対象のラベル名のハッシュマップ
    let checkbox_labels = config
        .labels
        .iter()
        .filter(|label| label.checkbox)
        .map(|label| label.label.clone())
        .collect::<HashSet<_>>();
    let archive_to_label = config
        .labels
        .iter()
        .filter(|label| label.checkbox)
        .map(|label| (archive_header_for_label(&label.label), label.label.clone()))
        .collect::<HashMap<_, _>>();
    let mut current_label: Option<String> = None;
    let mut in_archive = false;
    let mut result = ExistingCheckboxData::default();

    for raw_line in markdown.lines() {
        let line = raw_line.trim_end();
        if let Some(label_name) = header_to_label.get(line) {
            current_label = Some(label_name.clone());
            in_archive = false;
            continue;
        }
        if let Some(label_name) = archive_to_label.get(line) {
            current_label = Some(label_name.clone());
            in_archive = true;
            continue;
        }
        if line.starts_with("## ") {
            current_label = None;
            in_archive = false;
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
                .states
                .entry(label_name.clone())
                .or_default()
                .insert(key.clone(), in_archive || checkbox_is_checked(line));
            if in_archive || checkbox_is_checked(line) {
                result
                    .archived
                    .entry(label_name.clone())
                    .or_default()
                    .push((key, line.to_string()));
            }
        }
    }
    result
}

fn collect_existing_section_bodies(markdown: &str, config: &Config) -> HashMap<String, String> {
    let header_to_label = config
        .labels
        .iter()
        .map(|label| {
            (
                header_for_label(label.mark.as_deref(), &label.label),
                label.label.clone(),
            )
        })
        .collect::<HashMap<_, _>>();

    let mut current_label: Option<String> = None;
    let mut sections = HashMap::new();

    for raw_line in markdown.lines() {
        let trimmed = raw_line.trim_end();
        if let Some(label_name) = header_to_label.get(trimmed) {
            current_label = Some(label_name.clone());
            sections
                .entry(label_name.clone())
                .or_insert_with(String::new);
            continue;
        }
        if trimmed.starts_with("## ") {
            current_label = None;
            continue;
        }
        let Some(label_name) = &current_label else {
            continue;
        };

        if let Some(body) = sections.get_mut(label_name) {
            body.push_str(raw_line);
            body.push('\n');
        }
    }

    // Remove trailing newline from each section body to prevent accumulation
    for body in sections.values_mut() {
        if body.ends_with('\n') {
            body.pop();
        }
    }

    sections
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
