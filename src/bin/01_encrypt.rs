use magic_crypt::{new_magic_crypt, MagicCryptTrait};
use std::io::{stdin, stdout, Write};

fn get_user_input() -> String {
    let mut s = String::new();
    let _ = stdout().flush();
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

fn main() {
    println!("=== Encrypted Serial Key Generator ===");

    println!("Please input 4-digits Customer ID: ");
    let customerid = Some(get_user_input());

    println!("Please input 8-digits Product ID: ");
    let productid = Some(get_user_input());

    let plain_serial = format!("{}{}", customerid.unwrap(), productid.unwrap());
    println!("Plain serial: {}", plain_serial); 

    let mc = new_magic_crypt!("magickey", 256);
    let serial = mc.encrypt_str_to_base64(&plain_serial); 
    println!("Encrypted serial: {}", serial);

    let dec = mc.decrypt_base64_to_string(serial).unwrap(); 
    println!("Decrypted serial: {}", dec);
    let verify_customerid = &dec[0..4];
    let verify_productid = &dec[4..12];
    println!("Verify Customer ID: {}", verify_customerid);
    println!("Verify Product ID: {}", verify_productid);
}

#[cfg(test)]
mod tests {
    use super::*;
    use magic_crypt::{new_magic_crypt, MagicCryptTrait};

    #[test]
    fn test_encryption_decryption() {
        let plain_text = "1234qwerasdf";
        let mc = new_magic_crypt!("testkey", 256);
        
        // 암호화
        let encrypted = mc.encrypt_str_to_base64(plain_text);
        assert_ne!(encrypted, plain_text); // 암호화된 결과는 원본과 달라야 함
        
        // 복호화
        let decrypted = mc.decrypt_base64_to_string(&encrypted).unwrap();
        assert_eq!(decrypted, plain_text); // 복호화된 결과는 원본과 같아야 함
    }

    #[test]
    fn test_encryption_with_different_keys() {
        let plain_text = "1234qwerasdf";
        let mc1 = new_magic_crypt!("key1", 256);
        let mc2 = new_magic_crypt!("key2", 256);
        
        // 서로 다른 키로 암호화
        let encrypted1 = mc1.encrypt_str_to_base64(plain_text);
        let encrypted2 = mc2.encrypt_str_to_base64(plain_text);
        
        // 같은 평문이라도 다른 키로 암호화하면 결과가 달라야 함
        assert_ne!(encrypted1, encrypted2);
        
        // 각각 올바른 키로 복호화
        let decrypted1 = mc1.decrypt_base64_to_string(&encrypted1).unwrap();
        let decrypted2 = mc2.decrypt_base64_to_string(&encrypted2).unwrap();
        
        assert_eq!(decrypted1, plain_text);
        assert_eq!(decrypted2, plain_text);
    }

    #[test]
    fn test_wrong_key_decryption() {
        let plain_text = "1234qwerasdf";
        let mc_correct = new_magic_crypt!("correctkey", 256);
        let mc_wrong = new_magic_crypt!("wrongkey", 256);
        
        // 올바른 키로 암호화
        let encrypted = mc_correct.encrypt_str_to_base64(plain_text);
        
        // 잘못된 키로 복호화 시도 (실패해야 함)
        let result = mc_wrong.decrypt_base64_to_string(&encrypted);
        assert!(result.is_err());
    }

    #[test]
    fn test_base64_encoding() {
        let mc = new_magic_crypt!("testkey", 256);
        let plain_text = "1234qwerasdf";
        
        let encrypted = mc.encrypt_str_to_base64(plain_text);
        
        // BASE64 인코딩 검증 (알파벳, 숫자, +, /, = 만 포함)
        let valid_chars = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/=";
        assert!(encrypted.chars().all(|c| valid_chars.contains(c)));
    }

    #[test]
    fn test_end_to_end_process() {
        let customer_id = "5678";
        let product_id = "asdfqwer";
        let plain_serial = format!("{}{}", customer_id, product_id);
        
        let mc = new_magic_crypt!("magickey", 256);
        let encrypted = mc.encrypt_str_to_base64(&plain_serial);
        let decrypted = mc.decrypt_base64_to_string(&encrypted).unwrap();
        
        // 전체 프로세스 검증
        assert_eq!(decrypted, plain_serial);
        
        // ID 추출 검증
        let extracted_customer_id = &decrypted[0..4];
        let extracted_product_id = &decrypted[4..12];
        
        assert_eq!(extracted_customer_id, customer_id);
        assert_eq!(extracted_product_id, product_id);
    }
}