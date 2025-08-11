use magic_crypt::{new_magic_crypt, MagicCryptTrait};
use std::io::{stdin, stdout, Write};

fn get_user_input() -> String {
    let mut s = String::new();
    let _ = stdout().flush();
    stdin().read_line(&mut s).expect("input error");
    if let Some('\n') = s.chars().next_back() { s.pop(); }
    if let Some('\r') = s.chars().next_back() { s.pop(); }
    s
}

struct SerialData {
    customer_id: String,
    product_id: String,
    user_len: usize,
    product_len: usize,
}

fn make_serial_data(user_len: usize, product_len: usize) -> SerialData {
    SerialData {
        customer_id: String::new(),
        product_id: String::new(),
        user_len,
        product_len,
    }
}

fn prompt_fixed(label: &str, len: usize) -> String {
    loop {
        println!("Please input {}-digits for {}:", len, label);
        let s = get_user_input();
        if s.len() == len {
            return s;
        }
        println!("Invalid length: expected {}, got {}", len, s.len());
    }
}

fn prompt_inputs(data: &mut SerialData) {
    data.customer_id = prompt_fixed("UserID", data.user_len);
    data.product_id = prompt_fixed("ProductID", data.product_len);
}


fn plain_serial(data: &SerialData) -> String {
    let mut out = String::with_capacity(data.user_len + data.product_len);
    out.push_str(&data.customer_id);
    out.push_str(&data.product_id);
    out
}

fn verify_decrypted(data: &SerialData, decrypted: &str) -> (bool, bool) {
    let u = data.user_len;
    let p = data.product_len;

    if decrypted.len() != u + p {
        return (false, false);
    }
    let seg_user = &decrypted[0..u];
    let seg_prod = &decrypted[u..u + p];

    (seg_user == data.customer_id, seg_prod == data.product_id)
}

fn main() {
    println!("=== Serial Key Generator (No Traits, No impl) ===");

    let mut data = make_serial_data(4, 8);

    prompt_inputs(&mut data);

    let plain = plain_serial(&data);
    println!("Plain serial: {}", plain);

    let mc = new_magic_crypt!("magickey", 256);
    let encrypted = mc.encrypt_str_to_base64(&plain);
    println!("Encrypted serial: {}", encrypted);

    let decrypted = mc
        .decrypt_base64_to_string(&encrypted)
        .expect("Failed to decrypt serial");
    println!("Decrypted serial: {}", decrypted);

    let (ok_user, ok_prod) = verify_decrypted(&data, &decrypted);
    println!("Verify UserID: {}", ok_user);
    println!("Verify ProductID: {}", ok_prod);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_plain_serial() {
        let mut d = make_serial_data(4, 8);
        d.customer_id = "1234".to_string();
        d.product_id = "qwerasdf".to_string();
        assert_eq!(plain_serial(&d), "1234qwerasdf");
    }

    #[test]
    fn test_verify_segments() {
        let mut d = make_serial_data(4, 8);
        d.customer_id = "5678".to_string();
        d.product_id = "asdfqwer".to_string();

        let plain = plain_serial(&d);
        let (u_ok, p_ok) = verify_decrypted(&d, &plain);
        assert!(u_ok && p_ok);
    }

    #[test]
    fn test_encrypt_decrypt_roundtrip() {
        let mc = new_magic_crypt!("testkey", 256);
        let plain = "1234qwerasdf";
        let enc = mc.encrypt_str_to_base64(plain);
        let dec = mc.decrypt_base64_to_string(&enc).unwrap();
        assert_eq!(dec, plain);
    }
}
