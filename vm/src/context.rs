use crate::builtin::{Builtin, Number};
use crate::lexer::{Lexer, Word};
use crate::symbol::{Symbol, SymbolId, SymbolKind, SymbolStorage};
use std::collections::{HashMap, LinkedList};
use std::sync::{Arc, Mutex, RwLock};
use crate::jit::compiler::Compiler;
use crate::trees::{CFG, CFGNodeId, InsertCFGNode};
use crate::sources::{Entry, Source, SourceId, Sources, SourceSource};

pub struct Context {
    symbols: Arc<RwLock<SymbolStorage>>,
    sources: Arc<RwLock<Sources>>,
    compiler: Arc<RwLock<Compiler>>,
    cfg: Arc<RwLock<CFG>>,
    cfg_stack: LinkedList<CFGNodeId>,
}

impl Context {
    pub fn new() -> Self {
        let symbols = Default::default();
        let sources = Default::default();
        let cfg = Arc::new(RwLock::new(CFG::new()));
        let cfg_stack = LinkedList::new();
        let compiler = Arc::new(RwLock::new(Compiler::new()));
        let mut new = Self {
            symbols,
            sources,
            compiler,
            cfg,
            cfg_stack,
        };
        new.populate_default_symbols();
        new
    }

    pub fn execute(&mut self, code: &str) -> crate::Result<()> {
        let (source_id, source) = {
            let mut sources = self.sources.write().unwrap();
            let id = sources.new_source(SourceSource::String(code.as_ptr()));
            let source = sources.get_source(id).unwrap();
            (id, source)
        };

        let (root_id, scope_id) = {
            self.cfg.write().unwrap().add_root()
        };

        self.cfg_stack.push_back(scope_id);

        let mut lexer = Lexer::new(code);

        while let Some(word) = lexer.next_word() {
            self.read_word(&mut lexer, word);
        }
        {
            let mut cfg = self.cfg.write().unwrap();
            let mut buffer = String::new();
            cfg.debug_traverse(&self.symbols.read().unwrap(), &root_id, &mut buffer, 0);
            println!("{}", buffer);
            cfg.lower(self.symbols.clone(), &root_id);
            let mut buffer = String::new();
            cfg.debug_traverse(&self.symbols.read().unwrap(), &root_id, &mut buffer, 0);
            println!("{}", buffer);
        }

        let (id, compilation) = {
            let mut compiler = self.compiler.write().unwrap();
            let tree = self.cfg.read().unwrap();
            let (id, compilation) = compiler.compile(&tree, self.symbols.clone(), root_id);
            (id, compilation)
        };
        println!("{}", compilation.to_string());
        println!("opcodes: {:02X?}", compilation.opcodes);
        let fun = compilation.memory.to_fn();
        let res = fun(0);
        println!("result: {}", res);
        Ok(())
    }

    pub fn read_word(&mut self, _lexer: &mut Lexer, word: Word) {
        if let Ok(symbols) = self.symbols.read() {
            if let Some(id) = symbols.get_id(word.data) {
                let symbol = symbols.get(&id);
                let node_id = self.cfg_stack.back().unwrap();
                {
                    let mut cfg = self.cfg.write().unwrap();
                    cfg.insert(node_id, InsertCFGNode::Symbol(id));
                }
                match symbol.kind {
                    SymbolKind::ReaderMacro => { todo!() }
                    _ => { }
                }
                return;
            }
        }

        if let Some(num) = Number::parse(word.data) {
            let builtin = Builtin::Number(num);
            let id = {
                let mut symbols = self.symbols.write().unwrap();
                builtin.add_symbol(&mut symbols)
            };

            let node_id = self.cfg_stack.back().unwrap();
            let mut cfg = self.cfg.write().unwrap();
            cfg.insert(node_id, InsertCFGNode::Symbol(id));
        } else if word.data.starts_with("\"") {
            todo!("implement Strings");
        } else if word.data == "[" {
            todo!("implement quotations")
        } else if word.data == ":" {
            todo!("implement functions")
        } else {
            unreachable!()
        }
    }

    // pub fn parse_until<'a, 'b>(lexer: &mut Lexer, accum: &'b mut Vec<CFGNodeId>, until: String) -> &'b mut Vec<CFGNodeId> {
    //     accum
    // }

    pub fn parse_function_def(lexer: &mut Lexer) {}

    pub fn populate_default_symbols(&mut self) {
        let mut symbols = self.symbols.write().unwrap();
        Builtin::Add.add_symbol(&mut symbols);
        Builtin::Sub.add_symbol(&mut symbols);
        Builtin::Mul.add_symbol(&mut symbols);
        Builtin::Div.add_symbol(&mut symbols);
        Builtin::DotPrint.add_symbol(&mut symbols);
        Builtin::Drop.add_symbol(&mut symbols);
    }
}
