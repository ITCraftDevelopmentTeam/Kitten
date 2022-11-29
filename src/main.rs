mod analysis;
pub mod api;
mod bitcode;
pub mod vm;
use api::{GCMode, SafeMode, API};

use anyhow::Result;
use clap::Parser;
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

    ///File path
    #[arg(short, long)]
    file_path: Option<PathBuf>,
}

#[tokio::main]
async fn main() -> Result<()> {
    let matches = Cli::parse();

    let result_gc_mode = match matches.gc_mode.as_deref() {
        Some("UnGC") => GCMode::UnGC,
        Some("SimpleGC") => GCMode::SimpleGC,
        _ => GCMode::UnGC,
    };
    let result_safe_mode = match matches.safe_mode.as_deref() {
        Some("Normal") => SafeMode::Normal,
        Some("Safe") => SafeMode::Safe,
        _ => SafeMode::Normal,
    };

    let api = API::new(result_gc_mode, result_safe_mode);
    let file_context = match matches.file_path {
        Some(n) => {
            if tokio::fs::read(&n).await.is_ok() {
                tokio::fs::read_to_string(&n).await
            } else {
                tracing::info!("The file cannot be found,path:{:#?}",&n);
                return Err(anyhow::anyhow!(
                    "The file cannot be found,path:{:#?}",&n
                ));
            }
        }
        None => {
            tracing::info!("The file cannot be found,the value you entered is empty");
            return Err(anyhow::anyhow!(
                "The file cannot be found,the value you entered is empty"
            ));
        }
    };

    ///异步守护进程/垃圾回收/VM主线程
    let vm_result = tokio::spawn(async move {todo!()});
    let guardianship_process = tokio::spawn(async move {todo!()});
    let gc_process = tokio::spawn(async move {todo!()});

    Ok(())
}
