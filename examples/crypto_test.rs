use magic_crypt::{new_magic_crypt, MagicCryptTrait};

fn main() {
    let mc = new_magic_crypt!("magickey", 256);
    let base64 = mc.encrypt_str_to_base64("Hello world!");
    println!("Encrypted: {}", base64);

    let decrypted_string = mc.decrypt_base64_to_string(&base64).unwrap();
    println!("Decrypted: {}", decrypted_string);
}