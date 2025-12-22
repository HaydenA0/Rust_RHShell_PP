mod execute;
mod globals;
mod repl;
mod token;
mod tokenizer;

fn main() {
    repl::repl_loop();
}
