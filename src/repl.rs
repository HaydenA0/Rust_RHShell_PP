use std::io;

pub fn repl_loop() {
    eprint!("RH\n$ ");
    let mut user_input = String::new();
    io::stdin()
        .read_line(&mut user_input)
        .expect("Error Reading");

    let user_input_processed = process_input(&user_input);
}

fn process_input(user_input: &String) -> String {
    let user_input_processed = user_input.trim();
    // change of type ahead
    let user_input_processed = user_input_processed
        .split_whitespace()
        .collect::<Vec<&str>>()
        .join(" ");
    return user_input_processed;
}
