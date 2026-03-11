mod core;
mod model;
use core::{
    build_markdown, collect_annotations, create_markdown, load_config, resolve_config_path,
    update_markdown,
};

use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use std::fs;
use std::path::PathBuf;

/// Extract TODO-like annotations into Markdown
#[derive(Parser, Debug)]
#[command(version, about, arg_required_else_help = false)]
struct Args {
    /// Project root directory to scan
    #[arg(short, long, default_value = ".", global = true)]
    root: PathBuf,
    /// Output Markdown file
    #[arg(short, long, default_value = "annotations.md", global = true)]
    output: PathBuf,
    /// Path to TOML config file
    #[arg(short, long, global = true)]
    config: Option<PathBuf>,
    /// Subcommands for additional functionalities
    #[command(subcommand)]
    command: Option<Command>,
}
#[derive(Subcommand, Debug)]
enum Command {
    /// Create a new Markdown file with current annotations (default)
    Create,
    /// Update the output file with current annotations
    #[command(alias = "up")]
    Update,
    /// Print the current annotations
    Print,
}

fn main() -> Result<()> {
    let args = Args::parse();
    let root = fs::canonicalize(&args.root)
        .with_context(|| format!("Failed to resolve root path: {}", args.root.display()))?;
    let config_path = resolve_config_path(args.config)?;
    let config = load_config(&config_path)?;
    let annotations = collect_annotations(&root, &config)?;
    match args.command {
        Some(Command::Print) => {
            let markdown = build_markdown(&root, &config, &annotations);
            println!("{}", markdown);
        }
        Some(Command::Update) => {
            update_markdown(&args.output, &root, &config, &annotations)?;
        }
        Some(Command::Create) | None => {
            let markdown = build_markdown(&root, &config, &annotations);
            create_markdown(&args.output, &markdown)?;
        }
    }
    Ok(())
}
