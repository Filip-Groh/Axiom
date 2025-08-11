use std::collections::{HashMap};
use std::hash::Hash;

pub struct SymbolTable<K: Eq + Hash, V> {
    symbol_tables: Vec<HashMap<K, V>>
}

impl<K: Eq + Hash, V> SymbolTable<K, V> {
    pub fn new() -> SymbolTable<K, V> {
        SymbolTable {
            symbol_tables: vec![HashMap::new()]
        }
    }
    
    pub fn push(&mut self) {
        self.symbol_tables.push(HashMap::new());
    }
    
    pub fn pop(&mut self) {
        self.symbol_tables.pop();
    }
    
    pub fn add(&mut self, identifier: K, value: V) -> Option<V> {
        self.symbol_tables.last_mut().unwrap().insert(identifier, value)
    }
    
    pub fn get(&self, identifier: &K) -> Option<&V> {
        self.symbol_tables.iter().rev().find_map(|symbol_table| symbol_table.get(identifier))
    }
    
    pub fn has(&self, identifier: &K) -> bool {
        self.symbol_tables.iter().any(|symbol_table| symbol_table.contains_key(identifier))
    }
}