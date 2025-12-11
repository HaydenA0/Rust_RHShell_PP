use std::process::Command;

use crate::token::{Token, TokenType};

pub fn process_tokens(tokens: &Vec<Token>) {
    // BUG : ls -aih | will print out an Error that | is invalid
    let mut command: String = String::new();
    let mut args: Vec<String> = Vec::new();
    for token in tokens {
        match token.token_type {
            TokenType::Command => {
                command = token.token_string.to_string();
            }
            TokenType::Argument => {
                let arg_string = token.token_string.as_str();
                args.push(String::from(arg_string));
            }
            TokenType::Keyword => {
                // command = String::new();
                // args = args + token.token_string.as_str();
            }
        }
    }
    execute_command(&command, &args);
}

fn execute_command(command: &String, args: &Vec<String>) {
    let mut child = Command::new(command)
        .args(args)
        .spawn()
        .expect("Failed to start command");
    let _ = child.wait().expect("Failed to wait on child");
}
