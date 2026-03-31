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
#### `cargo install`
```bash
cargo install suppa
```
#### `cargo binstall`
```bash
cargo binstall suppa
```
#### `cargo install --git`
```bash
cargo install --git https://github.com/c0b23092db/suppa
```

### binary
- [Windows](https://github.com/c0b23092db/suppa/releases/download/v0.3.0/spp.exe)

## コマンド
```bash
> spp
CLI: Extract TODO-like annotations into Markdown

Usage: spp.exe [OPTIONS] [COMMAND]

Commands:
  init          Initialize a default config file in the current directory
  new           Create a new Markdown file with current annotations
  update        Update the output file with current annotations
  print         Print the current annotations
  simple-print  Simple Print
  summary       Print a summary of the annotations
  help          Print this message or the help of the given subcommand(s)

Options:
  -r, --root <ROOT>      Project root directory to scan [default: .]
  -o, --output <OUTPUT>  Output file [default: annotations.<format>]
  -f, --format <FORMAT>  Format of output [default: Markdown]
  -c, --config <CONFIG>  Path to TOML config file
  -h, --help             Print help
  -V, --version          Print version
```

| オプション | 短縮形 | デフォルト | 説明 |
| --- | --- | --- | --- |
| `--root` | `-r` | `.` | スキャン対象のプロジェクトルート |
| `--output` | `-o` | `annotations.md` | Markdownファイルのパス |
| `--format` | `-f` | `markdown` | 出力する形式 |
| `--config` | `-c` | `~/.config/suppa/config.toml` | TOML設定ファイルのパス |

### デフォルト動作
```
spp
```
- 出力先のファイルが**存在しない**場合: [新規作成(New)](#新規作成new)
- 出力先のファイルが**存在する**場合: [更新(Update)](#更新update)

### 新規作成(New)
```bash
spp new
spp create
```
アノテーションを抽出してMarkdownファイルを生成します。

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

### 設定ファイルの初期化(Init)
```bash
spp init
```
プロジェクトルートに設定ファイルを作成します。
設定ファイルは以下の順番で読み込まれます。
1. プロジェクトルートの設定ファイル
2. ホームディレクトリの`.config/suppa/config.toml`

### 出力(Print)
```bash
spp print
```
ターミナルに指定したフォーマットで出力します。

### 単純出力(SimplePrint)
```bash
spp simple-print
spp sp
```
ターミナルに一覧を出力します。
```
example\Rust.rs:1 [TODO] Annotation for todo
example\Rust.rs:2 [FIX] Annotation for fix
```

### 統計(summary)
```bash
spp summary
```
ターミナルにアノテーションコメントの統計を出力します。
```
Annotation Summary:
  ✅ TODO: 1
  📒 INFO: 2
  🔥 FIX: 1
  ⚠️ WARNING: 1
  ？ XXX: 1
```

## 設定ファイル（TOML）
```toml
comment = ["//", "#", "--"]
exclude = ["md"]
[Example]
enabled = true
update = true
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
| `update` | 更新の有無 |
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

#### Markdown
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

#### Toon
出力ファイル: [annotations.toon](../example/annotations.toon)
```toon
suppa:
  "✅ TODO"[1]{check,path,line,context}:
    false,"example\\Rust.rs",1,Annotation for todo
  "📒 INFO"[2]{path,line,context}:
    "example\\Rust.rs",5,Annotation for note
    "example\\Rust.rs",6,Annotation for info
  "🔥 FIX"[1]{path,line,context}:
    "example\\Rust.rs",2,Annotation for fix
  "⚠️ WARNING"[1]{path,line,context}:
    "example\\Rust.rs",3,Annotation for warning
  "？ XXX"[1]{path,line,context}:
    "example\\Rust.rs",4,Annotation for xxx
  "⚡ PERF"[0]:
```

#### Json
> [!WARNING]
> この形式に対する基本的なメンテナンスは行われません。

出力ファイル: [annotations.json](../example/annotations.json)

```json
{
  "project": "suppa",
  "labels": [
    {
      "label": "TODO",
      "mark": "✅",
      "checkbox": true,
      "annotations": [
        {
          "file": "example\\Rust.rs",
          "line": 1,
          "content": "Annotation for todo"
        }
      ]
    },
    {
      "label": "INFO",
      "mark": "📒",
      "checkbox": false,
      "annotations": [
        {
          "file": "example\\Rust.rs",
          "line": 5,
          "content": "Annotation for note"
        },
        {
          "file": "example\\Rust.rs",
          "line": 6,
          "content": "Annotation for info"
        }
      ]
    },
    {
      "label": "FIX",
      "mark": "🔥",
      "checkbox": false,
      "annotations": [
        {
          "file": "example\\Rust.rs",
          "line": 2,
          "content": "Annotation for fix"
        }
      ]
    },
    {
      "label": "WARNING",
      "mark": "⚠️",
      "checkbox": false,
      "annotations": [
        {
          "file": "example\\Rust.rs",
          "line": 3,
          "content": "Annotation for warning"
        }
      ]
    },
    {
      "label": "XXX",
      "mark": "？",
      "checkbox": false,
      "annotations": [
        {
          "file": "example\\Rust.rs",
          "line": 4,
          "content": "Annotation for xxx"
        }
      ]
    },
    {
      "label": "PERF",
      "mark": "⚡",
      "checkbox": false,
      "annotations": []
    }
  ]
}
```

## TODO

## 貢献
現状は頻繁な更新が行われるためお待ちください。
~~バグ報告、機能提案、プルリクエストを歓迎します。~~

## LICENSE
[MIT License](../LICENSE) / <http://opensource.org/licenses/MIT>

## 開発者
- ikata
