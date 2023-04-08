use vm::architecture::x64::instruction::{
    Instruction, MovInstruction, Operand, PopInstruction, PushInstruction, Register,
    RetInstruction, Size,
};

use vm::context::Context;
use vm::jit::Jitter;
use vm::lexer::Lexer;
use vm::memory::{ExecutableMemory, PAGE_SIZE};

// const DEMO: &str = "5   10 + .\n\n \n\t\n2 2 3 * + .\n";
const DEMO: &str = "5 10 + drop\n";

fn main() {
    let mut context = Context::new();
    context.execute(DEMO).unwrap();
}
