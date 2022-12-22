use super::const_pool::ConstPool;
use super::dynamic_memory::DynamicMemory;
use crate::api::{GCMode, Mode, SafeMode};
use crate::bitcode::KEYWORD;
use anyhow::Result;

enum Status {
    Working,
    Free,
    Unknown,
}
impl Default for Status {
    fn default() -> Self {
        Self::Unknown
    }
}

pub struct KittenVM {
    pub const_pool: ConstPool,
    pub dynamic_memory: DynamicMemory,
    status: Status,
    pub mode: Mode,
}

impl KittenVM {
    pub fn new(gc_mode: GCMode, safe_mode: SafeMode) -> Self {
        Self {
            const_pool: ConstPool::default(),
            dynamic_memory: DynamicMemory::default(),
            status: Status::default(),
            mode: Mode { safe_mode, gc_mode },
        }
    }
    /*
    匹配指令
    */
    pub async fn lexer(&mut self, code: String) -> Result<()> {
        if code.is_empty() {
            tracing::info!("code.is_empty()");
        }
        let code_info: Vec<&str> = code.split(' ').collect();
        let in_match = code_info[0];
        let _result: Result<(), anyhow::Error> = match in_match {
            "free" => self.dynamic_memory.free(),
            "add_gc" => self.dynamic_memory.add_gc(code_info[1].parse()?).await,
            "new" => {
                self.dynamic_memory
                    .new_mem(code_info[1].parse().unwrap(), code_info[2].parse().unwrap())
                    .await
            }
            "mov" => {
                self.dynamic_memory
                    .mov(code_info[1].parse().unwrap(), code_info[2].parse()?)
                    .await
            }
            "add" => {
                self.dynamic_memory
                    .add(
                        code_info[1].parse()?,
                        code_info[2].parse()?,
                        code_info[3].parse()?,
                    )
                    .await
            }
            "sud" => {
                self.dynamic_memory
                    .sud(
                        code_info[1].parse()?,
                        code_info[2].parse()?,
                        code_info[3].parse()?,
                    )
                    .await
            }
            "mul" => {
                self.dynamic_memory
                    .mul(
                        code_info[1].parse()?,
                        code_info[2].parse()?,
                        code_info[3].parse()?,
                    )
                    .await
            }
            "div" => {
                self.dynamic_memory
                    .div(
                        code_info[1].parse()?,
                        code_info[2].parse()?,
                        code_info[3].parse()?,
                    )
                    .await
            }
            "sll" => {
                self.dynamic_memory
                    .sll(
                        code_info[1].parse()?,
                        code_info[2].parse()?,
                        code_info[3].parse()?,
                    )
                    .await
            }
            "sra" => {
                self.dynamic_memory
                    .sra(
                        code_info[1].parse()?,
                        code_info[2].parse()?,
                        code_info[3].parse()?,
                    )
                    .await
            }
            "xor" => {
                self.dynamic_memory
                    .xor(
                        code_info[1].parse()?,
                        code_info[2].parse()?,
                        code_info[3].parse()?,
                    )
                    .await
            }
            "or" => {
                self.dynamic_memory
                    .or(
                        code_info[1].parse()?,
                        code_info[2].parse()?,
                        code_info[3].parse()?,
                    )
                    .await
            }
            "and" => {
                self.dynamic_memory
                    .and(
                        code_info[1].parse()?,
                        code_info[2].parse()?,
                        code_info[3].parse()?,
                    )
                    .await
            }
            _ => {
                tracing::error!(
                    "Keyword not found\n All the current keywords are as follows{:#?}",
                    KEYWORD
                );
                return Err(anyhow::anyhow!("Keyword not found"));
            }
        };
        Ok(())
    }
}
