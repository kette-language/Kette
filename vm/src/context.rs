use crate::builtin::Builtin;
use crate::lexer::{Lexer, Word};
use crate::symbol::{Symbol, SymbolId, SymbolKind, SymbolStorage, SymbolValue};
use std::collections::{HashMap, LinkedList};
use std::sync::{Arc, Mutex, RwLock};
use crate::cfg::{CFG, CFGNodeId, PreCFGNode};
use crate::jit::Jitter;
use crate::sources::{Entry, Source, SourceId, Sources, SourceSource};

pub struct Context {
    pub symbols: Arc<RwLock<SymbolStorage>>,
    pub sources: Arc<RwLock<Sources>>,
    pub jitter: Arc<RwLock<Jitter>>,
    pub cfg: Arc<RwLock<CFG>>,
    pub cfg_stack: LinkedList<CFGNodeId>
}

impl Context {
    pub fn new() -> Self {
        let symbols = Default::default();
        let sources = Default::default();
        let mut cfg = CFG::new();
        let mut cfg_stack = LinkedList::new();
        let id = cfg.add_root();
        cfg_stack.push_back(id);
        Self {
            symbols,
            sources,
            jitter: Arc::new(RwLock::new(Jitter::new())),
            cfg: Arc::new(RwLock::new(cfg)),
            cfg_stack,
        }
    }

    pub fn execute(&mut self, code: &str) -> crate::Result<()> {
        let (source_id, source) = {
            let mut sources = self.sources.write().unwrap();
            let id = sources.new_source(SourceSource::String(code.as_ptr()));
            let source = sources.get_source(id).unwrap();
            (id, source)
        };

        let mut lexer = Lexer::new(code);

        loop {
            if let Some(word) = lexer.next_word() {
                self.parse_word(&mut lexer, word, (source_id, source.clone()));
            } else {
                break;
            }
        }

        let root = self.cfg_stack.front().unwrap();
        let memory = {
            let mut jitter = self.jitter.write().unwrap();
            {
                let cfg = self.cfg.read().unwrap();
                jitter.compile_cfg(&cfg, root);
            }
            println!("opcodes: {:02X?}", jitter.next);
            jitter.compile()
        };
        let executable = memory.to_fn();
        let result = executable(0);
        println!("result: {}", result);
        Ok(())
    }

    pub fn parse_word(&mut self,  _lexer: &mut Lexer, word: Word, source: (SourceId, Arc<Mutex<Source>>)) -> Option<CFGNodeId> {
        if word.data.starts_with("\"") {
            todo!("implement Strings");
        } else if word.data == "[" {
            todo!("implement quotations")
        } else if word.data == ":" {
            todo!("implement functions")
        } else if let Some(builtin) = Builtin::from_str(word.data) {
            let top_id = self.cfg_stack.back().unwrap();
            let node = PreCFGNode::Builtin(builtin);
            let mut cfg = self.cfg.write().unwrap();
            Some(cfg.insert(top_id, node))
        } else {
            todo!("implement symbol lookups")
        }
    }

    pub fn parse_until<'a, 'b>(lexer: &mut Lexer, accum: &'b mut Vec<CFGNodeId>, until: String) -> &'b mut Vec<CFGNodeId> {
        accum
    }

    pub fn parse_function_def(lexer: &mut Lexer) {

    }

    pub fn populate_default_symbols(&mut self) {
        // self.symbols.
    }
}
