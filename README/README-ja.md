# Suppa（すっぱ）
```bash
spp.exe
```
**プロジェクトファイルのアノテーションコメントをMarkdownファイルで出力するCLI**

English - [README.md](../README.md)

## 実行環境
### 検証済
- [x] Windows 11(64bit)
### 未検証
- [ ] Linux
- [ ] Mac

## インストール

### 前提条件
- [ripgrep](https://github.com/BurntSushi/ripgrep)

### cargo
`cargo install`
```bash
cargo install suppa
```
`cargo binstall`
```bash
cargo binstall suppa
```
`cargo install --git`
```bash
cargo install --git https://github.com/c0b23092db/suppa
```

### binary
- [Windows](https://github.com/c0b23092db/suppa/releases/download/v0.1.0/spp.exe)

## コマンド
```bash
>spp
CLI: Extract TODO-like annotations into Markdown

Usage: spp.exe [OPTIONS] [COMMAND]

Commands:
  create  Create a new Markdown file with current annotations (default)
  update  Update the output file with current annotations
  print   Print the current annotations
  help    Print this message or the help of the given subcommand(s)

Options:
  -r, --root <ROOT>      Project root directory to scan [default: .]
  -o, --output <OUTPUT>  Output Markdown file [default: annotations.md]
  -c, --config <CONFIG>  Path to TOML config file
  -h, --help             Print help
  -V, --version          Print version
```

| オプション | 短縮形 | デフォルト | 説明 |
| --- | --- | --- | --- |
| `--config` | `-c` | `~/.config/suppa/config.toml` | TOML設定ファイルのパス |
| `--root` | `-r` | `.` | スキャン対象のプロジェクトルート |
| `--output` | `-o` | `annotations.md` | Markdownファイルのパス |

### 新規作成(Create)
アノテーションを抽出してMarkdownファイルを生成します。
#### デフォルト設定（~/.config/suppa/config.toml）
```bash
spp
spp create
```
#### 設定ファイルを指定
```bash
spp.exe -c suppa.toml
```
#### 出力先・スキャン対象を指定
```bash
spp.exe -r ./src -o todo.md
```

### 更新(Update)
既存のMarkdownファイルを更新します。
`checkbox = true` のラベルは **新規項目の追記のみ** 行い、既存の `[x]` チェック状態を保持します。
```bash
spp.exe up
spp.exe update
```

### 出力(Print)
ターミナルにMarkdown形式で出力します。
```bash
spp.exe print
```

## 設定ファイル（TOML）
```toml
comment = ["//", "#", "--"]
[Example]
mark = "✅"
alias = ["ex"]
checkbox = false
```

### 設定項目

| キー | 説明 |
| --- | --- |
| `comment` | コメント記号のリスト |

| キー | 説明 |
| --- | --- |
| `mark` | セクションヘッダーに付けるマーク |
| `checkbox` | `true` にすると `- [ ]` チェックボックス形式で出力 |
| `alias` | 同一ラベルとして扱うエイリアス名のリスト |

### デフォルト設定
```toml
comment = ["//", "#", "--"]

[TODO]
mark = "✅"
checkbox = true

[FIX]
mark = "🔥"
alias = ["FIXIT", "FIXME", "BUG", "ISSUE"]

[WARNING]
mark = "⚠️"
alias = ["WARN","XXX"]

[HACK]
mark = "🔨"

[PERF]
mark = "⚡"
alias = ["OPTIMIZE"]

[NOTE]
mark = "📒"
alias = ["INFO","MEMO"]
```

## 出力例

### Input File
```rs
// TODO: Annotation for todo
// FIX: Annotation for fix
// WARNING: Annotation for warning
// HACK: Annotation for hack
// PERF: Annotation for performance
// NOTE: Annotation for note
fn main(){
    println!("Hello, world!");
}
```

### Output File
```md
# suppa

## 🔥 FIX
- Annotation for fix (example\Rust.rs:2)

## 🔨 HACK
- Annotation for hack (example\Rust.rs:4)

## 📒 NOTE
- Annotation for note (example\Rust.rs:6)

## ⚡ PERF
- Annotation for performance (example\Rust.rs:5)

## ✅ TODO
- [ ] Annotation for todo (example\Rust.rs:1)

## ⚠️ WARNING
- Annotation for warning (example\Rust.rs:3)

```

## TODO
- [ ] toml: include: 対象ファイル
- [ ] toml: exclude: 除外ファイル
- [ ] toml: enable: アノテーションの有効化
- [ ] toml: sort: アノテーションの並び順
- [ ] toml: priority: アノテーションの優先度
- [ ] Run: ボトルネックの解消

## LICENSE
[MIT License](../LICENSE) / <http://opensource.org/licenses/MIT>

## 開発者
- ikata
