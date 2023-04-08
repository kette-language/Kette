use crate::builtin::{Builtin, Number};
use std::collections::HashMap;
use std::sync::RwLock;
use crate::sources::{EntryId, SourceId};

pub type SymbolId = usize;
pub type SymbolMappings = RwLock<HashMap<String, SymbolId, ahash::RandomState>>;
pub type SymbolTable = RwLock<HashMap<SymbolId, Symbol, ahash::RandomState>>;

pub enum SymbolValue {
    Number(Number),
    String(Vec<(SourceId, EntryId)>)
}

pub enum SymbolKind {
    Value(SymbolValue),
    Builtin,
    Function,
    Macro,
    ReaderMacro,
}

pub struct Symbol {
    pub identifier: String,
    pub source: (SourceId, EntryId),
    pub kind: SymbolKind,
    pub parent: Option<SymbolId>,
}

pub struct SymbolStorage {
    table: SymbolTable,
    mappings: SymbolMappings,
    next_id: RwLock<SymbolId>,
}

impl SymbolStorage {
    pub fn new() -> Self {
        Self {
            table: Default::default(),
            mappings: Default::default(),
            next_id: Default::default(),
        }
    }

    // pub fn insert_or_get() -> Symbol {
    //
    // }
}

impl Default for SymbolStorage {
    fn default() -> Self {
        Self::new()
    }
}