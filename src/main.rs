mod core;
mod model;
mod tests;
use core::{
    build_json, build_markdown, collect_annotations, create_json, create_markdown, load_config,
    print_summary, resolve_config_path, simple_print_annotations, update_markdown,
    run_init,
};

use anyhow::{Result, bail};
use clap::{Parser, Subcommand};
use model::OutputFormat;
use std::path::PathBuf;

/// Extract TODO-like annotations into Markdown
#[derive(Parser, Debug)]
#[command(version, about, arg_required_else_help = false)]
struct Args {
    /// Project root directory to scan
    #[arg(short, long, default_value = ".", global = true)]
    root: PathBuf,
    /// Output file
    #[arg(short, long, global = true)]
    output: Option<PathBuf>,
    /// Format of output
    #[arg(short, long, default_value = "Markdown", global = true)]
    format: Option<String>,
    /// Path to TOML config file
    #[arg(short, long, global = true)]
    config: Option<PathBuf>,
    /// Subcommands for additional functionalities
    #[command(subcommand)]
    command: Option<Command>,
}
#[derive(Subcommand, Debug)]
enum Command {
    /// Initialize a default config file in the current directory
    Init,
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
    /// Print a summary of the annotations
    Summary,
}

fn main() -> Result<()> {
    let args = Args::parse();
    if !args.root.exists() {
        bail!("root path not found: {}", args.root.display());
    }
    let config_path = resolve_config_path(args.config)?;
    let config = load_config(&config_path)?;
    let format = OutputFormat::parse(args.format.as_deref())?;
    let output_path = args.output.unwrap_or_else(|| match format {
        OutputFormat::Markdown => PathBuf::from("annotations.md"),
        OutputFormat::Json => PathBuf::from("annotations.json"),
    });
    let annotations = collect_annotations(&args.root, &config)?;
    match args.command {
        Some(Command::Print) => match format {
            OutputFormat::Markdown => println!("{}", build_markdown(&args.root, &config, &annotations)),
            OutputFormat::Json => println!("{}", build_json(&args.root, &config, &annotations)),
        },
        Some(Command::SimplePrint) => simple_print_annotations(&args.root, &config)?,
        Some(Command::Summary) => print_summary(&config, &annotations)?,
        Some(Command::Init) => run_init()?,
        Some(Command::New) => match format {
            OutputFormat::Markdown => create_markdown(&output_path, &args.root, &config, &annotations)?,
            OutputFormat::Json => create_json(&output_path, &args.root, &config, &annotations)?,
        },
        Some(Command::Update) => match format {
            OutputFormat::Markdown => update_markdown(&output_path, &args.root, &config, &annotations)?,
            OutputFormat::Json => println!("Not implemented: update command is only available for Markdown format"),
        },
        None => match format {
            OutputFormat::Markdown => {
                if output_path.exists() {
                    update_markdown(&output_path, &args.root, &config, &annotations)?;
                } else {
                    create_markdown(&output_path, &args.root, &config, &annotations)?;
                }
            }
            OutputFormat::Json => create_json(&output_path, &args.root, &config, &annotations)?,
        },
    }
    Ok(())
}
