use crate::builtin::Builtin;
use crate::lexer::Lexer;
use crate::symbol::SymbolStorage;
use std::collections::HashMap;
use std::sync::RwLock;

pub struct Context {
    pub symbols: SymbolStorage,
}

impl Context {
    pub fn new() -> Self {
        Self {
            symbols: SymbolStorage::new(),
        }
    }

    pub fn execute_str(&mut self, code: &str) -> crate::Result<()> {
        let mut lexer = Lexer::new(code);
        while let Some(word) = lexer.next_word() {
            println!("{:?}", word);
        }
        Ok(())
    }

    pub fn populate_default_symbols(&mut self) {}
}
