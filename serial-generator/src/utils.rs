use std::io::{stdin, stdout, Write};

pub fn get_user_input() ->String{
    let mut input  = String::new();
    stdout().flush().unwrap();
    stdin().read_line(&mut input).expect("입력 오류");
    input.trim().to_string()
}