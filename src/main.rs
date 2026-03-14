mod core;
mod model;
mod tests;
use core::{
    build_markdown, collect_annotations, create_markdown, load_config, resolve_config_path,
    simple_print_annotations, update_markdown,
};

use anyhow::{Result, bail};
use clap::{Parser, Subcommand};
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
    /// Create a new Markdown file with current annotations
    #[command(alias = "create")]
    New,
    /// Update the output file with current annotations
    #[command(alias = "up")]
    Update,
    /// Print the current annotations
    Print,
    /// Simple Print
    #[command(alias = "sp")]
    SimplePrint,
}

fn main() -> Result<()> {
    let args = Args::parse();
    if !args.root.exists() {
        bail!("root path not found: {}", args.root.display());
    }
    let config_path = resolve_config_path(args.config)?;
    let config = load_config(&config_path)?;
    let annotations = collect_annotations(&args.root, &config)?;
    match args.command {
        Some(Command::Print) => {
            let markdown = build_markdown(&args.root, &config, &annotations);
            println!("{}", markdown);
        }
        Some(Command::SimplePrint) => simple_print_annotations(&args.root, &config)?,
        Some(Command::New) => create_markdown(&args.output, &args.root, &config, &annotations)?,
        Some(Command::Update) => update_markdown(&args.output, &args.root, &config, &annotations)?,
        None => {
            if args.output.exists() {
                update_markdown(&args.output, &args.root, &config, &annotations)?;
            } else {
                create_markdown(&args.output, &args.root, &config, &annotations)?;
            }
        }
    }
    Ok(())
}
