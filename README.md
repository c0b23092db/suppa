# Suppa（すっぱ）

```bash
spp.exe
```

**CLI tool that outputs annotation comments in project files as a Markdown file**

Japanese - [README-ja.md](./README/README-ja.md)

## Environment

### Verified

- [x] Windows 11 (64bit)

### Not verified

- [ ] Linux
- [ ] Mac

## Installation

### Prerequisites

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

### Binary

- [Windows](https://github.com/c0b23092db/suppa/releases/download/v0.1.0/spp.exe)

## Commands

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

| Option    | Short | Default                         | Description                               |
| --------- | ----- | ------------------------------- | ----------------------------------------- |
| `--config`| `-c`  | `~/.config/suppa/config.toml`   | Path to TOML config file                  |
| `--root`  | `-r`  | `.`                             | Project root directory to scan            |
| `--output`| `-o`  | `annotations.md`                | Output Markdown file path                 |

### Create

Extracts annotations and generates a Markdown file.

#### Using default config (`~/.config/suppa/config.toml`)

```bash
spp
spp create
```

#### Specify config file

```bash
spp.exe -c suppa.toml
```

#### Specify output path and scan target

```bash
spp.exe -r ./src -o todo.md
```

### Update

Updates an existing Markdown file.

For labels with `checkbox = true`, **only new items are appended**, and the existing `[x]` checked state is preserved.

```bash
spp.exe up
spp.exe update
```

### Print

Prints annotations in Markdown format to the terminal.

```bash
spp.exe print
```

## Config file (TOML)

```toml
comment = ["//", "#", "--"]
[Example]
mark = "✅"
alias = ["ex"]
checkbox = false
```

### Config keys

| Key       | Description             |
| --------- | ----------------------- |
| `comment` | List of comment markers |

| Key       | Description                                   |
| --------- | --------------------------------------------- |
| `mark`    | Mark to attach to the section header          |
| `checkbox`| If `true`, output in `- [ ]` checkbox format  |
| `alias`   | List of alias names treated as the same label |

### Default config

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

## Output example

### Input file

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

### Output file

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

- [ ] toml: include: Target files
- [ ] toml: exclude: Excluded files
- [ ] toml: enable: Enable/disable annotations
- [ ] toml: sort: Sort order of annotations
- [ ] toml: priority: Priority of annotations
- [ ] Run: Eliminate bottlenecks

## LICENSE

[MIT License](./LICENSE) / <http://opensource.org/licenses/MIT>

## Author

- ikata
