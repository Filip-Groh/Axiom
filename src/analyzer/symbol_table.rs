use std::collections::{HashMap};

pub struct SymbolTable {
    symbol_tables: Vec<HashMap<String, ()>>
}

impl SymbolTable {
    pub fn new() -> SymbolTable {
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
    
    pub fn add(&mut self, identifier: String) -> Option<()> {
        self.symbol_tables.last_mut().unwrap().insert(identifier, ())
    }
    
    pub fn get(&self, identifier: &String) -> Option<&()> {
        self.symbol_tables.iter().rev().find_map(|symbol_table| symbol_table.get(identifier))
    }
    
    pub fn has(&self, identifier: &String) -> bool {
        self.symbol_tables.iter().any(|symbol_table| symbol_table.contains_key(identifier))
    }
}