use crate::data::GenSerialData;
use lazy_static::lazy_static;
use magic_crypt::{MagicCrypt256, MagicCryptTrait, new_magic_crypt};

const CRYPT_KEY: &str = "magickey";

lazy_static! {
    static ref MAGIC_CRYPT_BASE: MagicCrypt256 = new_magic_crypt!(CRYPT_KEY, 256);
}

fn generate_serial(items: &mut Vec<Box<dyn GenSerialData>>) -> String {
    let mut data = String::new();
    for item in items.iter_mut() {
        data.push_str(&item.get_rawdata());
    }
    data
}

pub fn encrypt_serial(items: &mut Vec<Box<dyn GenSerialData>>) -> String {
    let plain_serial = generate_serial(items);
    MAGIC_CRYPT_BASE.encrypt_str_to_base64(&plain_serial)
}

fn decrypt_plain_serial(plain_serial: String) -> String {
    MAGIC_CRYPT_BASE
        .decrypt_base64_to_string(plain_serial)
        .unwrap_or_else(|e| {
            eprintln!("Decryption failed: {e:?}");
            panic!("Invalid encrypted serial data provided: {e:?}");
        })
}

#[derive(Debug)]
pub struct DecryptedSerialData {
    pub name: String,
    pub digit: String,
}

pub fn decrypt_serial(
    plain_serial: String,
    items: &mut Vec<Box<dyn GenSerialData>>,
) -> Vec<DecryptedSerialData> {
    let dec = decrypt_plain_serial(plain_serial);
    let mut result_vec: Vec<DecryptedSerialData> = vec![];

    let mut offset = 0;
    for item in items.iter_mut() {
        let len = item.get_length();
        let rawdata = &dec[offset..offset + len];
        let decrypt_data = DecryptedSerialData {
            name: item.get_name(),
            digit: rawdata.to_owned(),
        };
        result_vec.push(decrypt_data);
        offset += len;
    }

    result_vec
}
