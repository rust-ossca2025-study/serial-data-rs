use std::io::{Write, stdin, stdout};

pub fn get_user_input(name: &str, expected_len: usize) -> String {
    loop {
        print!("Please input {expected_len}-digits {name}: ");
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

        match validate_input(&s, expected_len, name) {
            Ok(_) => return s,
            Err(e) => {
                eprintln!("Error: {e}");
                println!("Please try again.");
            }
        }
    }
}

fn validate_input(input: &str, expected_len: usize, name: &str) -> Result<(), String> {
    if input.len() != expected_len {
        return Err(format!("{name} must be {expected_len} digits"));
    }
    Ok(())
}
