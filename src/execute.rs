use std::{error::Error, process::Command};

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
    let child_result = Command::new(command).args(args).spawn();
    let mut child = match child_result {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Failed to spawn : {e}\n");
            return;
        }
    };
    let _ = child.wait().expect("Failed to wait on child");
}
