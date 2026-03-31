use crate::model::{Annotation, Config, LabelDefinition};

use anyhow::{Context, Result};
use colored::{Color, Colorize};
use regex::Regex;
use std::path::{Path, PathBuf};
use std::process::Command;

/// Collect annotations from source files based on the provided configuration
pub fn collect_annotations(root: &Path, config: &Config) -> Result<Vec<Annotation>> {
    let mut annotations = Vec::new();
    let matches = run_matches_paser(root, config)?;
    for (file, line_number, label, line_text) in matches {
        if !line_text.is_empty() {
            annotations.push(Annotation {
                file: file.clone(),
                line: line_number,
                label,
                content: line_text,
            });
        }
    }
    Ok(annotations)
}

/// Simple print annotations in a human-readable format
pub fn simple_print_annotations(root: &Path, config: &Config) -> Result<()> {
    let matches = run_matches_paser(root, config)?;
    for (file, line_number, label, line_text) in matches {
        let path = format!("{}", file.display()).bright_black();
        let line = line_number.to_string().cyan();
        println!(
            "{}:{} [{}] {}",
            path,
            line,
            color_label(&label),
            line_text
        );
    }
    Ok(())
}

/// ripgrepを実行して、マッチした行の[ファイルパス,行番号,ラベル,行テキスト]を返す
fn run_matches_paser(root: &Path, config: &Config) -> Result<Vec<(PathBuf, u64, String, String)>> {
    let pattern = rg_build_pattern(&config.comments, &config.labels)
        .context("Failed to Build Regex Pattern")?;
    let mut command = Command::new("rg");
    if !config.exclude.is_empty() {
        for ext in &config.exclude {
            command.arg("-T").arg(ext);
        }
    }
    let output = command
        .arg("--line-number")
        .arg("--no-heading")
        .arg("--with-filename")
        .arg("--regexp")
        .arg(pattern.as_str())
        .arg(root)
        .output()
        .context("Failed to execute rg command. install ripgrep and ensure 'rg' is available")?;
    let stdout = String::from_utf8_lossy(&output.stdout);
    let mut matches = Vec::new();
    for line in stdout.lines() {
        if let Some(match_result) = text_pattern(line, &config.comments) {
            matches.push(match_result);
        }
    }
    Ok(matches)
}

/// ripgrepの正規表現パターンを構築する
fn rg_build_pattern(comments: &[String], labels: &[LabelDefinition]) -> Result<Regex> {
    let string_comments = comments.join("|");
    let string_terms = labels
        .iter()
        .filter(|label| label.enabled)
        .flat_map(|label| label.alias.iter())
        .cloned()
        .collect::<Vec<_>>()
        .join("|");
    let pattern = format!(r"({})\s*({})\s*:\s*(.+)", string_comments, string_terms);
    Regex::new(&pattern).with_context(|| format!("invalid regex pattern: {pattern}"))
}

/// ripgrepの出力行を解析して[ファイルパス,行番号,ラベル,行テキスト]を抽出する
fn text_pattern(line: &str, comments: &[String]) -> Option<(PathBuf, u64, String, String)> {
    let line = line
        .strip_prefix(".\\")
        .or_else(|| line.strip_prefix("./"))
        .unwrap_or(line);
    let mut parts = line.splitn(4, ':');
    let file = PathBuf::from(parts.next().unwrap_or("").to_string());
    let line_number = parts.next().unwrap_or("0").parse::<u64>().unwrap_or(0);
    let mut label = parts.next().unwrap_or("").trim().to_string();
    for comment in comments {
        label = label
            .strip_prefix(comment)
            .unwrap_or(&label)
            .trim()
            .to_string();
    }
    let line_text = parts.next().unwrap_or("").trim().to_string();
    if !line_text.is_empty() {
        Some((file, line_number, label, line_text))
    } else {
        None
    }
}

fn color_label(label: &str) -> colored::ColoredString {
    label
        .color(deterministic_color(label))
        .bold()
}

fn deterministic_color(label: &str) -> Color {
    const PALETTE: [Color; 6] = [
        Color::Yellow,
        Color::Cyan,
        Color::Magenta,
        Color::Green,
        Color::Blue,
        Color::Red,
    ];
    let hash = label.bytes().fold(0usize, |acc, b| acc.wrapping_add(b as usize));
    PALETTE[hash % PALETTE.len()]
}
