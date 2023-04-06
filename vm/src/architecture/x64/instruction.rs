use crate::memory::ExecutableMemory;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(u8)]
pub enum Size {
    Byte1 = 1,
    Byte2 = 2,
    Byte3 = 3,
    Byte4 = 4,
    Byte8 = 8,
}

pub trait Instruction {
    fn build(&self) -> AssemblyInstruction;
}

#[derive(Debug, Copy, Clone)]
pub struct AssemblyInstruction {
    pub legacy_prefix: Option<(u32, Size)>,
    pub prefix: Option<(u32, Size)>,
    pub opcode: (u32, Size),
    pub modrm: Option<u8>,
    pub sib: Option<u8>,
    pub displacement: Option<(u64, Size)>,
    pub immediate: Option<(u64, Size)>,
}

impl Default for AssemblyInstruction {
    fn default() -> Self {
        Self {
            legacy_prefix: None,
            prefix: None,
            opcode: (0, Size::Byte1),
            modrm: None,
            sib: None,
            displacement: None,
            immediate: None,
        }
    }
}

pub struct MovInstruction {
    pub src: Operand,
    pub dst: Operand,
}

pub struct AddInstruction {
    src: Operand,
    dst: Operand,
}

pub struct SubInstruction {
    src: Operand,
    dst: Operand,
}

pub struct RetInstruction;

impl Instruction for MovInstruction {
    fn build(&self) -> AssemblyInstruction {
        let mut instruction = AssemblyInstruction::default();

        match &self.dst {
            Operand::Register(dst) => {
                instruction.prefix = dst.prefix;
                match &self.src {
                    Operand::Register(src) => {}
                    Operand::RegisterAddress((src_reg, src_offset, src_size)) => {
                        unimplemented!()
                    }
                    Operand::Address((src_addr, src_offset, src_dst)) => {
                        unimplemented!()
                    }
                    Operand::Constant((value, size)) => {
                        instruction.immediate = Some((*value, *size));

                        if !(size == &Size::Byte4 && dst.size == Size::Byte8) && size != &dst.size {
                            panic!("sizes do not match")
                        }
                        match dst.size {
                            Size::Byte1 => {}
                            Size::Byte2 => {}
                            Size::Byte3 => {
                                panic!("invalid register size")
                            }
                            Size::Byte4 => {
                                instruction.opcode = (0xb8 | dst.id as u32, Size::Byte1);
                            }
                            Size::Byte8 => {
                                if size == &Size::Byte8 {
                                    instruction.opcode = (0xb8 | dst.id as u32, Size::Byte1);
                                } else {
                                    instruction.opcode = (0xc7, Size::Byte1);
                                    instruction.modrm = Some(encode_modrm(3, 0, dst.id));
                                }
                            }
                        }
                    }
                }
            }
            Operand::RegisterAddress((dst_reg, dst_offset, dst_size)) => {
                unimplemented!()
            }
            Operand::Address((dst_addr, dst_offset, dst_size)) => {
                unimplemented!()
            }
            Operand::Constant(_) => {
                panic!("Destination cannot be constant")
            }
        };

        instruction
    }
}

impl Instruction for RetInstruction {
    fn build(&self) -> AssemblyInstruction {
        AssemblyInstruction {
            legacy_prefix: None,
            prefix: None,
            opcode: (0xC3, Size::Byte1),
            modrm: None,
            sib: None,
            displacement: None,
            immediate: None,
        }
    }
}

pub fn encode_modrm(m: u8, reg: u8, rm: u8) -> u8 {
    m << 6 | reg << 3 | rm
}

pub enum Operand {
    Register(Register),
    RegisterAddress((Register, i32, Size)),
    Address((u64, i32, Size)),
    Constant((u64, Size)),
}

pub struct Register {
    pub prefix: Option<(u32, Size)>,
    pub size: Size,
    pub id: u8,
}

