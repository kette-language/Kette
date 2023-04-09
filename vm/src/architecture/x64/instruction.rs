use crate::architecture::x64::opcode::Opcode;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(u8)]
pub enum Size {
    Byte1 = 1,
    Byte2 = 2,
    Byte3 = 3,
    Byte4 = 4,
    Byte8 = 8,
}

impl Size {
    pub fn to_string(&self) -> String {
        match self {
            Size::Byte1 => "byte",
            Size::Byte2 => "word",
            Size::Byte3 => "Byte3",
            Size::Byte4 => "dword",
            Size::Byte8 => "qword",
        }.to_owned()
    }
}

pub trait Instruction {
    fn build(&self) -> AssemblyInstruction;
    fn to_string(&self) -> String;
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
    pub src: Operand,
    pub dst: Operand,
}

pub struct SubInstruction {
    pub src: Operand,
    pub dst: Operand,
}

pub struct PushInstruction {
    pub op: Operand,
}

pub struct PopInstruction {
    pub op: Operand,
}

pub struct RetInstruction;

impl Instruction for MovInstruction {
    fn build(&self) -> AssemblyInstruction {
        let mut instruction = AssemblyInstruction::default();

        match &self.dst {
            Operand::Register(dst) => {
                let prefixes = dst.prefixes();
                match &self.src {
                    Operand::Register(src) => {
                        instruction.modrm = Some(Register::encode_modrm(11, src.id, dst.id));
                        instruction.opcode = (Opcode::MOV_REG_REG, Size::Byte1);
                    }
                    Operand::RegisterAddress(_) => {
                        todo!()
                    }
                    Operand::Address(_) => {
                        todo!()
                    }
                    Operand::Constant((value, size)) => {
                        instruction.immediate = Some((*value, *size));
                        if !(size == &Size::Byte4 && dst.size == Size::Byte8) && size != &dst.size {
                            panic!("sizes do not match")
                        }
                        match dst.size {
                            Size::Byte1 => {
                                instruction.prefix = prefixes[0];
                                instruction.opcode =
                                    (Opcode::MOV_REG_CONST_B1 | dst.id as u32, Size::Byte1)
                            }
                            Size::Byte2 => {
                                instruction.prefix = prefixes[1];
                                instruction.opcode =
                                    (Opcode::MOV_REG_CONST_B248 | dst.id as u32, Size::Byte1)
                            }
                            Size::Byte4 => {
                                instruction.prefix = prefixes[2];
                                instruction.opcode =
                                    (Opcode::MOV_REG_CONST_B248 | dst.id as u32, Size::Byte1);
                            }
                            Size::Byte8 => {
                                instruction.prefix = prefixes[3];
                                if size == &Size::Byte8 {
                                    instruction.opcode =
                                        (Opcode::MOV_REG_CONST_B248 | dst.id as u32, Size::Byte1);
                                } else {
                                    instruction.opcode = (Opcode::MOV_REG_CONST_B8, Size::Byte1);
                                    instruction.modrm = Some(Register::encode_modrm(3, 0, dst.id));
                                }
                            }
                            _ => panic!("Invalid Size"),
                        }
                    }
                }
            }
            Operand::RegisterAddress(_) => {
                todo!()
            }
            Operand::Address(_) => {
                todo!()
            }
            Operand::Constant(_) => {
                panic!("Destination cannot be constant")
            }
        };

        instruction
    }

    fn to_string(&self) -> String {
        let mut result = String::from("mov ");
        match &self.dst {
            Operand::Register(r) => {
                result.push_str(&r.to_string())
            }
            Operand::RegisterAddress(_) => {},
            Operand::Address(_) => {},
            Operand::Constant(_) => panic!("Destination cannot be constant")
        }
        result.push_str(", ");
        match &self.src {
            Operand::Register(r) => {
                result.push_str(&r.to_string())
            }
            Operand::RegisterAddress(_) => {},
            Operand::Address(_) => {},
            Operand::Constant(_) => panic!("Destination cannot be constant")
        }
        result
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

    fn to_string(&self) -> String {
        String::from("ret")
    }
}

impl Instruction for PushInstruction {
    fn build(&self) -> AssemblyInstruction {
        let mut instruction = AssemblyInstruction::default();
        match &self.op {
            Operand::Register(r) => {
                match r.sub_id {
                    0 => instruction.prefix = None,
                    1 => instruction.prefix = r.prefixes()[3],
                    _ => {
                        panic!("Invalid SubId")
                    }
                }
                instruction.opcode = (Opcode::PUSH_REG | r.id as u32, Size::Byte1);
            }
            Operand::Address(_) => {
                todo!()
            }
            Operand::RegisterAddress(_) => {
                todo!()
            }
            Operand::Constant((val, size)) => match size {
                Size::Byte1 => {
                    instruction.opcode = (Opcode::PUSH_CONST_B1, Size::Byte1);
                    instruction.immediate = Some((*val, Size::Byte1));
                }
                Size::Byte2 | Size::Byte3 | Size::Byte4 => {
                    instruction.opcode = (Opcode::PUSH_CONST_B4, Size::Byte1);
                    instruction.immediate = Some((*val, Size::Byte4));
                }
                Size::Byte8 => {
                    panic!("invalid size")
                }
            },
        };
        instruction
    }

