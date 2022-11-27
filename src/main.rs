mod analysis;
pub mod api;
mod bitcode;
mod test;
pub mod vm;

use api::{GCMode, SafeMode, API};

use anyhow::Result;
use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(author, version, about,long_about = None)]
struct Cli {
    ///GC mode:UnGC or SimpleGC
    #[arg(short, long)]
    gc_mode: Option<String>,

    ///Safe Mode:Normal or Safe
    #[arg(short, long)]
    safe_mode: Option<String>,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    ///Run file name, usually with ktn as suffix
    File {
        ///File path
        #[arg(short, long)]
        file_path: Option<PathBuf>,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    let matches = Cli::parse();

    let result_gc_mode = match matches.gc_mode.as_deref(){
        Some("UnGC") => GCMode::UnGC,
        Some("SimpleGC") => GCMode::SimpleGC,
        _ => GCMode::UnGC,
    };
    let result_safe_mode = match matches.safe_mode.as_deref(){
        Some("Normal") => SafeMode::Normal,
        Some("Safe") => SafeMode::Safe,
        _ => SafeMode::Normal,
    };

    let api = API::new(result_gc_mode,result_safe_mode);
    Ok(())
}