impl Register {
    pub const RAX: Register = Register {
        prefix: Some((0x48, Size::Byte1)),
        size: Size::Byte8,
        id: 0,
    };
    pub const RCX: Register = Register {
        prefix: Some((0x48, Size::Byte1)),
        size: Size::Byte8,
        id: 1,
    };
    pub const RDX: Register = Register {
        prefix: Some((0x48, Size::Byte1)),
        size: Size::Byte8,
        id: 2,
    };
    pub const RBX: Register = Register {
        prefix: Some((0x48, Size::Byte1)),
        size: Size::Byte8,
        id: 3,
    };
    pub const RSP: Register = Register {
        prefix: Some((0x48, Size::Byte1)),
        size: Size::Byte8,
        id: 4,
    };
    pub const RBP: Register = Register {
        prefix: Some((0x48, Size::Byte1)),
        size: Size::Byte8,
        id: 5,
    };
    pub const RSI: Register = Register {
        prefix: Some((0x48, Size::Byte1)),
        size: Size::Byte8,
        id: 6,
    };
    pub const RDI: Register = Register {
        prefix: Some((0x48, Size::Byte1)),
        size: Size::Byte8,
        id: 7,
    };

    pub const R8: Register = Register {
        prefix: Some((0x49, Size::Byte1)),
        size: Size::Byte8,
        id: 0,
    };
    pub const R9: Register = Register {
        prefix: Some((0x49, Size::Byte1)),
        size: Size::Byte8,
        id: 1,
    };
    pub const R10: Register = Register {
        prefix: Some((0x49, Size::Byte1)),
        size: Size::Byte8,
        id: 2,
    };
    pub const R11: Register = Register {
        prefix: Some((0x49, Size::Byte1)),
        size: Size::Byte8,
        id: 3,
    };
    pub const R12: Register = Register {
        prefix: Some((0x49, Size::Byte1)),
        size: Size::Byte8,
        id: 4,
    };
    pub const R13: Register = Register {
        prefix: Some((0x49, Size::Byte1)),
        size: Size::Byte8,
        id: 5,
    };
    pub const R14: Register = Register {
        prefix: Some((0x49, Size::Byte1)),
        size: Size::Byte8,
        id: 6,
    };
    pub const R15: Register = Register {
        prefix: Some((0x49, Size::Byte1)),
        size: Size::Byte8,
        id: 7,
    };

    pub const EAX: Register = Register {
        prefix: None,
        size: Size::Byte4,
        id: 0,
    };
    pub const ECX: Register = Register {
        prefix: None,
        size: Size::Byte4,
        id: 1,
    };
    pub const EDX: Register = Register {
        prefix: None,
        size: Size::Byte4,
        id: 2,
    };
    pub const EBX: Register = Register {
        prefix: None,
        size: Size::Byte4,
        id: 3,
    };
    pub const ESP: Register = Register {
        prefix: None,
        size: Size::Byte4,
        id: 4,
    };
    pub const EBP: Register = Register {
        prefix: None,
        size: Size::Byte4,
        id: 5,
    };
    pub const ESI: Register = Register {
        prefix: None,
        size: Size::Byte4,
        id: 6,
    };
    pub const EDI: Register = Register {
        prefix: None,
        size: Size::Byte4,
        id: 7,
    };

    pub const R8D: Register = Register {
        prefix: Some((0x41, Size::Byte1)),
        size: Size::Byte4,
        id: 0,
    };
    pub const R9D: Register = Register {
        prefix: Some((0x41, Size::Byte1)),
        size: Size::Byte4,
        id: 1,
    };
    pub const R10D: Register = Register {
        prefix: Some((0x41, Size::Byte1)),
        size: Size::Byte4,
        id: 2,
    };
    pub const R11D: Register = Register {
        prefix: Some((0x41, Size::Byte1)),
        size: Size::Byte4,
        id: 3,
    };
    pub const R12D: Register = Register {
        prefix: Some((0x41, Size::Byte1)),
        size: Size::Byte4,
        id: 4,
    };
    pub const R13D: Register = Register {
        prefix: Some((0x41, Size::Byte1)),
        size: Size::Byte4,
        id: 5,
    };
    pub const R14D: Register = Register {
        prefix: Some((0x41, Size::Byte1)),
        size: Size::Byte4,
        id: 6,
    };
    pub const R15D: Register = Register {
        prefix: Some((0x41, Size::Byte1)),
        size: Size::Byte4,
        id: 7,
    };

