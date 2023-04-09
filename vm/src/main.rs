use vm::context::Context;
use vm::lexer::Lexer;

// const DEMO: &str = "5   10 + .\n\n \n\t\n2 2 3 * + .\n";
const DEMO: &str = "5 10 + drop\n";

fn main() {
    let mut context = Context::new();
    context.execute(DEMO).unwrap();
}
