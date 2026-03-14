# Suppa（すっぱ）
```bash
spp.exe
```
**プロジェクトのアノテーションコメントをMarkdownファイルで出力するCLI**

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
- [Windows](https://github.com/c0b23092db/suppa/releases/download/v0.2.0/spp.exe)

## コマンド
```bash
>spp
CLI: Extract TODO-like annotations into Markdown

Usage: spp.exe [OPTIONS] [COMMAND]

Commands:
  new           Create a new Markdown file with current annotations
  update        Update the output file with current annotations
  print         Print the current annotations
  simple-print  Simple Print
  help          Print this message or the help of the given subcommand(s)

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

### デフォルト動作
```
spp
```
- 出力先のファイルが**存在しない**場合: [新規作成(New)](#新規作成new)
- 出力先のファイルが**存在する**場合: [更新(Update)](#更新update)

### 新規作成(New)
アノテーションを抽出してMarkdownファイルを生成します。

#### デフォルト設定（~/.config/suppa/config.toml）
```bash
spp new
spp create
```

#### 設定ファイルを指定
```bash
spp -c suppa.toml
```

#### スキャン対象・出力先を指定
```bash
spp -r ./src -o todo.md
```

### 更新(Update)
```bash
spp up
spp update
```
既存のMarkdownファイルを更新します。
`checkbox = true` のラベルは **新規項目の追記のみ** 行い、既存の `[x]` チェック状態を保持します。

#### チェックボックスの扱い
`checkbox`の対象となっている章でリストにチェックが入っている場合、`### Archive:~~~`に移動されます。
ファイルから対象の文章が削除されても、こちらのリストにある要素は削除されません。

```md
## ✅ TODO
- [ ] Annotation for 1 (example\Rust.rs:1)
- [ ] Annotation for 2 (example\Rust.rs:2)
```
↓
```md
## ✅ TODO
- [ ] Annotation for 2 (example\Rust.rs:2)

### Archive:TODO
- [x] Annotation for 1 (example\Rust.rs:1)
```

### 出力(Print)
```bash
spp print
```
ターミナルにMarkdown形式で出力します。

### 単純出力(SimplePrint)
```bash
spp simple-print
spp sp
```
ターミナルに簡易リストで出力します。
```
example\Rust.rs:1 [TODO] Annotation for todo
example\Rust.rs:2 [FIX] Annotation for fix
```

## 設定ファイル（TOML）
```toml
comment = ["//", "#", "--"]
exclude = ["md"]
[Example]
enabled = true
mark = "✅"
alias = ["ex"]
checkbox = false
```
章の並び順は**tomlに設定した順番**になります。

### 設定項目

| キー | 説明 |
| --- | --- |
| `comment` | コメント記号のリスト |
| `exclude` | 除外するファイルのリスト |

| キー | 説明 |
| --- | --- |
| `enabled` | 有効化 |
| `mark` | セクションヘッダーに付けるマーク |
| `checkbox` | `true` にすると `- [ ]` チェックボックス形式で出力 |
| `alias` | 同一ラベルとして扱うエイリアス名のリスト |

### デフォルト設定
設定ファイル: [config.toml](../example/config.toml)
```toml
comment = ["//", "#", "--"]
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
```

## 出力例
デフォルトの設定で出力しています。

### Input File
対象ファイル: [Rust.rs](../example/Rust.rs)
```rs
// TODO: Annotation for todo
// FIX: Annotation for fix
// WARNING: Annotation for warning
// XXX: Annotation for xxx
// NOTE: Annotation for note
// INFO: Annotation for info
// HACK: Annotation for hack
fn main(){
    println!("Hello, world!");
}
```

### Output File
出力ファイル: [annotations.md](../example/annotations.md)
```md
# suppa

## ✅ TODO
- [ ] Annotation for todo (example\Rust.rs:1)

## 📒 INFO
- Annotation for note (example\Rust.rs:5)
- Annotation for info (example\Rust.rs:6)

## 🔥 FIX
- Annotation for fix (example\Rust.rs:2)

## ⚠️ WARNING
- Annotation for warning (example\Rust.rs:3)

## ？ XXX
- Annotation for xxx (example\Rust.rs:4)

```

## TODO

## 貢献
バグ報告、機能提案、プルリクエストを歓迎します。

## LICENSE
[MIT License](../LICENSE) / <http://opensource.org/licenses/MIT>

## 開発者
- ikata
