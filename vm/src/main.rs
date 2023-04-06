use vm::architecture::x64::instruction::{encode_modrm, Instruction, MovInstruction, Operand, Register, RetInstruction, Size};

use vm::context::Context;
use vm::jit::Jitter;
use vm::lexer::Lexer;
use vm::memory::{ExecutableMemory, PAGE_SIZE};

const DEMO: &str = "5   10 + .\n\n \n\t\n2 2 3 * + .\n";

fn main() {
    // let mut context = Context::new();
    // context.execute_str(DEMO).unwrap();

    let mut jit = Jitter::new();

    let instr1 = MovInstruction {
        src: Operand::Constant((666, Size::Byte4)),
        dst: Operand::Register(Register::RAX),
    };
    let instr2 = RetInstruction {};

    jit.write(instr1);
    jit.write(instr2);
    let exec_raw = jit.compile();
    let exec = exec_raw.to_fn();

    let value = exec(0);
    println!("value: {:?}", value);

    // 48C7C09A020000

    // let mut mem = ExecutableMemory::new(4000);
    // mem.push(0x48);
    // mem.push(0xC7);
    // mem.push(0xC0);
    // mem.push(0x9A);
    // mem.push(0x02);
    // mem.push(0x00);
    // mem.push(0x00);
    // mem.push(0xc3);
    // let exec = mem.to_fn();
    // let x = exec();
    // println!("Hello, world!, {:?}", x);
}
