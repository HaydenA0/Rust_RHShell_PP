use core::fmt;
use std::{
    io::{self},
    process::Command,
};

pub fn repl_loop() {
    loop {
        eprint!("RH\n$ ");
        let mut user_input = String::new();
        io::stdin()
            .read_line(&mut user_input)
            .expect("Error Reading");

        let user_input_processed = process_input(&user_input);
        let tokens = tokenize(user_input_processed);
        // print_tokens(&tokens); // NOTE : Just for debuggin, remove later
        process_tokens(&tokens);
    }
}
enum TokenType {
    Command,  // i.e grep, fzf, echo
    Argument, // flags and args
    Keyword,  // should be for |, &, >, ...
}
impl fmt::Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let diplay_str = match self {
            TokenType::Command => "Command",
            TokenType::Argument => "Argument",
            TokenType::Keyword => "Keyword",
        };
        write!(f, "{}", diplay_str)
    }
}

struct Token {
    token_type: TokenType,
    token_string: String,
}

// fn print_tokens(tokens: &Vec<Token>) {
//     // NOTE : Used for debuggin but the complier wont shut about
//     // it.
//     for token in tokens {
//         print!(
//             "TokenType : {}, TokenString : {}\n",
//             token.token_type, token.token_string
//         );
//     }
// }

fn process_tokens(tokens: &Vec<Token>) {
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

fn tokenize(user_input: Vec<&str>) -> Vec<Token> {
    let user_seperated: Vec<&str> = user_input;
    let mut tokens: Vec<Token> = Vec::new();
    let mut command_index = 0;
    // we need C style here
    for i in 0..user_seperated.len() {
        let section = user_seperated[i];
        if i == command_index {
            let token = Token {
                token_type: TokenType::Command,
                token_string: section.to_string(),
            };
            tokens.push(token);
        } else if section == "|" || section == "&" || section == ";" || section == ">" {
            command_index = i + 1;
            if i == user_seperated.len() - 1 {
                panic!("Do not put a Keyword in the end of the your command");
            }
            let token = Token {
                token_type: TokenType::Keyword,
                token_string: section.to_string(),
            };
            tokens.push(token);
        } else {
            let token = Token {
                token_type: TokenType::Argument,
                token_string: section.to_string(),
            };
            tokens.push(token);
        }
    }
    return tokens;
}

fn process_input(user_input: &String) -> Vec<&str> {
    let user_input_processed = user_input.trim();
    // TODO: merge the in " " -> "ab de" becomes "abde"
    let user_input_processed = user_input_processed
        .split_whitespace()
        .collect::<Vec<&str>>();
    return user_input_processed;
}
