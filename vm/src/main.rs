use vm::memory::{ExecutableMemory, PAGE_SIZE};

fn main() {
    let mut mem = ExecutableMemory::new(PAGE_SIZE);
    mem.push(0x48);
    mem.push(0xC7);
    mem.push(0xC0);
    mem.push(0x9A);
    mem.push(0x02);
    mem.push(0x00);
    mem.push(0x00);
    mem.push(0xc3);
    let exec = mem.to_fn();
    let x = exec();
    println!("Hello, world!, {:?}", x, );
}