use std::io::{self, Write, stdout};

use crate::{
    execute::process_tokens,
    tokenizer::{process_input, tokenize},
};

pub fn repl_loop() {
    loop {
        print!("$ ");
        stdout().flush();
        let mut user_input = String::new();
        io::stdin().read_line(&mut user_input).unwrap();
        let user_input_processed = process_input(&user_input);
        let tokens = tokenize(user_input_processed);
        process_tokens(&tokens);
    }
}