    fn to_string(&self) -> String {
        let mut result = String::from("push ");
        match &self.op {
            Operand::Register(r) => {
                result.push_str(&r.to_string());
            }
            Operand::RegisterAddress(_) => {},
            Operand::Address(_) => {},
            Operand::Constant((val, size)) => {
                result.push_str(&val.to_string());
                result.push_str(" ; ");
                result.push_str(&size.to_string());
            }
        }
        result
    }
}

impl Instruction for PopInstruction {
    fn build(&self) -> AssemblyInstruction {
        let mut instruction = AssemblyInstruction::default();
        match &self.op {
            Operand::Register(r) => {
                match r.sub_id {
                    0 => instruction.prefix = None,
                    1 => instruction.prefix = r.prefixes()[3],
                    _ => {
                        panic!("Invalid SubId")
                    }
                }
                instruction.opcode = (Opcode::POP_REG | r.id as u32, Size::Byte1);
            }
            Operand::Address(_) => {
                todo!()
            }
            Operand::RegisterAddress(_) => {
                todo!()
            }
            Operand::Constant(_) => panic!("Pop does not support constant"),
        };
        instruction
    }

    fn to_string(&self) -> String {
        let mut result = String::from("pop ");
        match &self.op {
            Operand::Register(r) => {
                result.push_str(&r.to_string());
            }
            Operand::RegisterAddress(_) => {},
            Operand::Address(_) => {},
            Operand::Constant(_) => panic!("Pop does not support constant"),
        }
        result
    }
}

impl Instruction for AddInstruction {
    fn build(&self) -> AssemblyInstruction {
        let mut instruction = AssemblyInstruction::default();
        match &self.dst {
            Operand::Register(dst) => {
                let prefixes = dst.prefixes();
                match &self.src {
                    Operand::Register(src) => {
                        if dst.size != src.size {
                            panic!("sizes do not match");
                        }
                        match dst.size {
                            Size::Byte1 => {}
                            Size::Byte2 => {}
                            Size::Byte4 => {}
                            Size::Byte8 => {
                                instruction.opcode = (Opcode::ADD_REG_REG_B8, Size::Byte1);
                                match dst.sub_id {
                                    0 => match src.sub_id {
                                        0 => instruction.prefix = Some((0x48, Size::Byte1)),
                                        1 => instruction.prefix = Some((0x4c, Size::Byte1)),
                                        _ => panic!("invalid SubId"),
                                    }
                                    1 => match src.sub_id {
                                        0 => instruction.prefix = Some((0x49, Size::Byte1)),
                                        1 => instruction.prefix = Some((0x4d, Size::Byte1)),
                                        _ => panic!("invalid SubId"),
                                    }
                                    _ => panic!("invalid SubId"),
                                }
                                instruction.modrm = Some(Register::encode_modrm(11, src.id, dst.id));
                            }
                            _ => panic!("invalid size"),
                        }
                    }
                    Operand::RegisterAddress(_) => {
                        todo!()
                    }
                    Operand::Address(_) => {
                        todo!()
                    }
                    Operand::Constant(_) => {
                        todo!()
                    }
                }
            }
            Operand::RegisterAddress(_) => {
                todo!()
            }
            Operand::Address(_) => {
                todo!()
            }
            Operand::Constant(_) => {
                panic!("Destination cannot be constant")
            }
        };
        instruction
    }