    pub const AX: Register = Register {
        prefix: Some((0x66, Size::Byte1)),
        size: Size::Byte2,
        id: 0,
    };
    pub const CX: Register = Register {
        prefix: Some((0x66, Size::Byte1)),
        size: Size::Byte2,
        id: 1,
    };
    pub const DX: Register = Register {
        prefix: Some((0x66, Size::Byte1)),
        size: Size::Byte2,
        id: 2,
    };
    pub const BX: Register = Register {
        prefix: Some((0x66, Size::Byte1)),
        size: Size::Byte2,
        id: 3,
    };
    pub const SP: Register = Register {
        prefix: Some((0x66, Size::Byte1)),
        size: Size::Byte2,
        id: 4,
    };
    pub const BP: Register = Register {
        prefix: Some((0x66, Size::Byte1)),
        size: Size::Byte2,
        id: 5,
    };
    pub const SI: Register = Register {
        prefix: Some((0x66, Size::Byte1)),
        size: Size::Byte2,
        id: 6,
    };
    pub const DI: Register = Register {
        prefix: Some((0x66, Size::Byte1)),
        size: Size::Byte2,
        id: 7,
    };

    pub const R8W: Register = Register {
        prefix: Some((0x66 | 41 << 8, Size::Byte2)),
        size: Size::Byte2,
        id: 0,
    };
    pub const R9W: Register = Register {
        prefix: Some((0x66 | 41 << 8, Size::Byte2)),
        size: Size::Byte2,
        id: 1,
    };
    pub const R10W: Register = Register {
        prefix: Some((0x66 | 41 << 8, Size::Byte2)),
        size: Size::Byte2,
        id: 2,
    };
    pub const R11W: Register = Register {
        prefix: Some((0x66 | 41 << 8, Size::Byte2)),
        size: Size::Byte2,
        id: 3,
    };
    pub const R12W: Register = Register {
        prefix: Some((0x66 | 41 << 8, Size::Byte2)),
        size: Size::Byte2,
        id: 4,
    };
    pub const R13W: Register = Register {
        prefix: Some((0x66 | 41 << 8, Size::Byte2)),
        size: Size::Byte2,
        id: 5,
    };
    pub const R14W: Register = Register {
        prefix: Some((0x66 | 41 << 8, Size::Byte2)),
        size: Size::Byte2,
        id: 6,
    };
    pub const R15W: Register = Register {
        prefix: Some((0x66 | 41 << 8, Size::Byte2)),
        size: Size::Byte2,
        id: 7,
    };

    pub const AL: Register = Register {
        prefix: None,
        size: Size::Byte1,
        id: 0,
    };
    pub const CL: Register = Register {
        prefix: None,
        size: Size::Byte1,
        id: 1,
    };
    pub const DL: Register = Register {
        prefix: None,
        size: Size::Byte1,
        id: 2,
    };
    pub const BL: Register = Register {
        prefix: None,
        size: Size::Byte1,
        id: 3,
    };
    pub const SPL: Register = Register {
        prefix: None,
        size: Size::Byte1,
        id: 4,
    };
    pub const BPL: Register = Register {
        prefix: None,
        size: Size::Byte1,
        id: 5,
    };
    pub const SIL: Register = Register {
        prefix: None,
        size: Size::Byte1,
        id: 6,
    };
    pub const DIL: Register = Register {
        prefix: None,
        size: Size::Byte1,
        id: 7,
    };

    pub const R8B: Register = Register {
        prefix: Some((0x41, Size::Byte1)),
        size: Size::Byte2,
        id: 0,
    };
    pub const R9B: Register = Register {
        prefix: Some((0x41, Size::Byte1)),
        size: Size::Byte2,
        id: 1,
    };
    pub const R10B: Register = Register {
        prefix: Some((0x41, Size::Byte1)),
        size: Size::Byte2,
        id: 2,
    };
    pub const R11B: Register = Register {
        prefix: Some((0x41, Size::Byte1)),
        size: Size::Byte2,
        id: 3,
    };
    pub const R12B: Register = Register {
        prefix: Some((0x41, Size::Byte1)),
        size: Size::Byte2,
        id: 4,
    };
    pub const R13B: Register = Register {
        prefix: Some((0x41, Size::Byte1)),
        size: Size::Byte2,
        id: 5,
    };
    pub const R14B: Register = Register {
        prefix: Some((0x41, Size::Byte1)),
        size: Size::Byte2,
        id: 6,
    };
    pub const R15B: Register = Register {
        prefix: Some((0x41, Size::Byte1)),
        size: Size::Byte2,
        id: 7,
    };
}
