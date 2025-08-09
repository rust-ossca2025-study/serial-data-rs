use std::{
    error::Error,
    io::{Write, stdin, stdout},
};

pub fn get_user_input(name: &str, expected_len: usize) -> String {
    loop {
        let input = get_input();
        match validate_input(&input, expected_len, name) {
            Ok(_) => return input,
            Err(e) => {
                eprintln!("Error: {e}");
            }
        }
    }
}

type Validate = fn(&str) -> Result<String, Box<dyn Error>>;

pub fn get_user_input_custom(validate_fn: Validate) -> String {
    loop {
        let input = get_input();
        match validate_fn(&input) {
            Ok(_) => return input,
            Err(e) => {
                eprintln!("Error: {e}");
            }
        }
    }
}

fn get_input() -> String {
    let _ = stdout().flush();
    let mut s = String::new();

    stdin()
        .read_line(&mut s)
        .expect("Did not enter a correct string");
    if let Some('\n') = s.chars().next_back() {
        s.pop();
    }
    if let Some('\r') = s.chars().next_back() {
        s.pop();
    }
    s
}

fn validate_input(input: &str, expected_len: usize, name: &str) -> Result<(), String> {
    if input.len() != expected_len {
        return Err(format!("{name} must be {expected_len} digits"));
    }
    if !input.chars().all(|c| c.is_alphanumeric()) {
        return Err(format!("{name} must be alphabet or number"));
    }
    Ok(())
}