    fn to_string(&self) -> String {
        let mut result = String::from("add ");
        match &self.dst {
            Operand::Register(r) => {
                result.push_str(&r.to_string())
            }
            Operand::RegisterAddress(_) => {},
            Operand::Address(_) => {},
            Operand::Constant(_) => panic!("Destination cannot be constant")
        }
        result.push_str(", ");
        match &self.src {
            Operand::Register(r) => {
                result.push_str(&r.to_string())
            }
            Operand::RegisterAddress(_) => {},
            Operand::Address(_) => {},
            Operand::Constant(_) => panic!("Destination cannot be constant")
        }
        result
    }
}

pub enum Operand {
    Register(Register),
    RegisterAddress((Register, i32, Size)),
    Address((u64, i32, Size)),
    Constant((u64, Size)),
}

pub struct Register {
    pub id: u8,
    pub sub_id: u8,
    // rax -> 0, r8 -> 1
    pub size: Size,
}

impl Register {
    pub const fn new(id: u8, sub_id: u8, size: Size) -> Self {
        Self { id, sub_id, size }
    }

    pub const RAX: Self = Self::new(0, 0, Size::Byte8);
    pub const RCX: Self = Self::new(1, 0, Size::Byte8);
    pub const RDX: Self = Self::new(2, 0, Size::Byte8);
    pub const RBX: Self = Self::new(3, 0, Size::Byte8);
    pub const RSP: Self = Self::new(4, 0, Size::Byte8);
    pub const RBP: Self = Self::new(5, 0, Size::Byte8);
    pub const RSI: Self = Self::new(6, 0, Size::Byte8);
    pub const RDI: Self = Self::new(7, 0, Size::Byte8);

    pub const R8: Self = Self::new(0, 1, Size::Byte8);
    pub const R9: Self = Self::new(1, 1, Size::Byte8);
    pub const R10: Self = Self::new(2, 1, Size::Byte8);
    pub const R11: Self = Self::new(3, 1, Size::Byte8);
    pub const R12: Self = Self::new(4, 1, Size::Byte8);
    pub const R13: Self = Self::new(5, 1, Size::Byte8);
    pub const R14: Self = Self::new(6, 1, Size::Byte8);
    pub const R15: Self = Self::new(7, 1, Size::Byte8);

    pub const EAX: Self = Self::new(0, 0, Size::Byte4);
    pub const ECX: Self = Self::new(1, 0, Size::Byte4);
    pub const EDX: Self = Self::new(2, 0, Size::Byte4);
    pub const EBX: Self = Self::new(3, 0, Size::Byte4);
    pub const ESP: Self = Self::new(4, 0, Size::Byte4);
    pub const EBP: Self = Self::new(5, 0, Size::Byte4);
    pub const ESI: Self = Self::new(6, 0, Size::Byte4);
    pub const EDI: Self = Self::new(7, 0, Size::Byte4);

    pub const R8D: Self = Self::new(0, 1, Size::Byte4);
    pub const R9D: Self = Self::new(1, 1, Size::Byte4);
    pub const R10D: Self = Self::new(2, 1, Size::Byte4);
    pub const R11D: Self = Self::new(3, 1, Size::Byte4);
    pub const R12D: Self = Self::new(4, 1, Size::Byte4);
    pub const R13D: Self = Self::new(5, 1, Size::Byte4);
    pub const R14D: Self = Self::new(6, 1, Size::Byte4);
    pub const R15D: Self = Self::new(7, 1, Size::Byte4);

    pub const AX: Self = Self::new(0, 0, Size::Byte2);
    pub const CX: Self = Self::new(1, 0, Size::Byte2);
    pub const DX: Self = Self::new(2, 0, Size::Byte2);
    pub const BX: Self = Self::new(3, 0, Size::Byte2);
    pub const SP: Self = Self::new(4, 0, Size::Byte2);
    pub const BP: Self = Self::new(5, 0, Size::Byte2);
    pub const SI: Self = Self::new(6, 0, Size::Byte2);
    pub const DI: Self = Self::new(7, 0, Size::Byte2);

    pub const R8W: Self = Self::new(0, 1, Size::Byte2);
    pub const R9W: Self = Self::new(1, 1, Size::Byte2);
    pub const R10W: Self = Self::new(2, 1, Size::Byte2);
    pub const R11W: Self = Self::new(3, 1, Size::Byte2);
    pub const R12W: Self = Self::new(4, 1, Size::Byte2);
    pub const R13W: Self = Self::new(5, 1, Size::Byte2);
    pub const R14W: Self = Self::new(6, 1, Size::Byte2);
    pub const R15W: Self = Self::new(7, 1, Size::Byte2);

