// src/main.rs
/*
 * Main executable for SmartSmartcontractsFramework
 */

use clap::Parser;
use smartsmartcontractsframework::{Result, run};

#[derive(Parser)]
#[command(version, about = SmartSmartcontractsFramework - A Rust implementation)]
struct Cli {
    /// Enable verbose output
    #[arg(short, long)]
    verbose: bool,
}

fn main() -> Result<()> {
    let args = Cli::parse();
    run(args.verbose)
}
