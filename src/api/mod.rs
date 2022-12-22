use crate::vm::machine::KittenVM;
use anyhow::{Ok, Result};
use clap::Parser;
use std::{path::Path, sync::Arc};

pub struct Mode {
    pub safe_mode: SafeMode,
    pub gc_mode: GCMode,
}

#[derive(Clone, Parser, Copy)]
pub enum SafeMode {
    Safe,
    Normal,
}

#[derive(Clone, Parser, Copy)]
pub enum GCMode {
    SimpleGC,
    UnGC,
}

pub struct API {
    vm: KittenVM,
}

impl API {
    pub fn new(gc_mode: GCMode, safe_mode: SafeMode) -> Self {
        Self {
            vm: KittenVM::new(gc_mode, safe_mode),
        }
    }
    pub async fn file(&self, _file_path: &Path) -> Result<()> {
        Ok(())
    }
}
