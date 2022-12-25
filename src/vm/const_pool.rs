use anyhow::{Ok, Result};
use std::{collections::HashMap, sync::Arc};

//存储变量名和位置
#[derive(Default)]
pub struct ConstPool {
    Pool: Vec<Arc<super::value_type::Type>>,
    Name: HashMap<String,usize>,
}

const CONST_POOL_INIT_CAP: usize = 8;

impl ConstPool {
    pub fn new(&self) -> Self {
        Self {
            Pool: Vec::with_capacity(CONST_POOL_INIT_CAP),
            Name: HashMap::new(),
        }
    }
    pub fn add(&mut self, value: super::value_type::Type,name:String) -> Result<()> {
        self.Pool.push(Arc::new(value));
        self.Name.insert(name,self.Pool.len());
        Ok(())
    }
}
