use std::io::{stdin, stdout, Write};
// 사용자 입력
fn get_user_input(prompt: &str , expected_len :usize) -> String {
    loop{
        print!("{}", prompt);
        let _ =stdout().flush();
        let mut s = String::new();
        stdin().read_line(&mut s).expect("입력 오류");
        s = s.trim().to_string();
        if s.len() == expected_len {
            return s;
        } else {
            println!("입력 길이가 {}자여야 합니다. 다시 시도하세요.", expected_len);
        }
    }

}

//시리얼 생성 함수 
fn generate_serial(customerid : &str , productid : &str)-> String{
    format!("{}{}" ,customerid , productid)
}

fn main() {
    
    let customerid = get_user_input("please input 4-digits Customer ID", 4);
    let productid = get_user_input("please input 8-digits Product ID", 8);

    let plain_serial = generate_serial(&customerid, &productid);
    println!("Plain serial: {}", plain_serial); // 암호화 전 시리얼 출력

    println!("Verify Customer ID: {}", &plain_serial[0..4]);
    println!("Verify Product ID: {}", &plain_serial[4..12]);
}
