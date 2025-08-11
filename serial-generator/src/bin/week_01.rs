use magic_crypt::{MagicCryptTrait, new_magic_crypt};
use std::io::{Write, stdin, stdout};

// 사용자 입력
fn get_user_input(prompt: &str, expected_len: usize) -> String {
    loop {
        print!("{}", prompt);
        let _ = stdout().flush();
        let mut s = String::new();
        stdin().read_line(&mut s).expect("입력 오류");
        s = s.trim().to_string();
        if s.len() == expected_len {
            return s;
        } else {
            println!(
                "입력 길이가 {}자여야 합니다. 다시 시도하세요.",
                expected_len
            );
        }
    }
}

//시리얼 생성 함수
fn generate_serial(customerid: &str, productid: &str) -> String {
    format!("{}{}", customerid, productid)
}

fn main() {
    let customerid = get_user_input("please input 4-digits Customer ID", 4);
    let productid = get_user_input("please input 8-digits Product ID", 8);

    let plain_serial = generate_serial(&customerid, &productid);
    println!("Plain serial: {}", plain_serial); // 암호화 전 시리얼 출력

    let mc = new_magic_crypt!("magickey", 256); // AES256 알고리즘을 사용하는 MagicCrypt256타입의 객체 생성
    let serial = mc.encrypt_str_to_base64(&plain_serial); // 암호화 후 BASE64로 인코딩

    println!("Encrypted serial: {}", serial);

    let dec = mc.decrypt_base64_to_string(serial).unwrap(); // BASE64로 인코딩된 데이터를 디코딩 후 암호 해제

    println!("Decrypted serial: {}", dec);
    println!("Verify Customer ID: {}", &dec[0..4]);
    println!("Verify Product ID: {}", &dec[4..12]);
}

#[cfg(test)] // 테스트 
mod tests {
    use super::*;
    use magic_crypt::{MagicCryptTrait, new_magic_crypt};

    #[test]
    fn test_encrypt_decrypt() {
        let mc = new_magic_crypt!("magickey", 256);
        let original = "1234qwertyaa";
        let encrypted = mc.encrypt_str_to_base64(&original);
        let decrypted = mc.decrypt_base64_to_string(encrypted).unwrap();
        assert_eq!(original, decrypted);
    }
}
