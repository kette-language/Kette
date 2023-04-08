use std::collections::HashMap;
use std::rc::Rc;
use std::sync::{Arc, Mutex};
use crate::builtin::Builtin;
use crate::symbol::SymbolId;

#[derive(Debug)]
pub struct StackEffect {
    pub inputs: Vec<String>,
    pub outputs: Vec<String>,
}

#[derive(Debug)]
pub enum PreCFGNode {
    Quotation(Vec<SymbolId>),
    Function {
        symbol: SymbolId,
        stack_effect: StackEffect,
        body: Vec<SymbolId>,
        inline: bool,
        recursive: bool,
    },
    QuotationCall,
    FunctionCall(SymbolId),
    Builtin(Builtin),
    YetUnknown(SymbolId),
    Other(String),
}


pub type CFGNodeId = usize;

#[derive(Debug)]
pub enum CFGNode {
    Global(Vec<CFGNodeId>),
    Quotation(Vec<CFGNodeId>),
    Function {
        symbol: SymbolId,
        stack_effect: StackEffect,
        body: Vec<CFGNodeId>,
        inline: bool,
        recursive: bool,
    },
    QuotationCall,
    FunctionCall(SymbolId),
    Builtin(Builtin),
    YetUnknown(String),
    Placeholder,
    Other(String)
}

#[derive(Debug)]
pub struct CFG {
    pub mappings: HashMap<SymbolId, CFGNodeId, ahash::RandomState>,
    pub nodes: HashMap<CFGNodeId, CFGNode>,
    pub next_id: CFGNodeId,
}

impl CFG {
    pub fn new() -> Self {
        Self {
            mappings: Default::default(),
            nodes: Default::default(),
            next_id: 0,
        }
    }

    pub fn insert(&mut self, scope: &CFGNodeId, node: PreCFGNode) -> CFGNodeId {
        match node {
            PreCFGNode::Quotation(_) => { todo!() }
            PreCFGNode::Function { .. } => { todo!() }
            PreCFGNode::QuotationCall => { todo!() }
            PreCFGNode::FunctionCall(_) => { todo!() }
            PreCFGNode::Builtin(builtin) => {
                let node = CFGNode::Builtin(builtin);
                let id = self.next_id;
                self.nodes.insert(id, node);
                match self.nodes.get_mut(scope).unwrap() {
                    CFGNode::Global(content) => {
                        content.push(id);
                    }
                    CFGNode::Quotation(_) => { todo!() }
                    CFGNode::Function { .. } => { todo!() }
                    CFGNode::Placeholder => { todo!() }
                    _ => panic!("Invalid Scope Node"),
                };
                self.next_id += 1;
                id
            }
            PreCFGNode::YetUnknown(_) => { todo!() }
            PreCFGNode::Other(_) => { todo!() }
        }
    }

    pub fn add_root(&mut self) -> CFGNodeId {
        let node = CFGNode::Global(Vec::new());
        let id = self.next_id;
        self.nodes.insert(id, node);
        self.next_id += 1;
        id
    }

    pub fn add_placeholder(&mut self) -> CFGNodeId {
        let node = CFGNode::Placeholder;
        let id = self.next_id;
        self.nodes.insert(id, node);
        self.next_id += 1;
        id
    }
}

impl Default for CFG {
    fn default() -> Self {
        Self::new()
    }
}