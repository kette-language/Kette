use crate::builtin::{Builtin, Number};
use std::collections::HashMap;
use std::sync::RwLock;
use crate::sources::{EntryId, SourceId};

#[derive(Debug, Clone)]
pub struct StackEffect {
    pub inputs: Vec<String>,
    pub outputs: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct FunctionDef {
    symbol: SymbolId,
    stack_effect: StackEffect,
    body: Vec<SymbolId>,
    inline: bool,
    recursive: bool,
}

#[derive(Debug, Clone)]
pub enum SymbolKind {
    Builtin(Builtin),
    Function(FunctionDef),
    Macro,
    ReaderMacro,
}

pub type SymbolId = usize;

#[derive(Debug)]
pub struct Symbol {
    pub identifier: String, // extra identifier so the key can be more "dynamic" e.g. "ident#234"
    pub kind: SymbolKind,
}

pub struct SymbolStorage {
    table: HashMap<SymbolId, Symbol, ahash::RandomState>,
    mappings: HashMap<String, SymbolId, ahash::RandomState>,
    next_id: SymbolId,
}

impl SymbolStorage {
    pub fn new() -> Self {
        Self {
            table: Default::default(),
            mappings: Default::default(),
            next_id: Default::default(),
        }
    }

    pub fn get_id(&self, key: &str) -> Option<SymbolId> {
        self.mappings.get_key_value(key).map(|(_, id)| *id)
    }

    pub fn get(&self, id: &SymbolId) -> &Symbol {
        self.table.get(id).unwrap()
    }

    pub fn insert_or_get(&mut self, key: String, ident: String, kind: SymbolKind) -> SymbolId {
        if let Some((_, id)) = self.mappings.get_key_value(&key) {
            return *id;
        }
        let id = self.next_id;
        self.mappings.insert(key, id);
        let symbol = Symbol { identifier: ident, kind };
        self.table.insert(id,symbol );
        self.next_id += 1;
        return id;

    }
}

impl Default for SymbolStorage {
    fn default() -> Self {
        Self::new()
    }
}