    pub const AL: Self = Self::new(0, 0, Size::Byte1);
    pub const CL: Self = Self::new(1, 0, Size::Byte1);
    pub const DL: Self = Self::new(2, 0, Size::Byte1);
    pub const BL: Self = Self::new(3, 0, Size::Byte1);
    pub const SPL: Self = Self::new(4, 0, Size::Byte1);
    pub const BPL: Self = Self::new(5, 0, Size::Byte1);
    pub const SIL: Self = Self::new(6, 0, Size::Byte1);
    pub const DIL: Self = Self::new(7, 0, Size::Byte1);

    pub const R8B: Self = Self::new(0, 1, Size::Byte1);
    pub const R9B: Self = Self::new(1, 1, Size::Byte1);
    pub const R10B: Self = Self::new(2, 1, Size::Byte1);
    pub const R11B: Self = Self::new(3, 1, Size::Byte1);
    pub const R12B: Self = Self::new(4, 1, Size::Byte1);
    pub const R13B: Self = Self::new(5, 1, Size::Byte1);
    pub const R14B: Self = Self::new(6, 1, Size::Byte1);
    pub const R15B: Self = Self::new(7, 1, Size::Byte1);

    pub fn prefixes(&self) -> [Option<(u32, Size)>; 4] {
        match self.sub_id {
            0 => [
                None,
                Some((0x66, Size::Byte1)),
                None,
                Some((0x48, Size::Byte1)),
            ],
            1 => [
                Some((0x41, Size::Byte1)),
                Some((0x66 | 41 << 8, Size::Byte2)),
                Some((0x41, Size::Byte1)),
                Some((0x49, Size::Byte1)),
            ],
            _ => {
                panic!("Invalid SubId")
            }
        }
    }

    pub fn encode_modrm(m: u8, reg: u8, rm: u8) -> u8 {
        m << 6 | reg << 3 | rm
    }

    pub fn to_string(&self) -> String {
        match self.size {
            Size::Byte8 => match self.sub_id {
                0 => match self.id {
                    0 => "RAX",
                    1 => "RCX",
                    2 => "RDX",
                    3 => "RBX",
                    4 => "RSP",
                    5 => "RBP",
                    6 => "RSI",
                    7 => "RDI",
                    _ => panic!("Invalid Id"),
                }
                1 => match self.id {
                    0 => "R8",
                    1 => "R9",
                    2 => "R10",
                    3 => "R11",
                    4 => "R12",
                    5 => "R13",
                    6 => "R14",
                    7 => "R15",
                    _ => panic!("Invalid Id"),
                }
                _ => panic!("Invalid SubId"),
            }
            Size::Byte4 => match self.sub_id {
                0 => match self.id {
                    0 => "EAX",
                    1 => "ECX",
                    2 => "EDX",
                    3 => "EBX",
                    4 => "ESP",
                    5 => "EBP",
                    6 => "ESI",
                    7 => "EDI",
                    _ => panic!("Invalid Id"),
                }
                1 => match self.id {
                    0 => "R8D",
                    1 => "R9D",
                    2 => "R10D",
                    3 => "R11D",
                    4 => "R12D",
                    5 => "R13D",
                    6 => "R14D",
                    7 => "R15D",
                    _ => panic!("Invalid Id"),
                }
                _ => panic!("Invalid SubId"),
            }
            Size::Byte2 => match self.sub_id {
                0 => match self.id {
                    0 => "AX",
                    1 => "CX",
                    2 => "DX",
                    3 => "BX",
                    4 => "SP",
                    5 => "BP",
                    6 => "SI",
                    7 => "DI",
                    _ => panic!("Invalid Id"),
                }
                1 => match self.id {
                    0 => "R8W",
                    1 => "R9W",
                    2 => "R10W",
                    3 => "R11W",
                    4 => "R12W",
                    5 => "R13W",
                    6 => "R14W",
                    7 => "R15W",
                    _ => panic!("Invalid Id"),
                }
                _ => panic!("Invalid SubId"),
            }
            Size::Byte1 => match self.sub_id {
                0 => match self.id {
                    0 => "AL",
                    1 => "CL",
                    2 => "DL",
                    3 => "BL",
                    4 => "SPL",
                    5 => "BPL",
                    6 => "SIL",
                    7 => "DIL",
                    _ => panic!("Invalid Id"),
                }
                1 => match self.id {
                    0 => "R8B",
                    1 => "R9B",
                    2 => "R10B",
                    3 => "R11B",
                    4 => "R12B",
                    5 => "R13B",
                    6 => "R14B",
                    7 => "R15B",
                    _ => panic!("Invalid Id"),
                }
                _ => panic!("Invalid SubId"),
            }
            _ => panic!("invalid Size")
        }.to_string()
    }
}