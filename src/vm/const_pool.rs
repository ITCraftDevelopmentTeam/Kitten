use anyhow::{Ok, Result};
use std::{collections::HashMap, sync::Arc};

//存储变量名和位置
#[derive(Default)]
pub struct ConstPool {
    Pool: Vec<Arc<super::value_type::Type>>,
}

const CONST_POOL_INIT_CAP: usize = 8;

impl ConstPool {
    pub fn new(&self) -> Self {
        Self {
            Pool: Vec::with_capacity(CONST_POOL_INIT_CAP),
        }
    }
    pub fn add(&mut self, value: super::value_type::Type) -> Result<()> {
        self.Pool.push(Arc::new(value));
        Ok(())
    }
}
