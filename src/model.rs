use serde::Deserialize;
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct Config {
    pub comments: Vec<String>,
    pub labels: Vec<LabelDefinition>,
}

#[derive(Debug, Clone)]
pub struct LabelDefinition {
    pub name: String,
    pub mark: Option<String>,
    pub checkbox: bool,
    pub terms: Vec<String>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct LabelConfig {
    #[serde(default)]
    pub mark: Option<String>,
    #[serde(default)]
    pub checkbox: bool,
    #[serde(default)]
    pub alias: Vec<String>,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct Annotation {
    pub label: String,
    pub content: String,
    pub file: PathBuf,
    pub line: u64,
}

pub const DEFAULT_CONFIG: &str = r##"comment = ["//", "#", "--"]

[TODO]
mark = "✅"
checkbox = true

[FIX]
mark = "🔥"
alias = ["FIXIT", "FIXME", "BUG", "ISSUE"]

[WARNING]
mark = "⚠️"
alias = ["WARN", "XXX"]

[HACK]
mark = "🔨"

[PERF]
mark = "⚡"
alias = ["OPTIMIZE"]

[NOTE]
mark = "📒"
alias = ["INFO", "MEMO"]
"##;