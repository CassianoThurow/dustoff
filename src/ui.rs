use anyhow::Result;
use dialoguer::{theme::ColorfulTheme, Confirm, MultiSelect};
use humansize::{format_size, BINARY};

use crate::{cleaner, model::CleanupItem};

pub fn print_analysis(items: &[CleanupItem]) {
    println!("Dustoff analysis\n");

    let mut known_total = 0_u64;
    for item in items {
        let size = item
            .estimated_bytes
            .map(|bytes| {
                known_total += bytes;
                format_size(bytes, BINARY)
            })
            .unwrap_or_else(|| "calculated during cleanup".to_owned());
        let state = if item.available { "available" } else { "not installed / empty" };
        println!("- {:<28} {:>12}  [{}; {}]", item.label, size, item.risk.marker(), state);
    }

    println!("\nKnown reclaimable size: {}", format_size(known_total, BINARY));
    println!("No files were deleted.");
}

pub fn run_interactive(items: Vec<CleanupItem>) -> Result<()> {
    println!("Dustoff — safe Linux cleanup\n");
    let available: Vec<_> = items.into_iter().filter(|item| item.available).collect();

    if available.is_empty() {
        println!("Nothing to clean.");
        return Ok(());
    }

    let labels: Vec<String> = available
        .iter()
        .map(|item| {
            let size = item
                .estimated_bytes
                .map(|bytes| format_size(bytes, BINARY))
                .unwrap_or_else(|| "size determined by tool".to_owned());
            format!("{:<26} {:>12}  [{}] — {}", item.label, size, item.risk.marker(), item.description)
        })
        .collect();

    let selections = MultiSelect::with_theme(&ColorfulTheme::default())
        .with_prompt("Select what to clean (nothing is selected by default)")
        .items(&labels)
        .interact()?;

    if selections.is_empty() {
        println!("No cleanup actions selected.");
        return Ok(());
    }

    println!("\nReview:");
    for index in &selections {
        let item = &available[*index];
        println!("- {} [{}]", item.label, item.risk.marker());
    }

    let confirmed = Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("Run these cleanup actions?")
        .default(false)
        .interact()?;

    if !confirmed {
        println!("Cleanup cancelled.");
        return Ok(());
    }

    let mut failures = 0;
    for index in selections {
        let item = &available[index];
        print!("Cleaning {}... ", item.label);
        match cleaner::clean(item) {
            Ok(()) => println!("done"),
            Err(error) => {
                failures += 1;
                println!("failed: {error:#}");
            }
        }
    }

    if failures == 0 {
        println!("\nCleanup completed successfully.");
    } else {
        println!("\nCleanup completed with {failures} failure(s).");
    }

    Ok(())
}
