use std::cell::Ref;
use std::ops::Deref;
use std::rc::Rc;
use std::sync::{Arc, RwLock};
use crate::builtin::Builtin;
use crate::trees::{CFG, CFGNode, CFGNodeId};
use crate::symbol::{SymbolKind, SymbolStorage};

impl CFG {
    pub fn lower(&mut self, symbols: Arc<RwLock<SymbolStorage>>, node: &CFGNodeId) {
        let node = {
            unsafe { &mut *(self.nodes.get_mut(node).unwrap() as *mut _) }
        };
        match node {
            CFGNode::Root(ref root) => {
                for node in root {
                    self.lower(symbols.clone(), node);
                }
            }
            CFGNode::Scope { ref body, .. } => {
                for node in body {
                    self.lower(symbols.clone(), node);
                }
            }
            CFGNode::Quotation(_) => {}
            CFGNode::Function { .. } => {}
            CFGNode::ReaderMacro { .. } => {}
            CFGNode::Macro { .. } => {}
            CFGNode::Symbol(id) => {
                let symbol = symbols.read().unwrap().get(id).kind.clone();
                match symbol {
                    SymbolKind::Builtin(b) => {
                        match b {
                            Builtin::Number(num) => {
                                *node = CFGNode::Number(num);
                            }
                            _ => {
                                *node = CFGNode::Builtin(b);
                            }
                        }
                    }
                    SymbolKind::Function(_) => {}
                    SymbolKind::Macro => {}
                    SymbolKind::ReaderMacro => {}
                }
            }
            CFGNode::Unknown(_) => {}
            _ => {}
        }
    }
}