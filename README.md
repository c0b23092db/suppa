# Suppa
```bash
spp.exe
```
**A CLI tool to extract annotation comments from your project and output them to a Markdown file**

Japanese - [README-ja.md](README/README-ja.md)

## Runtime Environment

### Verified
- [x] Windows 11 (64bit)

### Unverified
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


### binary

- [Windows](https://github.com/c0b23092db/suppa/releases/download/v0.2.0/spp.exe)


## Commands

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

| Option | Short | Default | Description |
| :-- | :-- | :-- | :-- |
| `--config` | `-c` | `~/.config/suppa/config.toml` | Path to TOML configuration file |
| `--root` | `-r` | `.` | Project root directory to scan |
| `--output` | `-o` | `annotations.md` | Path to Markdown file |

### Default Behavior

```
spp
```

- If the output file **does not exist**: [Create New (New)](#create-new-new)
- If the output file **exists**: [Update](#update)


### Create New (New)

Extracts annotations and generates a Markdown file.

#### Using default configuration (~/.config/suppa/config.toml)

```bash
spp new
spp create
```


#### Specifying a configuration file

```bash
spp -c suppa.toml
```


#### Specifying scan target and output destination

```bash
spp -r ./src -o todo.md
```


### Update

```bash
spp up
spp update
```

Updates an existing Markdown file.
For labels with `checkbox = true`, **only new items are appended**, preserving the existing `[x]` checked state.

#### Checkbox Handling

When a list item is checked in a section with `checkbox` enabled, it will be moved to `### Archive:~~~`.
Even if the corresponding line is removed from the file, items in this archive list will not be deleted.

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


### Print

```bash
spp print
```

Outputs to the terminal in Markdown format.

### Simple Print (SimplePrint)

```bash
spp simple-print
spp sp
```

Outputs a simple list to the terminal.

```
example\Rust.rs:1 [TODO] Annotation for todo
example\Rust.rs:2 [FIX] Annotation for fix
```


## Configuration File (TOML)

```toml
comment = ["//", "#", "--"]
exclude = ["md"]
[Example]
enabled = true
mark = "✅"
alias = ["ex"]
checkbox = false
```

The order of sections follows **the order defined in the TOML file**.

### Configuration Items

| Key | Description |
| :-- | :-- |
| `comment` | List of comment symbols |
| `exclude` | List of files to exclude |

| Key | Description |
| :-- | :-- |
| `enabled` | Enable this section |
| `mark` | Mark to display in section header |
| `checkbox` | When `true`, outputs in `- [ ]` checkbox format |
| `alias` | List of alias names to be treated as the same label |

### Default Configuration

Configuration file: [config.toml](./example/config.toml)

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


## Example Output

Output using the default configuration.

### Input File

Target file: [Rust.rs](./example/Rust.rs)

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

Output file: [annotations.md](./example/annotations.md)

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

## Contributing

Bug reports, feature suggestions, and pull requests are welcome.

## LICENSE

[MIT License](./LICENSE) / [http://opensource.org/licenses/MIT](http://opensource.org/licenses/MIT)

## Developer

- ikata
