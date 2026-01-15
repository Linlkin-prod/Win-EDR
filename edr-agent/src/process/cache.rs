use std::collections::HashMap;
use crate::model::ProcessContext;

pub struct ProcessCache {
    cache: HashMap<u32, ProcessContext>,
}

impl ProcessCache {
    pub fn new() -> Self {
        Self {
            cache: HashMap::new(),
        }
    }

    pub fn insert(&mut self, pid: u32, context: ProcessContext) {
        self.cache.insert(pid, context);
    }

    #[allow(dead_code)]
    pub fn get(&self, pid: &u32) -> Option<&ProcessContext> {
        self.cache.get(pid)
    }
}

