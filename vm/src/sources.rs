use std::collections::HashMap;
use std::ffi::c_void;
use std::sync::{Arc, Mutex, RwLock};

pub type SourceId = usize;
pub type EntryId = usize;

// TODO: use Ids for this
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SourceSource {
    String(*const u8), // lifetimes ;)))))
    File(String),
    Repl(String),
    Other(*const c_void) // TODO: lol
}

impl SourceSource {
    pub fn name(&self) -> String {
        match self {
            SourceSource::String(s) => "".to_string(),
            SourceSource::File(p) => p.clone(),
            SourceSource::Repl(id) => id.clone(),
            SourceSource::Other(_) => "".to_string(),
        }
    }
}

#[derive(Debug)]
pub struct Source {
    pub name: String,
    pub kind: SourceSource,
    pub entries: HashMap<EntryId, Entry>,
    pub next_id: EntryId,
}

#[derive(Debug)]
pub struct Entry {
    pub line: u64,
    pub column: u64,
}

#[derive(Debug)]
pub struct Sources {
    table: HashMap<SourceId, Arc<Mutex<Source>>, ahash::RandomState>,
    mappings: HashMap<SourceSource, SourceId, ahash::RandomState>,
    next_id: SourceId,
}

impl Sources {
    pub fn new() -> Self {
        Self {
            table: Default::default(),
            mappings: Default::default(),
            next_id: 0,
        }
    }

    pub fn new_source(&mut self, source: SourceSource) -> SourceId {
        let source_id = {
            self.mappings.insert(source.clone(), self.next_id);
            self.next_id += 1;
            *self.mappings.get(&source).unwrap()
        };
        let source = Source::new(source);
        self.table.insert(source_id, Arc::new(Mutex::new(source)));
        source_id
    }

    pub fn get_source(&self, id: SourceId) -> Option<Arc<Mutex<Source>>> {
        self.table.get(&id).map(|s| s.clone())
    }
}

impl Source {
    pub fn new(kind: SourceSource) -> Self {
        Self {
            name: kind.name(),
            kind,
            entries: Default::default(),
            next_id: 0,
        }
    }

    pub fn new_entry(&mut self, entry: Entry) -> EntryId {
        let id = self.next_id;
        self.entries.insert(id, entry);
        self.next_id += 1;
        id
    }
}

impl Default for Sources {
    fn default() -> Self {
        Self::new()
    }
}