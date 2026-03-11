use crate::model::{Annotation, Config};

use anyhow::{Context, Result, bail};
use regex::Regex;
use std::collections::HashSet;
use std::fs::read_to_string;
use std::path::{Path, PathBuf};
use std::process::Command;

pub fn collect_annotations(root: &Path, config: &Config) -> Result<Vec<Annotation>> {
    let mut annotations = Vec::new();

    for label in &config.labels {
        let mut unique = HashSet::new();
        for comment in &config.comments {
            for term in &label.terms {
                let pattern = build_pattern(comment, term)?;
                let matches = run_rg(root, &pattern)?;
                for (file, line_number, line_text) in matches {
                    if let Some(content) = extract_content(&line_text, &pattern) {
                        let content = content.trim().to_string();
                        if content.is_empty() {
                            continue;
                        }

                        let annotation = Annotation {
                            label: label.name.clone(),
                            content,
                            file,
                            line: line_number,
                        };

                        if unique.insert(annotation.clone()) {
                            annotations.push(annotation);
                        }
                    }
                }
            }
        }
    }

    Ok(annotations)
}

fn build_pattern(comment: &str, term: &str) -> Result<Regex> {
    let escaped_comment = regex::escape(comment);
    let escaped_term = regex::escape(term);
    let pattern = format!(
        r"(?:^|\s){}\s*{}\s*:\s*(.*)$",
        escaped_comment, escaped_term
    );
    Regex::new(&pattern).with_context(|| format!("invalid regex pattern: {pattern}"))
}

fn run_rg(root: &Path, pattern: &Regex) -> Result<Vec<(PathBuf, u64, String)>> {
    let output = Command::new("rg")
        .arg("-T")
        .arg("md")
        .arg("--files-with-matches")
        .arg("-e")
        .arg(pattern.as_str())
        .arg(root)
        .output()
        .context("failed to execute rg command. install ripgrep and ensure 'rg' is available")?;

    if !output.status.success() {
        if output.status.code() == Some(1) {
            return Ok(Vec::new());
        }
        let stderr = String::from_utf8_lossy(&output.stderr);
        bail!("ripgrep command failed: {stderr}");
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    let mut matches = Vec::new();

    for file_path in stdout.lines() {
        if file_path.trim().is_empty() {
            continue;
        }

        let path = PathBuf::from(file_path);
        let full_path = if path.is_absolute() {
            path
        } else {
            root.join(path)
        };

        let content = match read_to_string(&full_path) {
            Ok(c) => c,
            Err(_) => continue,
        };

        for (idx, line) in content.lines().enumerate() {
            if pattern.is_match(line) {
                matches.push((full_path.clone(), (idx + 1) as u64, line.to_string()));
            }
        }
    }
    Ok(matches)
}

fn extract_content(line_text: &str, pattern: &Regex) -> Option<String> {
    pattern
        .captures(line_text)
        .and_then(|caps| caps.get(1).map(|m| m.as_str().to_string()))
}
