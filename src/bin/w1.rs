use magic_crypt::{MagicCryptTrait, new_magic_crypt};
use serial_data_rs::io::get_input;

fn main() {
    println!("Please input 4-digits Customer ID: ");
    let customer_id = get_input(|s| {
        if s.len() != 4 || !s.chars().all(char::is_numeric) {
            return Err("Customer ID must be exactly 4 digits long.".into());
        }
        Ok(())
    })
    .expect("Failed to read Customer ID");

    println!("Please input 8-alphanumeric Product ID: ");
    let product_id = get_input(|s| {
        if s.len() != 8 || !s.chars().all(|c| c.is_alphanumeric()) {
            return Err("Product ID must be exactly 8 alphanumeric characters.".into());
        }
        Ok(())
    })
    .expect("Failed to read Product ID");

    let plain_serial = format!("{}{}", customer_id, product_id);
    println!("Plain serial: {}", plain_serial);

    let mc = new_magic_crypt!("magickey", 256); // AES256 알고리즘을 사용하는 MagicCrypt256타입의 객체 생성
    let serial = mc.encrypt_str_to_base64(&plain_serial); // 암호화 후 BASE64로 인코딩
    println!("Encrypted serial: {}", serial);

    let dec = mc.decrypt_base64_to_string(serial).unwrap(); // BASE64로 인코딩된 데이터를 디코딩 후 암호 해제
    println!("Decrypted serial: {}", dec);

    let verify_customer_id = &plain_serial[0..4];
    let verify_product_id = &plain_serial[4..12];
    println!("Verify Customer ID: {}", verify_customer_id);
    println!("Verify Product ID: {}", verify_product_id);
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_magic_crypt() {
        use magic_crypt::{MagicCryptTrait, new_magic_crypt};
        let mc = new_magic_crypt!("magickey", 256);
        let base64 = mc.encrypt_str_to_base64("http://magiclen.org");

        assert_eq!("DS/2U8royDnJDiNY2ps3f6ZoTbpZo8ZtUGYLGEjwLDQ=", base64);
        assert_eq!(
            "http://magiclen.org",
            mc.decrypt_base64_to_string(&base64).unwrap()
        );
    }
}
