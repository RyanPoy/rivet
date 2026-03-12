use crate::sequel::term::table::TableInner;
use std::collections::HashMap;
use std::sync::Arc;

pub struct AliasCache {
    all_alias: HashMap<String, usize>,
    mapping: HashMap<usize, (String, Option<String>)>,
}
impl AliasCache {
    pub fn new() -> Self {
        Self {
            all_alias: HashMap::new(),
            mapping: HashMap::new(),
        }
    }
    pub fn add(&mut self, table_inner: &Arc<TableInner>, name: String, default_alias: Option<String>) {
        let addr = Arc::as_ptr(table_inner) as usize;
        if !self.mapping.contains_key(&addr) {
            let mut n = *self.all_alias.get(&name).unwrap_or(&0);
            self.mapping.insert(addr, (format!("{}{}", name, n), default_alias));
            self.all_alias.insert(name, n + 1);
        }
    }

    pub fn alias_of(&self, table_inner: &Arc<TableInner>) -> Option<String> {
        let addr = Arc::as_ptr(table_inner) as usize;
        if let Some((name, alias)) = self.mapping.get(&addr) {
            if alias.is_none() {
                Some(name.clone())
            } else {
                alias.clone()
            }
        } else {
            None
        }
    }
}
