use crate::token::{Token, TokenType};

pub fn tokenize(user_input: Vec<&str>) -> Vec<Token> {
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

pub fn process_input(user_input: &String) -> Vec<&str> {
    let user_input_processed = user_input.trim();
    // TODO: merge the in " " -> "ab de" becomes "abde"
    let user_input_processed = user_input_processed
        .split_whitespace()
        .collect::<Vec<&str>>();
    return user_input_processed;
}
