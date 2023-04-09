use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use crate::builtin::{Builtin, Number};
use crate::symbol::{StackEffect, SymbolId, SymbolStorage};

#[derive(Debug)]
pub enum InsertCFGNode {
    Quotation(Vec<SymbolId>),
    Symbol(SymbolId),
    Unknown(SymbolId),
}


pub type CFGNodeId = usize;

#[derive(Debug)]
pub enum CFGNode {
    Root(Vec<CFGNodeId>),
    Scope {
        parent: CFGNodeId,
        body: Vec<CFGNodeId>
    },
    Quotation(CFGNodeId),
    Function {
        symbol: SymbolId,
        stack_effect: StackEffect,
        body: CFGNodeId,
        inline: bool,
        recursive: bool,
    },
    ReaderMacro {
        symbol: SymbolId,
        body: CFGNodeId,
    },
    Macro {
        symbol: SymbolId,
        body: CFGNodeId,
    },
    Number(Number),
    Builtin(Builtin),
    Symbol(SymbolId),
    Unknown(SymbolId),
}

#[derive(Debug)]
pub struct CFG {
    roots: Vec<CFGNodeId>,
    mappings: HashMap<SymbolId, CFGNodeId, ahash::RandomState>,
    pub(super) nodes: HashMap<CFGNodeId, CFGNode>,
    next_id: CFGNodeId,
}

impl CFG {
    pub fn new() -> Self {
        Self {
            roots: Default::default(),
            mappings: Default::default(),
            nodes: Default::default(),
            next_id: 0,
        }
    }

    pub fn insert(&mut self, scope: &CFGNodeId, node: InsertCFGNode) -> CFGNodeId {
        let node = match node {
            InsertCFGNode::Quotation(_quote) => {
                todo!()
            }
            InsertCFGNode::Symbol(id) => {
                CFGNode::Symbol(id)
            }
            InsertCFGNode::Unknown(id) => {
                CFGNode::Unknown(id)
            }
        };

        let id = self.next_id();
        self.nodes.insert(id, node);

        match self.nodes.get_mut(scope).unwrap() {
            CFGNode::Scope { body, .. } => {
                body.push(id);
            }
            _ => { panic!("Invalid Id for Scope")}
        }
        id
    }

    pub fn add_root(&mut self) -> (CFGNodeId, CFGNodeId) {
        let root_id = self.next_id();
        let scope_id = self.next_id();
        let scope = CFGNode::Scope { parent: root_id, body: vec![] };
        let root = CFGNode::Root(vec![scope_id]);
        self.roots.push(root_id);
        self.nodes.insert(root_id, root);
        self.nodes.insert(scope_id, scope);
        (root_id, scope_id)
    }

    pub fn get(&self, id: &CFGNodeId) -> &CFGNode {
        self.nodes.get(id).unwrap()
    }

    fn next_id(&mut self) -> CFGNodeId {
        let id = self.next_id;
        self.next_id += 1;
        id
    }

    pub fn debug_traverse(&self, symbols: &SymbolStorage, node_id: &CFGNodeId, buffer: &mut String, depth: usize) {
        for _ in 0..depth {
            buffer.push(' ');
        }
        let node = self.nodes.get(node_id).unwrap();
        match node {
            CFGNode::Root(root) => {
                buffer.push_str(&format!("root: {}\n", node_id));
                for node in root {
                    self.debug_traverse(symbols, &node,  buffer, depth + 2);
                }
            }
            CFGNode::Scope { parent, body } => {
                buffer.push_str(&format!("scope: {} <-- {}\n", node_id, parent));
                for node in body {
                    self.debug_traverse(symbols, &node, buffer, depth + 2);
                }
            }
            CFGNode::Quotation(_) => { todo!() }
            CFGNode::Function { .. } => { todo!() }
            CFGNode::ReaderMacro { .. } => { todo!() }
            CFGNode::Macro { .. } => { todo!() }
            CFGNode::Symbol(id) => {
                let symbol = symbols.get(&id);
                buffer.push_str(&format!("symbol: {} --> {} | {:?}\n", node_id, id, symbol));
            }
            CFGNode::Unknown(_) => { todo!() }
            CFGNode::Number(num) => {
                buffer.push_str(&format!("number: {} --> {:?}\n", node_id, num));
            }
            CFGNode::Builtin(bt) => {
                buffer.push_str(&format!("builtin: {} --> {:?}\n", node_id, bt));
            }
        }
    }
}

impl Default for CFG {
    fn default() -> Self {
        Self::new()
    }
}