use std::io::{self};

use crate::{
    execute::process_tokens,
    tokenizer::{process_input, tokenize},
};

pub fn repl_loop() {
    loop {
        eprint!("RH\n $ ");
        let mut user_input = String::new();
        io::stdin()
            .read_line(&mut user_input)
            .expect("Error Reading");

        let user_input_processed = process_input(&user_input);
        let tokens = tokenize(user_input_processed);
        process_tokens(&tokens);
    }
}

// fn print_tokens(tokens: &Vec<Token>) {
//     for token in tokens {
//         print!(
//             "TokenType : {}, TokenString : {}\n",
//             token.token_type, token.token_string
//         );
//     }
// }
// print_tokens(&tokens);
