use crate::architecture::x64::instruction::{AssemblyInstruction, Instruction, Size};
use crate::builtin::Number;
use crate::memory::{ExecutableMemory, PAGE_SIZE};

pub struct Jitter {
    pub next: Vec<u8>,
}

impl Jitter {
    pub fn new() -> Self {
        Self {
            next: Vec::with_capacity(*PAGE_SIZE),
        }
    }

    pub fn compile(&mut self) -> ExecutableMemory {
        let mut mem = ExecutableMemory::new(self.next.len());
        unsafe { std::ptr::copy(self.next.as_ptr(), mem.as_mut_ptr(), self.next.len()) }
        mem
    }

    pub fn write(&mut self, instr: impl Instruction) {
        let instruction = instr.build();
        self.write_instruction(instruction);
    }

    pub fn write_instruction(&mut self, instruction: AssemblyInstruction) {
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
            self.write_sized(value as u64, size)
        }
        if let Some((value, size)) = prefix {
            self.write_sized(value as u64, size)
        }
        self.write_sized(opcode.0 as u64, opcode.1);
        if let Some(value) = modrm {
            self.next.push(value);
        }
        if let Some(value) = sib {
            self.next.push(value);
        }
        if let Some((value, size)) = displacement {
            self.write_sized(value, size)
        }
        if let Some((value, size)) = immediate {
            self.write_sized(value as u64, size)
        }
    }

    fn write_sized(&mut self, value: u64, size: Size) {
        let size = size as u8;
        if size >= 1 {
            self.next.push((value >> 0) as u8)
        }
        if size >= 2 {
            self.next.push((value >> 8) as u8)
        }
        if size >= 3 {
            self.next.push((value >> 16) as u8)
        }
        if size >= 4 {
            self.next.push((value >> 24) as u8)
        }
        if size >= 5 {
            self.next.push((value >> 32) as u8)
        }
        if size >= 6 {
            self.next.push((value >> 40) as u8)
        }
        if size >= 7 {
            self.next.push((value >> 48) as u8)
        }
        if size >= 8 {
            self.next.push((value >> 56) as u8)
        }
    }
}
