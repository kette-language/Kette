use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use crate::architecture::x64::instruction::{AddInstruction, AssemblyInstruction, Instruction, Operand, PopInstruction, PushInstruction, Register, RetInstruction, Size};
use crate::builtin::{Builtin, Number};
use crate::trees::{CFG, CFGNode, CFGNodeId};
use crate::jit::memory::ExecutableMemory;
use crate::symbol::{SymbolId, SymbolStorage};

pub type CompilationUnitId = usize;

#[derive(Debug, Copy, Clone)]
pub struct CompiledSymbol {
    pub compilation_unit: CompilationUnitId,
    pub offset: usize,
}

pub struct Compilation {
    pub memory: ExecutableMemory,
    pub opcodes: Vec<u8>, // TODO: remove this
    pub instructions: Vec<Box<dyn Instruction>>,
}

impl Compilation {
    pub fn to_string(&self) -> String {
        let mut buffer = String::new();
        for i in &self.instructions {
            buffer.push_str(&format!("{}\n", i.to_string()))
        };
        buffer
    }
}

pub struct Compiler {
    mappings: HashMap<SymbolId, CompiledSymbol, ahash::RandomState>,
    units: HashMap<CompilationUnitId, Arc<Compilation>, ahash::RandomState>,
    next_id: CompilationUnitId,
}

impl Compiler {
    pub fn new() -> Self {
        Self {
            mappings: Default::default(),
            units: Default::default(),
            next_id: 0,
        }
    }

    pub fn get(&self, id: &CompilationUnitId) -> &Compilation {
        self.units.get(&id).unwrap()
    }

    pub fn compile(&mut self, tree: &CFG, symbols: Arc<RwLock<SymbolStorage>>, start: CFGNodeId) -> (CompilationUnitId, Arc<Compilation>) {
        let mut instructions = Vec::new();
        self.compile_inner(&mut instructions, tree, symbols, &start);
        let mut buffer = Vec::new();
        for instr in &instructions {
            Self::write_instruction(&mut buffer, instr.build());
        }
        Self::write_instruction(&mut buffer, RetInstruction.build());
        let mut memory = ExecutableMemory::new(buffer.len());
        unsafe { std::ptr::copy(buffer.as_ptr(), memory.as_mut_ptr(), buffer.len()) }
        let compilation = Arc::new(Compilation { memory, opcodes: buffer, instructions });

        let id = self.next_id;
        self.next_id += 1;
        self.units.insert(id, compilation.clone());
        (id, compilation)
    }

    fn compile_inner(&mut self, buffer: &mut Vec<Box<dyn Instruction>>, tree: &CFG, symbols: Arc<RwLock<SymbolStorage>>, id: &CFGNodeId) {
        let node = tree.get(id);
        match node {
            CFGNode::Root(nodes) => {
                for node_id in nodes {
                    self.compile_inner(buffer, tree, symbols.clone(), node_id);
                }
            }
            CFGNode::Scope { body, .. } => {
                for node_id in body {
                    self.compile_inner(buffer, tree, symbols.clone(), node_id);
                }
            }
            CFGNode::Quotation(_) => { todo!() }
            CFGNode::Function { .. } => { todo!() }
            CFGNode::Macro { .. } => { todo!() }
            CFGNode::Number(number) => match number {
                Number::Uint(num) => {
                    let num = *num;
                    let size = if let Ok(_) = u8::try_from(num) {
                        Size::Byte1
                    } else if let Ok(_) = u32::try_from(num) {
                        Size::Byte4
                    } else {
                        Size::Byte8
                    };
                    buffer.push(Box::new(PushInstruction {
                        op: Operand::Constant((num, size)),
                    }));
                }
                Number::Int(num) => {
                    let num = *num;
                    let size = if let Ok(_) = i8::try_from(num) {
                        Size::Byte1
                    } else if let Ok(_) = i32::try_from(num) {
                        Size::Byte4
                    } else {
                        Size::Byte8
                    };
                    buffer.push(Box::new(PushInstruction {
                        op: Operand::Constant((num as u64, size)),
                    }));
                }
                Number::Float(_) => {}
                Number::Ratio(_, _) => { unimplemented!() }
            }
            CFGNode::Builtin(b) => match b {
                Builtin::Add => {
                    buffer.push(Box::new(PopInstruction {
                        op: Operand::Register(Register::RCX)
                    }));
                    buffer.push(Box::new(PopInstruction {
                        op: Operand::Register(Register::RAX)
                    }));
                    buffer.push(Box::new(AddInstruction {
                        src: Operand::Register(Register::RCX),
                        dst: Operand::Register(Register::RAX),
                    }));
                    buffer.push(Box::new(PushInstruction {
                        op: Operand::Register(Register::RAX),
                    }));
                }
                Builtin::Sub => { todo!() }
                Builtin::Mul => { todo!() }
                Builtin::Div => { todo!() }
                Builtin::DotPrint => { todo!() }
                Builtin::Drop => {
                    buffer.push(Box::new(PopInstruction {
                        op: Operand::Register(Register::RAX)
                    }));
                }
                _ => {}
            }
            CFGNode::Symbol(sym) => { todo!() }
            CFGNode::Unknown(_) => { panic!() }
            _ => {}
        }
    }


    pub fn write_instruction(buffer: &mut Vec<u8>, instruction: AssemblyInstruction) {
        let AssemblyInstruction {
            legacy_prefix,
            prefix,
            opcode,
            modrm,
            sib,
            displacement,
            immediate,
        } = instruction;
        if let Some((value, size)) = legacy_prefix {
            Self::write_sized(buffer, value as u64, size)
        }
        if let Some((value, size)) = prefix {
            Self::write_sized(buffer, value as u64, size)
        }
        Self::write_sized(buffer, opcode.0 as u64, opcode.1);
        if let Some(value) = modrm {
            buffer.push(value);
        }
        if let Some(value) = sib {
            buffer.push(value);
        }
        if let Some((value, size)) = displacement {
            Self::write_sized(buffer, value, size)
        }
        if let Some((value, size)) = immediate {
            Self::write_sized(buffer, value as u64, size)
        }
    }

    fn write_sized(buffer: &mut Vec<u8>, value: u64, size: Size) {
        match size {
            Size::Byte1 => {
                buffer.push((value >> 0) as u8);
            }
            Size::Byte2 => {
                buffer.push((value >> 0) as u8);
                buffer.push((value >> 8) as u8);
            }
            Size::Byte3 => {
                buffer.push((value >> 0) as u8);
                buffer.push((value >> 8) as u8);
                buffer.push((value >> 16) as u8);
            }
            Size::Byte4 => {
                buffer.push((value >> 0) as u8);
                buffer.push((value >> 8) as u8);
                buffer.push((value >> 16) as u8);
                buffer.push((value >> 24) as u8);
            }
            Size::Byte8 => {
                buffer.push((value >> 0) as u8);
                buffer.push((value >> 8) as u8);
                buffer.push((value >> 16) as u8);
                buffer.push((value >> 24) as u8);
                buffer.push((value >> 32) as u8);
                buffer.push((value >> 40) as u8);
                buffer.push((value >> 48) as u8);
                buffer.push((value >> 56) as u8);
            }
        }
    }
}