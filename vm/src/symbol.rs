use crate::builtin::Builtin;
use std::collections::HashMap;
use std::sync::RwLock;

pub type SymbolId = usize;
pub type SymbolMappings = RwLock<HashMap<String, SymbolId>>;
pub type SymbolTable = RwLock<HashMap<SymbolId, Symbol, ahash::RandomState>>;

pub enum SymbolKind {
    Normal,
    Builtin(Builtin),
    Macro,
    ReaderMacro,
}

pub struct Symbol {
    pub identifier: String,
    pub kind: SymbolKind,
    pub references: Vec<SymbolId>,
}

pub struct SymbolStorage {
    table: SymbolTable,
    mappings: SymbolMappings,
    next_id: SymbolId,
}

impl SymbolStorage {
    pub fn new() -> Self {
        Self {
            table: Default::default(),
            mappings: Default::default(),
            next_id: 0,
        }
    }
}
