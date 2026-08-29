mod cleaner;
mod model;
mod scanner;
mod ui;

use anyhow::Result;
use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(name = "dustoff", version, about = "Safe Linux cleanup for developers")]
struct Cli {
    #[command(subcommand)]
    command: Option<Command>,
}

#[derive(Debug, Subcommand)]
enum Command {
    /// Analyze the system without deleting anything
    Analyze,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let items = scanner::scan()?;

    match cli.command {
        Some(Command::Analyze) => ui::print_analysis(&items),
        None => ui::run_interactive(items)?,
    }

    Ok(())
}
