use std::io::Read;
use std::process::{ChildStdout, Command, Stdio};

// this part is a mess
// NOTE : this is so much better to handle piplines
//
// pub struct Pipeline {
//     pub commands: Vec<SimpleCommand>,
//     pub run_in_background: bool,
// }
//
// pub struct SimpleCommand {
//     pub program: String,
//     pub args: Vec<String>,
//     pub stdin_file: Option<String>,  // for <
//     pub stdout_file: Option<String>, // for >
// }

use crate::globals::{RED, RESET};
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
                let arg_string = token.token_string.clone();
                args.push(String::from(arg_string));
            }
            TokenType::Keyword => match token.token_string.as_str() {
                "|" => {
                    let mut first_output_child =
                        execute_command(&command, &args).expect("No stdout otuput");
                    let mut first_output = String::new();
                    first_output_child
                        .read_to_string(&mut first_output)
                        .expect("Could not read from the stdout");
                    command.clear();
                    args.clear();
                }
                _ => {}
            },
        }
    }
    execute_command(&command, &args);
}

fn execute_command(command: &String, args: &Vec<String>) -> Option<ChildStdout> {
    let child_result = Command::new(command)
        .args(args)
        .stdout(Stdio::piped())
        .spawn();
    let mut child = match child_result {
        Ok(c) => c,
        Err(_) => {
            eprint!("{RED} No such command {RESET}\n");
            return None;
        }
    };
    let _ = child.wait().expect("Failed to wait on child");
    let child_output_result = child.stdout;
    let child_output = match child_output_result {
        Some(c) => c,
        None => return None, // to rewrite
    };
    return Some(child_output);
}
