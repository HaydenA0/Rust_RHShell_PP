use core::fmt;
use std::io;

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

pub fn repl_loop() {
    eprint!("RH\n$ ");
    let mut user_input = String::new();
    io::stdin()
        .read_line(&mut user_input)
        .expect("Error Reading");

    let user_input_processed = process_input(&user_input);
    let tokens = tokenize(&user_input_processed);
    for token in tokens {
        print!(
            "TokenType : {}, TokenString : {}\n",
            token.token_type, token.token_string
        );
    }
}

fn tokenize(user_input: &String) -> Vec<Token> {
    let user_seperated: Vec<&str> = user_input.split_whitespace().collect::<Vec<&str>>();
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

fn process_input(user_input: &String) -> String {
    let user_input_processed = user_input.trim(); // TODO: merge the in " " -> "ab de" becomes "abde"
    // change of type ahead
    let user_input_processed = user_input_processed
        .split_whitespace()
        .collect::<Vec<&str>>()
        .join(" ");
    return user_input_processed;
}
