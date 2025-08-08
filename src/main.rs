mod cli;
mod request;
mod runner;
mod report;
mod utils;

use std::process;
use crate::cli::Args;
use crate::runner::run_benchmark;
use clap::Parser;
use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();
    if let Err(e) = run_benchmark(args).await {
        eprintln!("Benchmark failed: {:?}", e);
        process::exit(1);
    }
    Ok(())
}