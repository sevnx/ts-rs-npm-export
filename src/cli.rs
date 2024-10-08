/// Command line arguments for the ts-rs-export CLI
use clap::Parser;
use std::path::PathBuf;

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
/// Generate TypeScript bindings for Rust crates using `ts-rs`
pub struct Args {
    #[clap(long, short)]
    /// Path to the crate to generate the TypeScript bindings for
    pub path_to_crate: PathBuf,
}
