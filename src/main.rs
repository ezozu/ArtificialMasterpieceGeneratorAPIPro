// src/main.rs
/*
 * Main executable for ArtificialMasterpieceGeneratorAPIPro
 */

use clap::Parser;
use artificialmasterpiecegeneratorapipro::{Result, run};

#[derive(Parser)]
#[command(version, about = "ArtificialMasterpieceGeneratorAPIPro - A Rust implementation")]
struct Cli {
    /// Enable verbose output
    #[arg(short, long)]
    verbose: bool,
}

fn main() -> Result<()> {
    let args = Cli::parse();
    run(args.verbose)
}
