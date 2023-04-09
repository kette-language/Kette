use std::collections::HashMap;
use crate::cfg::CFG;
use crate::jit::memory::ExecutableMemory;
use crate::symbol::SymbolId;

pub type CompilationUnitId = usize;

#[derive(Debug, Copy, Clone)]
pub struct CompiledSymbol {
    compilation_unit: CompilationUnitId,
    offset: usize
}

pub struct Compilation {
    pub memory: ExecutableMemory,
}

pub struct Compiler {
    pub mappings: HashMap<SymbolId, CompiledSymbol, ahash::RandomState>,
    pub units: HashMap<CompilationUnitId, Compilation, ahash::RandomState>,
    pub next_id: CompilationUnitId,
}

impl Compiler {
    pub fn new() -> Self {
        Self {
            mappings: Default::default(),
            units: Default::default(),
            next_id: 0
        }
    }

    // pub fn compile(cfg: &CFG) -> CompilationUnitId {
    //
    // }
}