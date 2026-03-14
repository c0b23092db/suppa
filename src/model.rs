use serde::Deserialize;
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct Config {
    /// コメントのパターン
    pub comments: Vec<String>,
    /// 除外するファイルタイプ
    pub exclude: Vec<String>,
    /// ラベル定義のリスト
    pub labels: Vec<LabelDefinition>,
}

/// TOMLでの全体の設定
#[derive(Debug, Clone)]
pub struct LabelDefinition {
    /// ラベル名
    pub label: String,
    /// 有効化フラグ
    pub enabled: bool,
    /// ラベルに対応するマーク（例: "✅"）
    pub mark: Option<String>,
    /// チェックボックスの有無
    pub checkbox: bool,
    /// エイリアス
    pub alias: Vec<String>,
}

/// TOMLでのラベル定義
#[derive(Debug, Clone, Deserialize, Default)]
pub struct SerdeConfig {
    /// マーク
    #[serde(default)]
    pub mark: Option<String>,
    /// 有効化
    #[serde(default = "default_true")]
    pub enabled: bool,
    /// チェックボックスの有無
    #[serde(default)]
    pub checkbox: bool,
    /// エイリアス
    #[serde(default)]
    pub alias: Vec<String>,
}
fn default_true() -> bool {
    true
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct Annotation {
    /// ファイル名
    pub file: PathBuf,
    /// 行番号
    pub line: u64,
    /// ラベル
    pub label: String,
    /// コンテンツ
    pub content: String,
}

/// Default configuration in TOML format
pub const DEFAULT_CONFIG: &str = r##"comment = ["//", "#", "--"]
exclude = ["md"]

[TODO]
mark = "✅"
checkbox = true

[INFO]
mark = "📒"
alias = ["NOTE"]

[FIX]
mark = "🔥"
alias = ["FIXIT", "FIXME", "BUG", "ISSUE"]

[WARNING]
mark = "⚠️"
alias = ["WARN"]

[XXX]
mark = "？"

[HACK]
enabled = false
mark = "🔨"

[PERF]
mark = "⚡"
alias = ["OPTIMIZE"]
"##;