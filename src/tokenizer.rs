use crate::token::{Token, TokenType};

pub fn tokenize(user_seperated: Vec<String>) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();
    let mut command_index = 0;
    // we need C style here
    for i in 0..user_seperated.len() {
        let section = user_seperated[i].clone();
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

pub fn process_input(user_input: &String) -> Vec<String> {
    // NOTE: Clean this abomination of parsing
    let user_input_raw = user_input.trim();
    let mut user_input_processed: Vec<String> = Vec::new();
    let mut buffer: String = String::new();
    let mut is_quoted = false;

    for c in user_input_raw.chars() {
        if c == ' ' {
            if is_quoted {
                buffer.push(c);
                continue;
            } else {
                let buffer_copy = buffer.clone();
                if !buffer.is_empty() {
                    user_input_processed.push(buffer_copy);
                }
                buffer.clear();
            }
        } else if c == '"' {
            if is_quoted {
                is_quoted = false;
                let buffer_copy = buffer.clone();
                user_input_processed.push(buffer_copy);
                buffer.clear();
            } else {
                is_quoted = true;
            }
        } else {
            buffer.push(c);
        }
    }
    let buffer_copy = buffer.clone();
    if !buffer.is_empty() {
        user_input_processed.push(buffer_copy);
    }
    return user_input_processed;
}
