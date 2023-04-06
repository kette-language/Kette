use vm::lexer::Lexer;
use vm::memory::{ExecutableMemory, PAGE_SIZE};

const DEMO: &str = "5   10 + .\n\n \n\t\n2 2 3 * + .\n";

fn main() {
    let mut lexer = Lexer::new(DEMO);
    while let Some(word) = lexer.next_word() {
        println!("{:?}", word);
    }


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